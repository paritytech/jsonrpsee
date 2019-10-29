// Copyright 2019 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

extern crate proc_macro;

use inflector::Inflector as _;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned as _;

mod api_def;

/// Wraps around one or more API definitions and generates an enum.
///
/// The format within this macro must be:
///
/// ```ignore
/// rpc_api!{
///     Foo { ... }
///     pub(crate) Bar { ... }
/// }
/// ```
///
/// The `Foo` and `Bar` are identifiers, optionally prefixed with a visibility modifier
/// (e.g. `pub`).
///
/// The content of the blocks is the same as the content of a trait definition, except that
/// default implementations for methods are forbidden.
///
/// For each identifier (such as `Foo` and `Bar` in the example above), this macro will generate
/// an enum where each variant corresponds to a function of the definition. Function names are
/// turned into PascalCase to conform to the Rust style guide.
///
/// Each generated enum has a `next_request` method whose signature is:
///
/// ```ignore
/// async fn next_request(server: &'a mut jsonrpsee::core::Server<R, I>) -> Result<Foo<'a, R, I>, std::io::Error>;
/// ```
///
/// This method lets you grab the next request incoming from a server, and parse it to match of
/// the function definitions. Invalid requests are automatically handled.
///
/// Additionally, each generated enum has one method per function definition that lets you perform
/// the method has a client.
///
#[proc_macro]
pub fn rpc_api(input_token_stream: TokenStream) -> TokenStream {
    // Start by parsing the input into what we expect.
    let defs: api_def::ApiDefinitions = match syn::parse(input_token_stream) {
        Ok(d) => d,
        Err(err) => return err.to_compile_error().into(),
    };

    let mut out = Vec::with_capacity(defs.apis.len());
    for api in defs.apis {
        match build_api(api) {
            Ok(a) => out.push(a),
            Err(err) => return err.to_compile_error().into(),
        };
    }

    TokenStream::from(quote! {
        #(#out)*
    })
}

/// Generates the macro output token stream corresponding to a single API.
fn build_api(api: api_def::ApiDefinition) -> Result<proc_macro2::TokenStream, syn::Error> {
    let enum_name = &api.name;
    let visibility = &api.visibility;

    let mut variants = Vec::new();
    let mut tmp_variants = Vec::new();
    for function in &api.definitions {
        let function_is_notification = function.is_void_ret_type();
        let variant_name = snake_case_to_camel_case(&function.signature.ident);
        let ret = match &function.signature.output {
            syn::ReturnType::Default => quote!{()},
            syn::ReturnType::Type(_, ty) => quote_spanned!(ty.span()=> #ty),
        };

        let mut params_list = Vec::new();
        for input in function.signature.inputs.iter() {
            let (ty, pat_span, param_variant_name) = match input {
                syn::FnArg::Receiver(_) => {
                    return Err(syn::Error::new(input.span(), "Having `self` is not allowed in RPC queries definitions"));
                }
                syn::FnArg::Typed(syn::PatType { ty, pat, .. }) =>
                    (ty, pat.span(), param_variant_name(&pat)?),
            };

            params_list.push(quote_spanned!(pat_span=> #param_variant_name: #ty));
        }

        if !function_is_notification {
            if params_list.is_empty() {
                tmp_variants.push(quote_spanned!(function.signature.ident.span()=> #variant_name));
            } else {
                tmp_variants.push(quote_spanned!(function.signature.ident.span()=>
                    #variant_name {
                        #(#params_list,)*
                    }
                ));
            }
        }

        if function_is_notification {
            variants.push(quote_spanned!(function.signature.ident.span()=>
                #variant_name {
                    #(#params_list,)*
                }
            ));
        } else {
            variants.push(quote_spanned!(function.signature.ident.span()=>
                #variant_name {
                    respond: jsonrpsee::core::server::TypedResponder<'a, R, I, #ret>,
                    #(#params_list,)*
                }
            ));
        }
    }

    let next_request = {
        let mut notifications_blocks = Vec::new();
        let mut function_blocks = Vec::new();
        let mut tmp_to_rq = Vec::new();
        for function in &api.definitions {
            let function_is_notification = function.is_void_ret_type();
            let variant_name = snake_case_to_camel_case(&function.signature.ident);
            let rpc_method_name = function
                .attributes
                .method
                .clone()
                .unwrap_or_else(|| function.signature.ident.to_string());

            let mut params_builders = Vec::new();
            let mut params_names_list = Vec::new();

            for input in function.signature.inputs.iter() {
                let (ty, param_variant_name, rpc_param_name) = match input {
                    syn::FnArg::Receiver(_) => {
                        return Err(syn::Error::new(input.span(), "Having `self` is not allowed in RPC queries definitions"));
                    }
                    syn::FnArg::Typed(syn::PatType { ty, pat, attrs, .. }) =>
                        (ty, param_variant_name(&pat)?, rpc_param_name(&pat, &attrs)?),
                };

                params_names_list.push(quote_spanned!(function.signature.span()=> #param_variant_name));
                if !function_is_notification {
                    params_builders.push(quote_spanned!(function.signature.span()=>
                        let #param_variant_name: #ty = {
                            match request.params().get(#rpc_param_name) {
                                Ok(v) => v,
                                Err(_) => {
                                    // TODO: message
                                    request.respond(Err(jsonrpsee::core::common::Error::invalid_params(#rpc_param_name))).await;
                                    continue;
                                }
                            }
                        };
                    ));
                } else {
                    params_builders.push(quote_spanned!(function.signature.span()=>
                        let #param_variant_name: #ty = {
                            match request.params().get(#rpc_param_name) {
                                Ok(v) => v,
                                Err(_) => {
                                    // TODO: log this?
                                    continue;
                                }
                            }
                        };
                    ));
                }
            }

            if function_is_notification {
                notifications_blocks.push(quote_spanned!(function.signature.span()=>
                    if method == #rpc_method_name {
                        let request = n;
                        #(#params_builders)*
                        return Ok(#enum_name::#variant_name { #(#params_names_list),* });
                    }
                ));

            } else {
                function_blocks.push(quote_spanned!(function.signature.span()=>
                    if request_outcome.is_none() && method == #rpc_method_name {
                        let request = server.request_by_id(&request_id).unwrap();
                        #(#params_builders)*
                        request_outcome = Some(Tmp::#variant_name { #(#params_names_list),* });
                    }
                ));

                tmp_to_rq.push(quote_spanned!(function.signature.span()=>
                    Some(Tmp::#variant_name { #(#params_names_list),* }) => {
                        let request = server.request_by_id(&request_id).unwrap();
                        let respond = jsonrpsee::core::server::TypedResponder::from(request);
                        return Ok(#enum_name::#variant_name { respond #(, #params_names_list)* });
                    },
                ));
            }
        }

        let on_request = quote_spanned!(api.name.span()=> {
            #[allow(unused)]    // The enum might be empty
            enum Tmp {
                #(#tmp_variants,)*
            }

            let request_id = r.id();
            let method = r.method().to_owned();

            let mut request_outcome: Option<Tmp> = None;

            #(#function_blocks)*

            match request_outcome {
                #(#tmp_to_rq)*
                None => server.request_by_id(&request_id).unwrap().respond(Err(jsonrpsee::core::common::Error::method_not_found())).await,
            }
        });

        let on_notification = quote_spanned!(api.name.span()=> {
            let method = n.method().to_owned();
            #(#notifications_blocks)*
            // TODO: we received an unknown notification; log this?
        });

        quote_spanned!(api.name.span()=>
            #visibility async fn next_request(server: &'a mut jsonrpsee::core::Server<R, I>) -> Result<#enum_name<'a, R, I>, std::io::Error>
                where R: jsonrpsee::core::RawServer<RequestId = I>,
                        I: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync,
            {
                loop {
                    match server.next_event().await.unwrap() {        // TODO: don't unwrap
                        jsonrpsee::core::ServerEvent::Notification(n) => #on_notification,
                        jsonrpsee::core::ServerEvent::SubscriptionsClosed(_) => unimplemented!(),       // TODO:
                        jsonrpsee::core::ServerEvent::SubscriptionsReady(_) => unimplemented!(),       // TODO:
                        jsonrpsee::core::ServerEvent::Request(r) => #on_request,
                    }
                }
            }
        )
    };

    // Builds the functions that allow performing outbound JSON-RPC queries.
    let mut client_functions = Vec::new();
    for function in &api.definitions {
        let f_name = &function.signature.ident;
        let ret_ty = match function.signature.output {
            syn::ReturnType::Default => quote!(()),
            syn::ReturnType::Type(_, ref ty) => quote_spanned!(ty.span()=> #ty),
        };
        let rpc_method_name = function
            .attributes
            .method
            .clone()
            .unwrap_or_else(|| function.signature.ident.to_string());

        let mut params_list = Vec::new();
        let mut params_to_json = Vec::new();

        for (param_index, input) in function.signature.inputs.iter().enumerate() {
            let (ty, pat_span, rpc_param_name) = match input {
                syn::FnArg::Receiver(_) => {
                    return Err(syn::Error::new(input.span(), "Having `self` is not allowed in RPC queries definitions"));
                }
                syn::FnArg::Typed(syn::PatType { ty, pat, attrs, .. }) =>
                    (ty, pat.span(), rpc_param_name(&pat, &attrs)?),
            };

            let generated_param_name = syn::Ident::new(
                &format!("param{}", param_index),
                proc_macro2::Span::call_site(),
            );

            params_list.push(quote_spanned!(pat_span=> #generated_param_name: impl Into<#ty>));
            params_to_json.push(quote_spanned!(pat_span=>
                map.insert(
                    #rpc_param_name.to_string(),
                    jsonrpsee::core::common::to_value(#generated_param_name.into()).unwrap()        // TODO: don't unwrap
                );
            ));
        }

        let params_building = if params_list.is_empty() {
            quote!{jsonrpsee::core::common::Params::None}
        } else {
            let params_list_len = params_list.len();
            quote_spanned!(function.signature.span()=>
                jsonrpsee::core::common::Params::Map({
                    let mut map = jsonrpsee::core::common::JsonMap::with_capacity(#params_list_len);
                    #(#params_to_json)*
                    map
                })
            )
        };

        let is_notification = function.is_void_ret_type();
        let function_body = if is_notification {
            quote_spanned!(function.signature.span()=>
                client.send_notification(#rpc_method_name, #params_building).await
                    .map_err(jsonrpsee::core::client::ClientError::Inner)?;
                Ok(())
            )
            
        } else {
            quote_spanned!(function.signature.span()=>
                let rq_id = client.start_request(#rpc_method_name, #params_building).await
                    .map_err(jsonrpsee::core::client::ClientError::Inner)?;
                let data = client.request_by_id(rq_id).unwrap().await?;     // TODO: don't unwrap?
                Ok(jsonrpsee::core::common::from_value(data).unwrap())     // TODO: don't unwrap
            )
        };

        client_functions.push(quote_spanned!(function.signature.span()=>
            // TODO: what if there's a conflict between `client` and a param name?
            #visibility async fn #f_name<R: jsonrpsee::core::RawClient>(client: &mut jsonrpsee::core::Client<R> #(, #params_list)*)
                -> Result<#ret_ty, jsonrpsee::core::client::ClientError<<R as jsonrpsee::core::RawClient>::Error>> {
                #function_body
            }
        ));
    }

    // Builds the match variants for the implementation of `Debug`.
    let mut debug_variants = Vec::new();
    for function in &api.definitions {
        let variant_name = snake_case_to_camel_case(&function.signature.ident);
        debug_variants.push(quote_spanned!(function.signature.ident.span()=>
            #enum_name::#variant_name { /* TODO: params */ .. } => {
                f.debug_struct(stringify!(#enum_name))/* TODO: params */.finish()
            }
        ));
    }

    Ok(quote_spanned!(api.name.span()=>
        #visibility enum #enum_name<'a, R, I> {
            #(#variants),*
        }

        impl<'a, R, I> #enum_name<'a, R, I> {
            #next_request
        }

        impl<'a> #enum_name<'a, (), ()> {
            #(#client_functions)*
        }

        impl<'a, R, I> std::fmt::Debug for #enum_name<'a, R, I> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #(#debug_variants,)*
                }
            }
        }
    ))
}

/// Turns a snake case function name into an UpperCamelCase name suitable to be an enum variant.
fn snake_case_to_camel_case(snake_case: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&snake_case.to_string().to_pascal_case(), snake_case.span())
}

/// Determine the name of the variant in the enum based on the pattern of the function parameter.
fn param_variant_name(pat: &syn::Pat) -> syn::parse::Result<&syn::Ident> {
    match pat {
        // TODO: check other fields of the `PatIdent`
        syn::Pat::Ident(ident) => Ok(&ident.ident),
        _ => unimplemented!()
    }
}

/// Determine the name of the parameter based on the pattern.
fn rpc_param_name(pat: &syn::Pat, attrs: &[syn::Attribute]) -> syn::parse::Result<String> {
    // TODO: look in attributes if the user specified a param name
    match pat {
        // TODO: check other fields of the `PatIdent`
        syn::Pat::Ident(ident) => Ok(ident.ident.to_string()),
        _ => unimplemented!()
    }
}

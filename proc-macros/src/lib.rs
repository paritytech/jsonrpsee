extern crate proc_macro;

use inflector::Inflector as _;
use proc_macro::TokenStream;
use quote::quote;

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
    let defs: api_def::ApiDefinitions =
        syn::parse(input_token_stream).expect("failed to parse input");

    let out: Vec<_> = defs.apis.into_iter().map(build_api).collect();
    TokenStream::from(quote! {
        #(#out)*
    })
}

/// Generates the macro output token stream corresponding to a single API.
fn build_api(api: api_def::ApiDefinition) -> proc_macro2::TokenStream {
    let enum_name = &api.name;
    let visibility = &api.visibility;

    let mut variants = Vec::new();
    for function in &api.definitions {
        let variant_name = snake_case_to_camel_case(&function.ident);
        let ret = match &function.output {
            syn::ReturnType::Default => quote! {()},
            syn::ReturnType::Type(_, ty) => quote! {#ty},
        };

        variants.push(quote! {
            #variant_name {
                respond: jsonrpsee::core::server::TypedResponder<'a, R, I, #ret>,
            }
        });
    }

    let next_request = {
        let mut function_blocks = Vec::new();
        for function in &api.definitions {
            let variant_name = snake_case_to_camel_case(&function.ident);
            let rpc_method_name = function.ident.to_string();

            function_blocks.push(quote! {
                if method == #rpc_method_name {
                    let request = server.request_by_id(&request_id).unwrap();
                    /*$(
                        let $pn: $pty = {
                            let raw_val = match request.params().get(stringify!($pn)) {
                                Some(v) => v,
                                None => {
                                    request.respond(Err(jsonrpsee::core::common::Error::invalid_params("foo"))).await;       // TODO: message
                                    continue;
                                }
                            };

                            match jsonrpsee::core::common::from_value(raw_val.clone()) {
                                Ok(v) => v,
                                Err(_) => {
                                    request.respond(Err(jsonrpsee::core::common::Error::invalid_params("foo"))).await;       // TODO: message
                                    continue;
                                }
                            }
                        };
                    )**/

                    let respond = jsonrpsee::core::server::TypedResponder::from(request);
                    return Ok(#enum_name::#variant_name { respond });
                }
            });
        }

        quote! {
            #visibility async fn next_request(server: &'a mut jsonrpsee::core::Server<R, I>) -> Result<#enum_name<'a, R, I>, std::io::Error>
                where R: jsonrpsee::core::RawServer<RequestId = I>,
                        I: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync,
            {
                loop {
                    let (request_id, method) = match server.next_event().await.unwrap() {        // TODO: don't unwrap
                        jsonrpsee::core::ServerEvent::Notification(n) => unimplemented!(),       // TODO:
                        jsonrpsee::core::ServerEvent::Request(r) => (r.id(), r.method().to_owned()),
                    };

                    #(#function_blocks)*

                    server.request_by_id(&request_id).unwrap().respond(Err(jsonrpsee::core::common::Error::method_not_found())).await;
                }
            }
        }
    };

    // Builds the functions that allow performing outbound JSON-RPC queries.
    let mut client_functions = Vec::new();
    for function in &api.definitions {
        let f_name = &function.ident;
        let ret_ty = &function.output;
        let rpc_method_name = function.ident.to_string();

        let mut params_list = Vec::new();
        let mut params_to_json = Vec::new();

        for (param_index, input) in function.inputs.iter().enumerate() {
            let ty = match input {
                syn::FnArg::Receiver(_) => {
                    panic!("Having `self` is not allowed in RPC queries definitions")
                }
                syn::FnArg::Typed(syn::PatType { ty, .. }) => ty,
            };

            let generated_param_name = syn::Ident::new(
                &format!("param{}", param_index),
                proc_macro2::Span::call_site(),
            );

            params_list.push(quote! {#generated_param_name: #ty});
            params_to_json.push(quote! {
                let #generated_param_name = jsonrpsee::common::to_value(#generated_param_name).unwrap();        // TODO: don't unwrap
            });
        }

        client_functions.push(quote!{
            // TODO: what if there's a conflict in the param name?
            #visibility async fn #f_name(client: &mut jsonrpsee::core::Client<impl jsonrpsee::core::RawClient> #(, #params_list)*) #ret_ty {
                #(#params_to_json)*
                // TODO: pass params
                // TODO: don't unwrap
                client.request(#rpc_method_name, jsonrpsee::core::common::Params::None).await.unwrap()
            }
        });
    }

    // Builds the match variants for the implementation of `Debug`.
    let mut debug_variants = Vec::new();
    for function in &api.definitions {
        let variant_name = snake_case_to_camel_case(&function.ident);
        debug_variants.push(quote! {
            #enum_name::#variant_name { /* TODO: params */ .. } => {
                f.debug_struct(stringify!(#enum_name))/* TODO: params */.finish()
            }
        });
    }

    quote! {
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
    }
}

/// Turns a snake case function name into an UpperCamelCase name suitable to be an enum variant.
fn snake_case_to_camel_case(snake_case: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&snake_case.to_string().to_pascal_case(), snake_case.span())
}

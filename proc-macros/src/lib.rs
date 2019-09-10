extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

mod api_def;

#[proc_macro]
pub fn rpc_api(input_token_stream: TokenStream) -> TokenStream {
    // Start by parsing the input into what we expect.
    let defs: api_def::ApiDefinitions = syn::parse(input_token_stream)
        .expect("failed to parse input");

    let out: Vec<_> = defs.apis.into_iter().map(build_api).collect();
    TokenStream::from(quote!{
        #(#out)*
    })
}

fn build_api(api: api_def::ApiDefinition) -> proc_macro2::TokenStream {
    let enum_name = &api.name;

    let mut variants = Vec::new();
    for function in &api.definitions {
        let variant_name = &function.ident;
        let ret = match &function.output {
            syn::ReturnType::Default => quote!{()},
            syn::ReturnType::Type(_, ty) => quote!{#ty},
        };

        variants.push(quote!{
            #variant_name {
                respond: jsonrpsee::core::server::TypedResponder<'a, R, I, #ret>,
            }
        });
    }

    let next_request = {
        let mut function_blocks = Vec::new();
        for function in &api.definitions {
            let f_name = &function.ident;
            let ret_ty = &function.output;
            let rpc_method_name = function.ident.to_string();

            function_blocks.push(quote!{
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

                    let respond = jsonrpsee::core::server::TypedResponder {
                        rq: request,
                        response_ty: std::marker::PhantomData,
                    };
                    return Ok(#enum_name::#f_name { respond });
                }
            });
        }

        quote! {
            async fn next_request(server: &'a mut jsonrpsee::core::server::Server<R, I>) -> Result<#enum_name<'a, R, I>, std::io::Error>
                where R: jsonrpsee::core::server::raw::RawServer<RequestId = I>,
                        I: Clone + PartialEq + Eq + Send + Sync,
            {
                loop {
                    let (request_id, method) = match server.next_event().await.unwrap() {        // TODO: don't unwrap
                        jsonrpsee::core::server::ServerEvent::Notification(n) => unimplemented!(),       // TODO:
                        jsonrpsee::core::server::ServerEvent::Request(r) => (r.id(), r.method().to_owned()),
                    };

                    #(#function_blocks)*

                    server.request_by_id(&request_id).unwrap().respond(Err(jsonrpsee::core::common::Error::method_not_found())).await;
                }
            }
        }
    };

    let mut client_functions = Vec::new();
    for function in &api.definitions {
        let f_name = &function.ident;
        let ret_ty = &function.output;
        let rpc_method_name = function.ident.to_string();

        client_functions.push(quote!{
            async fn #f_name() #ret_ty {
                /*$(
                    let $pn = jsonrpsee::common::to_value($pn).unwrap();        // TODO: don't unwrap
                )**/

                let http = jsonrpsee::http_client("http://localhost:8000");
                http.request(#rpc_method_name).await.unwrap()
            }
        });
    }

    quote! {
        enum #enum_name<'a, R, I> {
            #(#variants),*
        }

        impl<'a, R, I> #enum_name<'a, R, I> {
            #next_request
        }

        // TODO: move inside of the impl block, but then you've got the question of how to handle
        // inferring generics
        #(#client_functions)*
    }
}

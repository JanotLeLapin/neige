use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, LitStr, Type, Token, parenthesized};
use syn::parse::{Parse, ParseStream, Result};

mod kw {
    syn::custom_keyword!(params);
    syn::custom_keyword!(body);
    syn::custom_keyword!(optional);
}

struct Endpoint {
    name: Ident,
    method: Ident,
    url: LitStr,
    params: Vec<(Ident, Option<Type>)>,
    body: Option<Type>,
    optional: bool,
    ret: Type,
}

struct Wrapper(Vec<Endpoint>);

impl Parse for Wrapper {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut res = Vec::new();
        loop {
            let name: Ident = input.parse()?;
            let method: Ident = input.parse()?;
            let url: LitStr = input.parse()?;

            let mut params = Vec::new();
            let mut body = None;
            let mut optional = false;

            while input.peek(Token![,]) {
                input.parse::<Token![,]>()?;

                if input.peek(kw::params) {
                    input.parse::<kw::params>()?;
                    input.parse::<Token![=]>()?;
                    let content;
                    parenthesized!(content in input);
                    loop {
                        let param_ident = content.parse::<Ident>()?;
                        let param_type = if content.peek(Token![:]) {
                            content.parse::<Token![:]>()?;
                            Some(content.parse::<Type>()?)
                        } else {
                            None
                        };

                        params.push((param_ident, param_type));

                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        } else {
                            break;
                        }
                    }
                }

                if input.peek(kw::body) {
                    input.parse::<kw::body>()?;
                    input.parse::<Token![=]>()?;
                    body = Some(input.parse()?);
                }

                if input.peek(kw::optional) {
                    input.parse::<kw::optional>()?;
                    optional = true;
                }
            }

            input.parse::<Token![=>]>()?;
            let ret: Type = input.parse()?;

            res.push(Endpoint {
                name,
                method,
                url,
                params,
                body,
                optional,
                ret,
            });

            if input.peek(Token![;]) {
                input.parse::<Token![;]>()?;
            } else {
                break;
            }
        }

        Ok(Wrapper(res))
    }
}

#[proc_macro]
pub fn wrapper(input: TokenStream) -> TokenStream {
    let tokens = syn::parse_macro_input!(input as Wrapper).0
        .into_iter()
        .map(|Endpoint { name, method, url, params, body, optional, ret }| {
            let (param_ident, param_type): (Vec<_>, Vec<_>) = params.into_iter().unzip();
            let (body_arg, body_value) = if let Some(body) = body {
                (
                    quote! { , #body },
                    quote! { Some(#body.to_string()) },
                )
            } else {
                (
                    quote! { },
                    quote! { None },
                )
            };
            let (return_type, return_expr) = if optional {
                (
                    quote! { Option<#ret> },
                    quote! { Some(body) },
                )
            } else {
                (
                    quote! { #ret },
                    quote! { body },
                )
            };
            quote! {
                pub async fn #name(tx: &mut Sender<RestRequest> #(,#param_ident: #param_type)* #body_arg) -> #return_type {
                    let body = send(tx, format!(#url #(,#param_ident)*), #body_value).await.unwrap();
                    #return_expr
                }
            }
        });

    let output = quote! {
        pub mod wrapper {
            use tokio::sync::mpsc::Sender;
            use crate::rest::{RestRequest, send};

            #(#tokens)*
        }
    };

    println!("{}", output);

    output.into()
}

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Block, ItemTrait, ReturnType, TraitItem, Visibility};

/// Note the methods on a trait for future use.
fn note_trait_methods(
    vis: &Visibility,
    ident_name: &str,
    trait_items: &[TraitItem],
) -> TokenStream2 {
    let note_ident = format_ident!("{}_METHODS", ident_name);
    let methods = trait_items
        .iter()
        .filter_map(|item| match &item {
            &TraitItem::Fn(method) => Some(method.sig.to_token_stream().to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();

    quote! {
        #[doc(hidden)]
        #vis const #note_ident: &[&'static str] = &[#(#methods),*];
    }
}

/// Rewrite the command trait differently for the frontend and backend.
#[proc_macro_attribute]
pub fn command_trait(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);

    let backend_command_trait_methods =
        note_trait_methods(&input.vis, "BACKEND_COMMANDS", &input.items);
    let frontend_command_trait_methods =
        note_trait_methods(&input.vis, "FRONTEND_COMMANDS", &input.items);

    let ident = &input.ident;
    let vis = &input.vis;
    let attrs = input.attrs;
    let backend_ident = format_ident!("Backend{}", ident.to_string());
    let frontend_ident = format_ident!("Frontend{}", ident.to_string());

    let (backend_items, frontend_items) = input.items.into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut backend_items, mut frontend_items), item| {
            let (backend_item, frontend_item) = match &item {
                TraitItem::Fn(method) => {
                    if method.sig.asyncness.is_some() {
                        let ty = match &method.sig.output {
                            ReturnType::Default => quote! { () },
                            ReturnType::Type(_, ty) => quote! { #ty },
                        };

                        let mut backend_method = method.clone();
                        let mut frontend_method = method.clone();

                        backend_method.sig.asyncness = None;
                        backend_method.sig.output = syn::parse::<ReturnType>(
                            quote! {
                                -> impl ::std::future::Future<Output = #ty> + ::std::marker::Send
                            }
                            .into(),
                        )
                        .unwrap();

                        if let Some(block) = backend_method.default.as_mut() {
                            *block = syn::parse::<Block>(
                                quote! {
                                    async {
                                        #block
                                    }
                                }
                                .into(),
                            )
                            .unwrap()
                        }

                        frontend_method.sig.asyncness = None;
                        frontend_method.sig.output = syn::parse::<ReturnType>(
                            quote! {
                                -> impl ::std::future::Future<Output = #ty>
                            }
                            .into(),
                        )
                        .unwrap();

                        if let Some(block) = frontend_method.default.as_mut() {
                            *block = syn::parse::<Block>(
                                quote! {
                                    async {
                                        #block
                                    }
                                }
                                .into(),
                            )
                            .unwrap()
                        }

                        (
                            TraitItem::Fn(backend_method),
                            TraitItem::Fn(frontend_method),
                        )
                    } else {
                        (item.clone(), item)
                    }
                }
                other => (other.clone(), other.clone()),
            };

            backend_items.push(backend_item);
            frontend_items.push(frontend_item);

            (backend_items, frontend_items)
        },
    );

    let backend_command_trait = quote! {
        #(#attrs)*
        #vis trait #backend_ident {
            #(#backend_items)*
        }
    };

    let frontend_command_trait = quote! {
        #(#attrs)*
        #vis trait #frontend_ident {
            #(#frontend_items)*
        }
    };

    quote! {
        #backend_command_trait

        #frontend_command_trait

        #backend_command_trait_methods

        #frontend_command_trait_methods
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        todo!()
    }
}

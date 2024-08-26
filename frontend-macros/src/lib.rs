use commands::FRONTEND_COMMANDS_METHODS;
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, FnArg, ItemStruct, Signature};

/// Implement application command methods for the frontend.
#[proc_macro_derive(FrontendCommands)]
pub fn derive_frontend_commands(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let ident = &input.ident;

    let mut methods = Vec::new();
    let mut method_arg_structs = Vec::new();

    for method_str in FRONTEND_COMMANDS_METHODS {
        let method_tokens = method_str.parse::<TokenStream>().unwrap();
        let method = parse_macro_input!(method_tokens as Signature);
        let method_name = &method.ident;
        let method_name_str = method_name.to_string();
        let struct_name = quote::format_ident!("__command__{}", method_name);
        let inputs = method
            .inputs
            .iter()
            .filter(|arg| match arg {
                FnArg::Receiver(_) => false,
                FnArg::Typed(_) => true,
            })
            .collect::<Punctuated<_, Comma>>();
        let input_names = inputs
            .iter()
            .filter_map(|arg| match arg {
                FnArg::Typed(pat) => Some(*pat.pat.clone()),
                _ => None,
            })
            .collect::<Punctuated<_, Comma>>();

        method_arg_structs.push(quote! {
            #[allow(non_camel_case_types)]
            #[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
            struct #struct_name {
                #inputs
            }
        });

        methods.push(quote! {
            #method {
                let args = #struct_name {
                    #input_names
                };
                let res = ::frontend_common::command_fetch(#method_name_str, &args).await.unwrap();
                res
            }
        });
    }

    quote! {
        #(#method_arg_structs)*

        impl ::commands::FrontendCommands for #ident {
            #(#methods)*
        }
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

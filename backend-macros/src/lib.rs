use commands::BACKEND_COMMANDS_METHODS;
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, FnArg, ItemImpl, Signature};

/// Wrap the backend's implementation of application commands in Tauri's command interface.
#[proc_macro_attribute]
pub fn backend_commands(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    let self_ty = &*input.self_ty;

    let mut method_matches = Vec::new();
    let mut method_arg_structs = Vec::new();

    for method_str in BACKEND_COMMANDS_METHODS {
        let method_tokens = method_str.parse::<TokenStream>().unwrap();
        let method = parse_macro_input!(method_tokens as Signature);
        let method_async = method.asyncness.is_some();
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

        let method_call = if method_async {
            quote! { self.#method_name(#input_names).await }
        } else {
            quote! { self.#method_name(#input_names) }
        };

        method_matches.push(quote! {
            #method_name_str => {
                let deserialized_req = ::serde_json::from_str(&req).map_err(|_| ::commands::CommandError::MalformedRequest(req))?;
                let #struct_name { #input_names } = deserialized_req;
                let res = #method_call;
                Ok(::serde_json::to_string(&res).unwrap())
            },
        });
    }

    quote! {
        #input

        #(#method_arg_structs)*

        impl #self_ty {
            /// The command function that parses all commands from the frontend.
            pub async fn command(&self, name: String, req: String) -> ::std::result::Result<::std::string::String, ::commands::CommandError> {
                match name.as_str() {
                    #(#method_matches)*
                    _ => Err(::commands::CommandError::InvalidCommand(name)),
                }
            }
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

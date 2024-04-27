use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn authenticate(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    // Extract the components of the function
    let sig = input.sig;
    let name = sig.clone().ident;
    let inputs = sig.clone().inputs;
    let output = sig.clone().output;
    let block = input.block;

    // Iterate over the function arguments and find the one named "ctx"
    let ctx_arg = inputs.iter().find_map(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(ident) = &*pat_type.pat {
                if ident.ident == "ctx" {
                    return Some(ident);
                }
            }
        }
        None
    });

    // Check if `ctx` argument exists
    let ctx = match ctx_arg {
        Some(ident) => ident,
        None => {
            return syn::Error::new_spanned(sig, "Function must have an argument named 'ctx'")
                .to_compile_error()
                .into();
        }
    };

    // Check if the function is async
    let is_async = sig.clone().asyncness;
    let expanded = quote! {
        #is_async fn #name(#inputs) #output {
            let query = ctx.data::<(String, String, String)>()?;
            let cred = ctx.data::<Credential>()?;
            if cred == &Credential::None {
                // check if path is available for unauthorized users
                return Err(RustyError::CredentialMissingError);
            }
            let db = #ctx.data::<DbClient>()?;
            match auth::authenticate(db, cred).await? {
                Some(user) => {
                    log::info!("successfully authenticated user `{}`", cred);
                },
                None => {
                    log::error!("failed to authenticate user `{}`", cred);
                    return Err(RustyError::UnauthenticatedError)
                },
            };
            #block
        }
    };

    TokenStream::from(expanded)
}

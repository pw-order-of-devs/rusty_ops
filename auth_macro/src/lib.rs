use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, PatIdent};

#[proc_macro_attribute]
#[cfg(not(tarpaulin_include))]
pub fn authenticate_basic(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    // Extract the context parameter
    let ctx = extract_ctx(&input);

    // expand function
    expand_fn("basic", &input, &ctx)
}

#[proc_macro_attribute]
#[cfg(not(tarpaulin_include))]
pub fn authenticate_bearer(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    // Extract the context parameter
    let ctx = extract_ctx(&input);

    // expand function
    expand_fn("bearer", &input, &ctx)
}

#[cfg(not(tarpaulin_include))]
fn extract_ctx(input: &ItemFn) -> PatIdent {
    // Extract the components of the function
    let sig = input.clone().sig;
    let inputs = sig.clone().inputs;

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
    match ctx_arg {
        Some(ident) => ident.clone(),
        None => panic!("Function must have an argument named 'ctx'"),
    }
}

#[cfg(not(tarpaulin_include))]
fn expand_fn(supported: &str, input: &ItemFn, ctx: &PatIdent) -> TokenStream {
    let sig = input.clone().sig;
    let name = sig.clone().ident;
    let inputs = sig.clone().inputs;
    let output = sig.clone().output;
    let block = input.clone().block;

    // Check if the function is async
    let is_async = sig.clone().asyncness;
    let expanded = quote! {
        #is_async fn #name(#inputs) #output {
            let query = ctx.data::<(String, String, String)>()?;
            let endpoint = format!("{}:{}:{}", query.0, query.1, query.2);
            let cred = ctx.data::<Credential>()?;
            if !get_public_gql_endpoints().contains(&endpoint) {
                let supported = #supported.split(',').collect::<Vec<&str>>();
                let cred_type = match cred {
                    Credential::Basic(_, _) => "basic",
                    Credential::Bearer(_) => "bearer",
                    Credential::None => {
                        log::error!("missing credential for endpoint `{}`", endpoint);
                        return Err(RustyError::CredentialMissingError);
                    },
                };
                if !supported.contains(&cred_type) {
                    log::error!("mismatching credential type for endpoint `{}`", endpoint);
                    return Err(RustyError::WrongCredentialTypeError);
                }
                let db = #ctx.data::<DbClient>()?;
                match auth::authenticate(db, cred).await {
                    Ok(user) => {
                        log::info!("authenticated user `{}` for endpoint `{}`: success", cred, endpoint);
                    },
                    Err(err) => {
                        log::error!("authenticated user `{}` for endpoint `{}`: error", cred, endpoint);
                        return Err(err)
                    },
                };
            }
            #block
        }
    };
    expanded.into()
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, PatIdent};

#[proc_macro_attribute]
pub fn authenticate(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let auth_type = extract_auth_type(&args);
    // Extract the context parameter
    let ctx = extract_ctx(&input);

    // expand function
    expand_fn(&auth_type, &input, &ctx)
}

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

fn expand_fn(auth_type: &str, input: &ItemFn, ctx: &PatIdent) -> TokenStream {
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
                let auth_type = #auth_type.split(',').collect::<Vec<&str>>();
                let cred_type = match cred {
                    Credential::Basic(_, _) => "basic",
                    Credential::Bearer(_) => "bearer",
                    Credential::None | Credential::System => {
                        log::error!("missing credential for endpoint `{endpoint}`");
                        return Err(RustyError::CredentialMissingError);
                    },
                };
                if !auth_type.contains(&cred_type) {
                    log::error!("mismatching credential type for endpoint `{endpoint}`");
                    return Err(RustyError::WrongCredentialTypeError);
                }
                let db = #ctx.data::<DbClient>()?;
                match auth::authenticate(db, cred).await {
                    Ok(username) => log::debug!("authenticated user `{cred}` for endpoint `{endpoint}`: success"),
                    Err(err) => {
                        log::error!("authenticated user `{cred}` for endpoint `{endpoint}`: {}", err.to_string());
                        return Err(err)
                    },
                };
            }
            #block
        }
    };
    expanded.into()
}

fn extract_auth_type(args: &TokenStream) -> String {
    match args.clone().into_iter().next() {
        Some(item) => item.to_string(),
        None => String::new(),
    }
}

use commons::env::var_or_default;
use serde_json::{json, Value};
use serde_valid::Validate;

use commons::errors::RustyError;
use domain::auth::credentials::{get_token_claim_str, Credential};
use domain::auth::permissions::Permission;
use domain::commons::search::SearchOptions;
use domain::RustyDomainItem;
use persist::db_client::DbClient;

pub async fn get_all<T: RustyDomainItem>(
    db: &DbClient,
    index: &str,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<Vec<T>, RustyError> {
    db.get_all(index, filter, options).await.map_err(|err| {
        log::error!("`{index}::get`: {err}");
        err
    })
}

pub async fn get_by_id<T: RustyDomainItem>(
    db: &DbClient,
    index: &str,
    id: &str,
) -> Result<Option<T>, RustyError> {
    db.get_one(index, json!({ "id": { "equals": id } }))
        .await
        .map_err(|err| {
            log::error!("`{index}::get`: {err}");
            err
        })
}

pub async fn get_one<T: RustyDomainItem>(
    db: &DbClient,
    index: &str,
    filter: &Value,
) -> Result<Option<T>, RustyError> {
    db.get_one(index, filter.clone()).await.map_err(|err| {
        log::error!("`{index}::get`: {err}");
        err
    })
}

pub async fn create<S, T, F>(
    db: &DbClient,
    index: &str,
    item: T,
    parse: F,
) -> Result<String, RustyError>
where
    S: RustyDomainItem,
    T: Clone + Validate + Send,
    F: FnOnce(T) -> S + Send,
{
    item.validate().map_err(|err| {
        log::error!("`{index}::create`: {err}");
        err
    })?;

    let id = db
        .create(index, &parse(item.clone()))
        .await
        .map_err(|err| {
            log::error!("`{index}::create`: {err}");
            err
        })?;
    Ok(id)
}

pub async fn delete_by_id<T: RustyDomainItem>(
    db: &DbClient,
    index: &str,
    id: &str,
) -> Result<u64, RustyError> {
    db.delete_one::<T>(index, json!({ "id": id }))
        .await
        .map_err(|err| {
            log::error!("`{index}::deleteById`: {err}");
            err
        })
}

pub async fn delete_all(db: &DbClient, index: &str) -> Result<u64, RustyError> {
    if !var_or_default("RUSTY_DEBUG", false) {
        log::warn!("`delete_all` is only supported in DEBUG mode");
        return Ok(0);
    }

    db.delete_all(index).await.map_err(|err| {
        log::error!("`{index}::deleteAll`: {err}");
        err
    })
}

pub async fn has_permission(
    db_client: &DbClient,
    cred: &Credential,
    id: &str,
    perm: (&str, &str),
) -> Result<(), RustyError> {
    match cred {
        Credential::Bearer(token) => {
            let username = get_token_claim_str(token, "sub");
            let permissions = check_user_permission(db_client, &username, perm).await?;
            if permissions.iter().any(|p| p.item == "ALL") {
                return Ok(());
            }

            if parse_permission_ids(&permissions).contains(&id.to_string()) {
                Ok(())
            } else {
                Err(RustyError::UnauthorizedError)
            }
        }
        Credential::System => Ok(()),
        _ => Err(RustyError::UnauthorizedError),
    }
}

async fn check_user_permission(
    db_client: &DbClient,
    username: &str,
    perm: (&str, &str),
) -> Result<Vec<Permission>, RustyError> {
    let entries = auth::get_user_permissions(db_client, username)
        .await?
        .iter()
        .filter(|p| p.resource == perm.0 && p.right == perm.1)
        .cloned()
        .collect();
    Ok(entries)
}

fn parse_permission_ids(permissions: &[Permission]) -> Vec<String> {
    permissions
        .iter()
        .filter(|p| p.item.starts_with("ID["))
        .map(|p| p.item.replace("ID[", "").replace(']', ""))
        .collect()
}

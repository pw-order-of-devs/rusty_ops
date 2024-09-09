use commons::env::var_or_default;
use serde_json::{json, Value};
use serde_valid::Validate;

use commons::errors::RustyError;
use domain::auth::credentials::{get_token_claim_str, Credential};
use domain::commons::search::SearchOptions;
use domain::RustyDomainItem;
use persist::db_client::DbClient;

pub async fn get_all<T: RustyDomainItem>(
    db: &DbClient,
    index: &str,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
) -> Result<Vec<T>, RustyError> {
    let values = db
        .get_all(index, filter, options)
        .await?
        .into_iter()
        .filter_map(|v| match serde_json::from_value(v) {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect();
    Ok(values)
}

pub async fn get_by_id<T: RustyDomainItem>(
    db: &DbClient,
    index: &str,
    id: &str,
) -> Result<Option<T>, RustyError> {
    let value = db
        .get_one(index, json!({ "id": { "equals": id } }))
        .await?
        .unwrap_or(Value::Null);
    Ok(serde_json::from_value(value)?)
}

pub async fn get_one<T: RustyDomainItem>(
    db: &DbClient,
    index: &str,
    filter: &Value,
) -> Result<Option<T>, RustyError> {
    let value = db
        .get_one(index, filter.clone())
        .await?
        .unwrap_or(Value::Null);
    Ok(serde_json::from_value(value)?)
}

pub async fn create(
    db: &DbClient,
    index: &str,
    base: impl Validate,
    item: impl RustyDomainItem,
) -> Result<String, RustyError> {
    base.validate().map_err(|err| {
        log::error!("`{index}::create`: {err}");
        err
    })?;

    let id = db.create(index, &item.to_value()?).await.map_err(|err| {
        log::error!("`{index}::create`: {err}");
        err
    })?;
    Ok(id)
}

pub async fn create_parse<S, T, F>(
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
        .create(index, &parse(item.clone()).to_value()?)
        .await
        .map_err(|err| {
            log::error!("`{index}::create`: {err}");
            err
        })?;
    Ok(id)
}

pub async fn delete_by_id(db: &DbClient, index: &str, id: &str) -> Result<u64, RustyError> {
    db.delete_one(index, json!({ "id": id }))
        .await
        .map_err(|err| {
            log::error!("`{index}::deleteById`: {err}");
            err
        })
}

pub async fn delete_many(db: &DbClient, index: &str, filter: &Value) -> Result<u64, RustyError> {
    let entries = db.get_all(index, &Some(filter.clone()), &None).await?;
    for entry in entries.iter() {
        if let Some(id) = entry
            .as_object()
            .and_then(|e| e.get("id"))
            .and_then(|e| e.as_str())
        {
            delete_by_id(db, index, id).await?;
        }
    }
    Ok(entries.len() as u64)
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

pub fn get_username_claim(cred: &Credential) -> Result<String, RustyError> {
    match cred {
        Credential::Bearer(token) => Ok(get_token_claim_str(token, "sub")),
        Credential::System => Ok("SYSTEM".to_string()),
        _ => Err(RustyError::UnauthorizedError),
    }
}

pub async fn check_project_write_permission(
    db: &DbClient,
    cred: &Credential,
    id: &str,
) -> Result<(), RustyError> {
    auth::authorize(
        db,
        &get_username_claim(cred)?,
        &format!("PROJECTS:WRITE:ID[{}]", id),
    )
    .await
}

pub fn remove_filter_field(filter: &mut Option<Value>, field_to_remove: &str) -> Option<Value> {
    if let Some(ref mut value) = filter {
        if let Some(obj) = value.as_object_mut() {
            obj.remove(field_to_remove)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn add_filter_field(filter: &mut Option<Value>, key: &str, value: Value) -> Value {
    if let Some(ref mut obj) = filter {
        if let Some(map) = obj.as_object_mut() {
            map.insert(key.to_string(), value);
            obj.clone()
        } else {
            json!({ key: value })
        }
    } else {
        json!({ key: value })
    }
}

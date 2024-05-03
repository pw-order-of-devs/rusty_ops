use serde_json::{json, Value};
use serde_valid::Validate;

use commons::errors::RustyError;
use domain::commons::search::SearchOptions;
use domain::RustyDomainItem;
use persist::db_client::DbClient;

pub fn to_paged(options: &Option<SearchOptions>) -> Result<(usize, usize), RustyError> {
    let options = options.clone().unwrap_or_default();
    let page = options.page_number.unwrap_or_default();
    let page = if page == 0 { 1 } else { page };
    let page = usize::try_from(page)?;
    let page_size = options.page_size.unwrap_or_default();
    let page_size = if page_size == 0 { 20 } else { page_size };
    let page_size = usize::try_from(page_size)?;
    Ok((page, page_size))
}

pub async fn get_total_count<T: RustyDomainItem>(
    db: &DbClient,
    index: &str,
    filter: &Option<Value>,
) -> Result<usize, RustyError> {
    let entries = db
        .get_all::<T>(index, filter, &None, false)
        .await
        .map_err(|err| {
            log::error!("here");
            log::error!("`{index}::get`: {err}");
            err
        })?;
    Ok(entries.len())
}

pub async fn get_all<T: RustyDomainItem>(
    db: &DbClient,
    index: &str,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
    paged: bool,
) -> Result<Vec<T>, RustyError> {
    db.get_all(index, filter, options, paged)
        .await
        .map_err(|err| {
            log::error!("`{index}::get`: {err}");
            err
        })
}

pub async fn get_by_id<T: RustyDomainItem>(
    db: &DbClient,
    index: &str,
    id: &str,
) -> Result<Option<T>, RustyError> {
    db.get_one(index, json!({ "id": id })).await.map_err(|err| {
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
    db.delete_all(index).await.map_err(|err| {
        log::error!("`{index}::deleteAll`: {err}");
        err
    })
}

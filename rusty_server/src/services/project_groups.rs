use async_graphql::SelectionField;
use serde_json::{json, Value};

use commons::errors::RustyError;
use domain::auth::credentials::Credential;
use domain::commons::search::SearchOptions;
use domain::projects::{Group, GroupModel, ProjectModel, RegisterGroup};
use persist::db_client::DbClient;

use crate::services::shared::{add_filter_field, get_username_claim, remove_filter_field};
use crate::services::{projects, shared};

const GROUPS_INDEX: &str = "project_groups";

// query

pub async fn get_all(
    db: &DbClient,
    cred: &Credential,
    filter: &Option<Value>,
    options: &Option<SearchOptions>,
    inner: &[SelectionField<'_>],
) -> Result<Vec<GroupModel>, RustyError> {
    let mut filter = filter.clone();
    let mut inner_filter = remove_filter_field(&mut filter, "projects");

    let entries = shared::get_all::<Group>(db, GROUPS_INDEX, &filter, options).await?;
    let mut filtered = vec![];
    let username = get_username_claim(cred)?;
    for entry in entries {
        if auth::authorize(
            db,
            &username,
            &format!("PROJECT_GROUPS:READ:ID[{}]", entry.id),
        )
        .await
        .is_ok()
        {
            filtered.push(GroupModel::from(&entry));
        }
    }

    if inner.iter().map(|f| f.name()).any(|f| f == "projects") {
        for f in &mut filtered {
            let filter = add_filter_field(&mut inner_filter, "group_id", json!({ "equals": f.id }));
            if let Ok(projects) = get_projects_for_group(db, cred, &filter, inner).await {
                f.projects = projects;
            }
        }
    }
    Ok(filtered)
}

pub async fn get_by_id(
    db: &DbClient,
    cred: &Credential,
    id: &str,
    filter: &Option<Value>,
    inner: &[SelectionField<'_>],
) -> Result<Option<GroupModel>, RustyError> {
    let mut filter = filter.clone();
    let mut inner_filter = remove_filter_field(&mut filter, "projects");

    let username = get_username_claim(cred)?;
    auth::authorize(db, &username, &format!("PROJECT_GROUPS:READ:ID[{id}]")).await?;
    if let Some(group) = shared::get_by_id::<Group>(db, GROUPS_INDEX, id).await? {
        let mut model = GroupModel::from(&group);
        if inner.iter().map(|f| f.name()).any(|f| f == "projects") {
            let filter =
                add_filter_field(&mut inner_filter, "group_id", json!({ "equals": model.id }));
            if let Ok(projects) = get_projects_for_group(db, cred, &filter, inner).await {
                model.projects = projects;
            }
        }
        Ok(Some(model))
    } else {
        Ok(None)
    }
}

async fn get_projects_for_group(
    db: &DbClient,
    cred: &Credential,
    filter: &Value,
    inner: &[SelectionField<'_>],
) -> Result<Vec<ProjectModel>, RustyError> {
    let projects_inner = if let Some(field) = inner.iter().find(|f| f.name() == "projects") {
        field.selection_set().collect()
    } else {
        vec![]
    };

    projects::get_all(db, cred, &Some(filter.clone()), &None, &projects_inner).await
}

// mutate

pub async fn create(
    db: &DbClient,
    cred: &Credential,
    group: RegisterGroup,
) -> Result<String, RustyError> {
    let username = get_username_claim(cred)?;
    auth::authorize(db, &username, "PROJECT_GROUPS:CREATE").await?;
    shared::create(db, GROUPS_INDEX, group, |r| Group::from(&r)).await
}

pub async fn delete_by_id(db: &DbClient, cred: &Credential, id: &str) -> Result<u64, RustyError> {
    let username = get_username_claim(cred)?;
    auth::authorize(db, &username, &format!("PROJECT_GROUPS:WRITE:ID[{id}]")).await?;
    shared::delete_by_id(db, GROUPS_INDEX, id).await
}

pub async fn delete_all(db: &DbClient) -> Result<u64, RustyError> {
    shared::delete_all(db, GROUPS_INDEX).await
}

use commons::errors::RustyError;
use persist::db_client::DbClient;

pub(crate) async fn authorize(
    db: &DbClient,
    username: &str,
    resource: &str,
) -> Result<(), RustyError> {
    if username == "SYSTEM" {
        log::debug!("authenticated SYSTEM user");
        return Ok(());
    }

    let permissions = crate::get_user_permissions(db, username).await?;
    let split = resource.split(':').collect::<Vec<&str>>();

    match split.len() {
        2 => {
            if permissions
                .into_iter()
                .map(|res| format!("{}:{}", res.resource, res.right))
                .any(|x| x == *resource)
            {
                Ok(())
            } else {
                Err(RustyError::UnauthorizedError)
            }
        }
        3 => {
            let base_resource = format!("{}:{}:ALL", split[0], split[1]);
            let permissions = permissions
                .into_iter()
                .map(|res| format!("{}:{}:{}", res.resource, res.right, res.item))
                .collect::<Vec<String>>();
            if permissions.contains(&base_resource) || permissions.iter().any(|x| x == resource) {
                Ok(())
            } else {
                Err(RustyError::UnauthorizedError)
            }
        }
        _ => Err(RustyError::UnauthorizedError),
    }
}

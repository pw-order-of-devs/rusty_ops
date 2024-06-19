use commons::errors::RustyError;
use persist::db_client::DbClient;

pub(crate) async fn authorize(
    db: &DbClient,
    username: &str,
    resource: &str,
) -> Result<(), RustyError> {
    let permissions = crate::get_user_permissions(db, username).await?;
    if permissions
        .into_iter()
        .map(|res| format!("{}:{}", res.resource, res.right))
        .any(|x| x == *resource)
    {
        Ok(())
    } else {
        Err(RustyError::UnauthenticatedError)
    }
}

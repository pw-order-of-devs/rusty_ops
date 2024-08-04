use commons::errors::RustyError;
use domain::auth::permissions::Permission;
use domain::auth::roles::Role;
use domain::RustyDomainItem;
use persist::db_client::DbClient;

use crate::utils::USER_ID;

pub const PERMISSIONS_INDEX: &str = "permissions";
pub const PERMISSION: &str = "RESOURCE:RIGHT";
pub const PERMISSION_ALL: &str = "RESOURCE:RIGHT:ALL";
pub const PERMISSION_ERR: &str = "INVALID";
pub const ROLE_ID: &str = "86ee6a82-cbec-4008-837f-d777ead0477b";
pub const ROLES_INDEX: &str = "roles";

pub(crate) async fn create_role(db_client: &DbClient) -> Result<String, RustyError> {
    db_client
        .create(
            ROLES_INDEX,
            &Role {
                id: ROLE_ID.to_string(),
                name: "role".to_string(),
                description: None,
                users: vec![USER_ID.to_string()],
            }
            .to_value()
            .unwrap(),
        )
        .await
}

pub(crate) async fn create_permission_user(db_client: &DbClient) -> Result<String, RustyError> {
    db_client
        .create(
            PERMISSIONS_INDEX,
            &Permission::new(Some(USER_ID.to_string()), None, "RESOURCE", "RIGHT", "ALL")
                .to_value()
                .unwrap(),
        )
        .await
}

pub(crate) async fn create_permission_role(db_client: &DbClient) -> Result<String, RustyError> {
    db_client
        .create(
            PERMISSIONS_INDEX,
            &Permission::new(None, Some(ROLE_ID.to_string()), "RESOURCE", "RIGHT", "ALL")
                .to_value()
                .unwrap(),
        )
        .await
}

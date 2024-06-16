use rstest::rstest;
use serde_valid::Validate;

use domain::auth::permissions::Permission;

#[rstest]
#[case(Permission::new(None, None, "sample", "sample", "ALL"), true)]
#[case(
    Permission::new(
        None,
        None,
        "sample",
        "sample",
        "ID[1569e702-d022-4962-b837-96e0b03d8823]"
    ),
    true
)]
#[case(
    Permission::new(
        None,
        None,
        "sample",
        "sample",
        "ID[1569e702-d022-2962-b837-96e0b03d8823]"
    ),
    false
)]
#[case(Permission::new(None, None, "sample", "sample", "ID[smth]"), false)]
#[case(
    Permission::new(
        None, None, "sample", "sample", "smth\
"
    ),
    false
)]
fn validate_permission_test(#[case] permission: Permission, #[case] expected: bool) {
    assert_eq!(expected, permission.validate().is_ok())
}

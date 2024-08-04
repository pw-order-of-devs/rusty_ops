use rstest::rstest;
use serde_valid::Validate;

use domain::jobs::{Job, RegisterJob};

#[test]
fn from_register_job_test() {
    let project_id = uuid::Uuid::new_v4().to_string();
    let name = "test_01";
    let description = "test_desc_01";
    let template = r#"
    stages:
       test:
          script:
            - echo "hello"
    "#;
    let input = RegisterJob::new(name, description, template, &project_id);
    let job = Job::from(&input);
    assert_eq!(36, job.id.len());
    assert_eq!(name.to_string(), job.name);
    assert!(job.description.is_some());
    assert_eq!(description.to_string(), job.description.unwrap());
    assert_eq!(template.to_string(), job.template);
    assert_eq!(project_id, job.project_id);
}

const PROJECT_ID: &str = "871188c7-6a26-41a0-b7a2-1cb97dcdb01a";
const TEMPLATE_MINIMAL: &str =
    "c3RhZ2VzOgogICB0ZXN0OgogICAgICBzY3JpcHQ6CiAgICAgICAgLSBlY2hvICJoZWxsbyI";

#[rstest]
#[case(RegisterJob::new("new job", "", TEMPLATE_MINIMAL, PROJECT_ID), true)]
#[case(RegisterJob::new("new job", "", "dfghfhfghf", PROJECT_ID), false)]
#[case(RegisterJob::new("", "", "", PROJECT_ID), false)]
#[case(RegisterJob::new("new", "", "", ""), false)]
#[case(RegisterJob::new("new job", "", "", PROJECT_ID), false)]
fn validate_user_test(#[case] job: RegisterJob, #[case] expected: bool) {
    assert_eq!(expected, job.validate().is_ok())
}

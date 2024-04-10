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

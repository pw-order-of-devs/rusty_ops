use commons::errors::RustyError;
use domain::templates::pipeline::PipelineTemplate;

#[test]
fn validate_from_yaml_minimal_test() {
    let yaml = r#"
    stages:
       test:
          script:
            - echo "hello"
    "#;

    let encoded = base64_url::encode(&yaml);
    let pipeline = PipelineTemplate::from_yaml(&encoded);
    assert!(pipeline.is_ok());
    let pipeline = pipeline.unwrap();
    assert_eq!(1, pipeline.stages.len());
    assert_eq!("test", pipeline.stages.keys()[0]);
    assert_eq!(1, pipeline.stages[0].script.len());
}

#[test]
fn validate_from_yaml_full_test() {
    let yaml = r#"
    image: alpine

    env:
      test_key: value

    before:
      script:
        - echo "hello"

    after:
      script:
        - echo "good bye"

    stages:
       test_1:
          image: rust:alpine
          env:
            test_key_stage_1: value
          script:
            - echo "hello"
       test_2:
          image: rust:alpine
          env:
            test_key_stage_2: value
          script:
            - echo "hello"
    "#;

    let encoded = base64_url::encode(&yaml);
    let pipeline = PipelineTemplate::from_yaml(&encoded);
    assert!(pipeline.is_ok());
    let pipeline = pipeline.unwrap();
    assert_eq!(2, pipeline.stages.len());
    assert_eq!(
        "value",
        pipeline.stages[0].clone().env.unwrap()["test_key_stage_1"]
    );
}

#[test]
fn validate_from_yaml_error_empty_stages_test() {
    let yaml = r#"
    stages:
    "#;

    let encoded = base64_url::encode(&yaml);
    let pipeline = PipelineTemplate::from_yaml(&encoded);
    assert!(pipeline.is_err());
    assert_eq!(
        RustyError::SerializationError("Pipeline template: stages cannot be empty".to_string()),
        pipeline.unwrap_err()
    );
}

#[test]
fn validate_from_yaml_error_empty_stage_scripts_test() {
    let yaml = r#"
    stages:
      test:
        script:
    "#;

    let encoded = base64_url::encode(&yaml);
    let pipeline = PipelineTemplate::from_yaml(&encoded);
    assert!(pipeline.is_err());
    assert_eq!(
        RustyError::SerializationError(
            "Pipeline template: stages.script cannot be empty".to_string()
        ),
        pipeline.unwrap_err()
    );
}

#[test]
fn validate_from_yaml_error_empty_before_scripts_test() {
    let yaml = r#"
    before:
      script:

    stages:
      test:
        script:
          - echo "hello"
    "#;

    let encoded = base64_url::encode(&yaml);
    let pipeline = PipelineTemplate::from_yaml(&encoded);
    assert!(pipeline.is_err());
    assert_eq!(
        RustyError::SerializationError(
            "Pipeline template: before.script cannot be empty".to_string()
        ),
        pipeline.unwrap_err()
    );
}

#[test]
fn validate_from_yaml_error_empty_after_scripts_test() {
    let yaml = r#"
    after:
      script:

    stages:
      test:
        script:
          - echo "hello"
    "#;

    let encoded = base64_url::encode(&yaml);
    let pipeline = PipelineTemplate::from_yaml(&encoded);
    assert!(pipeline.is_err());
    assert_eq!(
        RustyError::SerializationError(
            "Pipeline template: after.script cannot be empty".to_string()
        ),
        pipeline.unwrap_err()
    );
}

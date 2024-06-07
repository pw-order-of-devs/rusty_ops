use rstest::rstest;

use rusty_agent::resolver::assignment;

const PAYLOAD_OK: &str = r#"{"payload": {"data": {"pipelines": {
    "id": "dummy",
    "number": 0,
    "registerDate": "now",
    "status": "DEFINED",
    "jobId": "dummy"
}}}}"#;

#[rstest]
#[case(PAYLOAD_OK, true)]
#[case(r#"{"payload": {"data": "None"}}"#, true)]
#[case(r#"{"payload": {"data": {"pipelines": { "smth": "err" }}}}"#, true)]
#[case(r#"something"#, false)]
#[tokio::test]
async fn assign_pipeline_test(#[case] payload: &str, #[case] expected: bool) {
    let result = assignment::assign_pipeline("uuid", payload).await;
    assert_eq!(expected, result.is_ok());
}

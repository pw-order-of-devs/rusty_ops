use domain::commons::search::SearchFilter;
use serde_json::Value;

#[test]
fn search_filter_default_test() {
    match SearchFilter::default() {
        SearchFilter::Equals(Value::Null) => (),
        _ => panic!("`SearchFilter::default` error"),
    }
}

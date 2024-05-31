use serde_json::Value;

use domain::commons::search::SearchFilter;

fn compare_strings(item: &Value, f: &Value, comparison: fn(&String, &String) -> bool) -> bool {
    if item.is_string() && f.is_string() {
        comparison(
            &item.as_str().unwrap().to_lowercase(),
            &f.as_str().unwrap().to_lowercase(),
        )
    } else {
        false
    }
}

fn compare_u64(item: &Value, f: &Value, comparison: fn(&u64, &u64) -> bool) -> bool {
    if item.is_u64() && f.is_u64() {
        comparison(&item.as_u64().unwrap(), &f.as_u64().unwrap())
    } else {
        false
    }
}

fn compare_array(item: &Value, f: &Value, comparison: fn(Vec<Value>, Value) -> bool) -> bool {
    if !f.is_array() {
        false
    } else if item.is_string() {
        let array = f
            .as_array()
            .unwrap()
            .iter()
            .map(|item| item.as_str().unwrap_or_default().to_lowercase())
            .map(Value::String)
            .collect::<Vec<Value>>();
        comparison(array, Value::String(item.as_str().unwrap().to_lowercase()))
    } else {
        comparison(f.as_array().unwrap().clone(), item.clone())
    }
}

pub(crate) fn filter_results(filter: &Option<Value>, values: &[Value]) -> Vec<Value> {
    filter.as_ref().map_or_else(
        || values.to_vec(),
        |filter| {
            values
                .iter()
                .filter(|item| {
                    filter.as_object().map_or(true, |filter| {
                        filter.keys().all(|key| {
                            filter.get(key).map_or(true, |filter| {
                                let item = item.get(key).unwrap_or(&Value::Null);
                                match serde_json::from_value::<SearchFilter>(filter.clone())
                                    .unwrap_or_default()
                                {
                                    SearchFilter::Equals(f) => {
                                        compare_strings(item, &f, String::eq)
                                            || compare_u64(item, &f, u64::eq)
                                    }
                                    SearchFilter::NotEquals(f) => {
                                        compare_strings(item, &f, String::ne)
                                            || compare_u64(item, &f, u64::ne)
                                    }
                                    SearchFilter::StartsWith(f) => {
                                        compare_strings(item, &f, |a, b| a.starts_with(b))
                                    }
                                    SearchFilter::EndsWith(f) => {
                                        compare_strings(item, &f, |a, b| a.ends_with(b))
                                    }
                                    SearchFilter::Contains(f) => {
                                        compare_strings(item, &f, |a, b| a.contains(b))
                                    }
                                    SearchFilter::GreaterOrEqual(f) => {
                                        compare_strings(item, &f, String::ge)
                                            || compare_u64(item, &f, PartialOrd::ge)
                                    }
                                    SearchFilter::GreaterThan(f) => {
                                        compare_strings(item, &f, String::gt)
                                            || compare_u64(item, &f, PartialOrd::gt)
                                    }
                                    SearchFilter::LessOrEquals(f) => {
                                        compare_strings(item, &f, String::le)
                                            || compare_u64(item, &f, PartialOrd::le)
                                    }
                                    SearchFilter::LessThan(f) => {
                                        compare_strings(item, &f, String::lt)
                                            || compare_u64(item, &f, PartialOrd::lt)
                                    }
                                    SearchFilter::Before(f) => {
                                        compare_strings(item, &f, PartialOrd::lt)
                                    }
                                    SearchFilter::After(f) => {
                                        compare_strings(item, &f, PartialOrd::gt)
                                    }
                                    SearchFilter::NotBefore(f) => {
                                        compare_strings(item, &f, PartialOrd::ge)
                                    }
                                    SearchFilter::NotAfter(f) => {
                                        compare_strings(item, &f, PartialOrd::le)
                                    }
                                    SearchFilter::OneOf(f) => {
                                        compare_array(item, &f, |a, b| a.contains(&b))
                                    }
                                }
                            })
                        })
                    })
                })
                .cloned()
                .collect()
        },
    )
}

use std::cmp::Ordering;

use serde_json::{json, Value};

use commons::errors::RustyError;
use domain::commons::search::{SearchFilter, SearchOptions, SortOptions};

use crate::Persistence;

pub(crate) async fn get_one(
    p: &impl Persistence,
    index: &str,
    filter: Value,
) -> Result<Option<Value>, RustyError> {
    let values = p.get_all(index, &Some(filter), &None).await?;
    if values.len() == 1 {
        Ok(Some(values[0].clone()))
    } else {
        Ok(None)
    }
}

pub(crate) fn get_value_id(value: &Value) -> String {
    value
        .get("id")
        .unwrap_or(&Value::Null)
        .as_str()
        .unwrap_or_default()
        .to_string()
}

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
                                if filter == &Value::Null && item == &Value::Null {
                                    return true;
                                }

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

pub(crate) fn sort_results(options: &SearchOptions, filtered: &mut [Value]) {
    let sort_field = &options
        .clone()
        .sort_field
        .unwrap_or_else(|| "id".to_string());
    filtered.sort_by(
        |a, b| match (a[sort_field].clone(), b[sort_field].clone()) {
            (Value::String(a), Value::String(b)) => a.cmp(&b),
            (Value::Number(a), Value::Number(b)) => a
                .as_f64()
                .partial_cmp(&b.as_f64())
                .unwrap_or_else(|| panic!("Failed comparing by {sort_field}")),
            (Value::Bool(a), Value::Bool(b)) => a.cmp(&b),
            _ => Ordering::Equal,
        },
    );
    if options.sort_mode.unwrap_or_default() == SortOptions::Descending {
        filtered.reverse();
    }
}

pub(crate) fn delete_one_filter(filter: &Value) -> Value {
    if let Value::Object(map) = filter.clone() {
        let (first_key, first_value) = map
            .into_iter()
            .next()
            .unwrap_or((String::new(), Value::Null));
        json!({ first_key: { "equals": first_value } })
    } else {
        Value::Null
    }
}

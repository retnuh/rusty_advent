use serde_json::Value;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/2015/day12.json").unwrap();
    let parsed: Value = serde_json::from_str(&*input).unwrap();
    println!("Part 1: {}", sum_numbers(&parsed));
    println!("Part 2: {}", sum_numbers_excluding_red(&parsed));
}

fn sum_numbers(val: &Value) -> f64 {
    return match val {
        Value::Null | Value::Bool(_) | Value::String(_) => 0.0,
        Value::Number(x) => x.as_f64().unwrap(),
        Value::Array(vals) => vals.iter().map(sum_numbers).sum(),
        Value::Object(vals) => vals.values().map(sum_numbers).sum(),
    };
}

fn sum_numbers_excluding_red(val: &Value) -> f64 {
    return match val {
        Value::Null | Value::Bool(_) | Value::String(_) => 0.0,
        Value::Number(x) => x.as_f64().unwrap(),
        Value::Array(vals) => vals.iter().map(sum_numbers_excluding_red).sum(),
        Value::Object(vals) => {
            let red = vals.values().find(|&v| v.as_str() == Some("red"));
            match red {
                None => vals.values().map(sum_numbers_excluding_red).sum(),
                Some(_) => 0.0,
            }
        }
    };
}
#[test]
fn test_stuff() {
    assert_eq!(sum_numbers(&json!([[[3]]])), 3.0);
    assert_eq!(sum_numbers(&json!({"a":{"b":4},"c":-1})), 3.0);
    assert_eq!(
        sum_numbers_excluding_red(&json!([1,{"c":"red","b":2},3])),
        4.0
    );
}

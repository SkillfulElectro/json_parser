# json_parser
simple lib to convert json to hashmap in Rustlang

## install
https://crates.io/crates/json_parser_simple

## example :
```rust
use json_parser_simple;

fn main() {
    let json_str = r#"{
        "name": "John Doe",
        "age": 30,
        "is_student": false,
        "address": {
          "street": "123 Main St",
          "city": "Anytown",
          "zip": "12345"
        },
        "hobbies": ["reading", "swimming", "coding"]
      }"#
      ;
    println!("{:#?}",json_parser_simple::json_scan(&String::from(json_str)));
}
```
json scan returns ->
```rust
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}
```

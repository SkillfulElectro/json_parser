use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

fn json_get_value(value: &str) -> JsonValue {
    let trimmed = value.trim();

    if trimmed == "null" {
        JsonValue::Null
    } else if trimmed == "true" {
        JsonValue::Bool(true)
    } else if trimmed == "false" {
        JsonValue::Bool(false)
    } else if let Ok(number) = trimmed.parse::<f64>() {
        JsonValue::Number(number)
    } else if trimmed.starts_with('\"') && trimmed.ends_with('\"') {
        JsonValue::String(trimmed[1..trimmed.len() - 1].to_string())
    } else if trimmed.starts_with('{') && trimmed.ends_with('}') {
        parse_object(&trimmed[1..trimmed.len() - 1])
    } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
        parse_array(&trimmed[1..trimmed.len() - 1])
    } else {
        JsonValue::String(trimmed.to_string())
    }
}


fn parse_object(value: &str) -> JsonValue {
    let mut object = HashMap::new();
    let mut key = String::new();
    let mut val = String::new();
    let mut in_key = true;
    let mut in_string = false;
    let mut depth = 0;
    let mut chars = value.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '{' | '[' => {
                depth += 1;
                if in_key {
                    key.push(ch);
                } else {
                    val.push(ch);
                }
                while let Some(inner_ch) = chars.next() {
                    match inner_ch {
                        '}' | ']' => {
                            depth -= 1;
                            if in_key {
                                key.push(inner_ch);
                            } else {
                                val.push(inner_ch);
                            }
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {
                            if in_key {
                                key.push(inner_ch);
                            } else {
                                val.push(inner_ch);
                            }
                        }
                    }
                }
            }
            ':' if depth == 0 && in_key => {
                in_key = false;
            }
            ',' if depth == 0 => {
                object.insert(key.trim().trim_matches('\"').to_string(), json_get_value(val.trim()));
                key = String::new();
                val = String::new();
                in_key = true;
            }
            _ => {
                if in_key {
                    key.push(ch);
                } else {
                    val.push(ch);
                }
            }
        }
    }
    if !key.is_empty() {
        object.insert(key.trim().trim_matches('\"').to_string(), json_get_value(val.trim()));
    }
    JsonValue::Object(object)
}

fn parse_array(value: &str) -> JsonValue {
    let mut elements = Vec::new();
    let mut element = String::new();
    let mut depth = 0;
    let mut chars = value.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '{' | '[' => {
                depth += 1;
                element.push(ch);
                while let Some(inner_ch) = chars.next() {
                    match inner_ch {
                        '}' | ']' => {
                            depth -= 1;
                            element.push(inner_ch);
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => element.push(inner_ch),
                    }
                }
            }
            ',' if depth == 0 => {
                elements.push(json_get_value(element.trim()));
                element = String::new();
            }
            _ => element.push(ch),
        }
    }
    if !element.is_empty() {
        elements.push(json_get_value(element.trim()));
    }
    JsonValue::Array(elements)
}


pub fn json_scan(json: &str) -> HashMap<String, JsonValue> {
    if let JsonValue::Object(object) = json_get_value(json) {
        object
    } else {
        HashMap::new()
    }
}
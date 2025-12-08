use saffron_data::json::{Json, JsonElement};
use saffron_data::parse::Parse;
use std::collections::HashMap;

#[test]
fn test_parse_null() {
    let result = Json::parse("null").unwrap();
    assert_eq!(result.root, JsonElement::Null);
}

#[test]
fn test_parse_boolean_true() {
    let result = Json::parse("true").unwrap();
    assert_eq!(result.root, JsonElement::Boolean(true));
}

#[test]
fn test_parse_boolean_false() {
    let result = Json::parse("false").unwrap();
    assert_eq!(result.root, JsonElement::Boolean(false));
}

#[test]
fn test_parse_number_integer() {
    let result = Json::parse("42").unwrap();
    assert_eq!(result.root, JsonElement::Number(42.0));
}

#[test]
fn test_parse_number_float() {
    let result = Json::parse("3.14159").unwrap();
    assert_eq!(result.root, JsonElement::Number(3.14159));
}

#[test]
fn test_parse_number_negative() {
    let result = Json::parse("-42.5").unwrap();
    assert_eq!(result.root, JsonElement::Number(-42.5));
}

#[test]
fn test_parse_string_simple() {
    let result = Json::parse(r#""hello world""#).unwrap();
    assert_eq!(result.root, JsonElement::String("hello world".to_string()));
}

#[test]
fn test_parse_string_with_escapes() {
    let result = Json::parse(r#""line1\nline2\ttab""#).unwrap();
    assert_eq!(
        result.root,
        JsonElement::String("line1\nline2\ttab".to_string())
    );
}

#[test]
fn test_parse_string_with_quotes() {
    let result = Json::parse(r#""He said \"hello\"""#).unwrap();
    assert_eq!(
        result.root,
        JsonElement::String(r#"He said "hello""#.to_string())
    );
}

#[test]
fn test_parse_empty_array() {
    let result = Json::parse("[]").unwrap();
    assert_eq!(result.root, JsonElement::Array(vec![]));
}

#[test]
fn test_parse_array_numbers() {
    let result = Json::parse("[1, 2, 3, 4, 5]").unwrap();
    assert_eq!(
        result.root,
        JsonElement::Array(vec![
            JsonElement::Number(1.0),
            JsonElement::Number(2.0),
            JsonElement::Number(3.0),
            JsonElement::Number(4.0),
            JsonElement::Number(5.0),
        ])
    );
}

#[test]
fn test_parse_array_mixed_types() {
    let result = Json::parse(r#"[1, "text", true, null, false]"#).unwrap();
    assert_eq!(
        result.root,
        JsonElement::Array(vec![
            JsonElement::Number(1.0),
            JsonElement::String("text".to_string()),
            JsonElement::Boolean(true),
            JsonElement::Null,
            JsonElement::Boolean(false),
        ])
    );
}

#[test]
fn test_parse_nested_arrays() {
    let result = Json::parse("[[1, 2], [3, 4], [5]]").unwrap();
    assert_eq!(
        result.root,
        JsonElement::Array(vec![
            JsonElement::Array(vec![JsonElement::Number(1.0), JsonElement::Number(2.0)]),
            JsonElement::Array(vec![JsonElement::Number(3.0), JsonElement::Number(4.0)]),
            JsonElement::Array(vec![JsonElement::Number(5.0)]),
        ])
    );
}

#[test]
fn test_parse_empty_object() {
    let result = Json::parse("{}").unwrap();
    assert_eq!(result.root, JsonElement::Object(HashMap::new()));
}

#[test]
fn test_parse_simple_object() {
    let result = Json::parse(r#"{"name": "Saffron", "version": 1}"#).unwrap();

    if let JsonElement::Object(map) = result.root {
        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get("name"),
            Some(&JsonElement::String("Saffron".to_string()))
        );
        assert_eq!(map.get("version"), Some(&JsonElement::Number(1.0)));
    } else {
        panic!("Expected Object");
    }
}

#[test]
fn test_parse_object_with_all_types() {
    let result =
        Json::parse(r#"{"string": "value", "number": 42, "bool": true, "null": null}"#).unwrap();

    if let JsonElement::Object(map) = result.root {
        assert_eq!(map.len(), 4);
        assert_eq!(
            map.get("string"),
            Some(&JsonElement::String("value".to_string()))
        );
        assert_eq!(map.get("number"), Some(&JsonElement::Number(42.0)));
        assert_eq!(map.get("bool"), Some(&JsonElement::Boolean(true)));
        assert_eq!(map.get("null"), Some(&JsonElement::Null));
    } else {
        panic!("Expected Object");
    }
}

#[test]
fn test_parse_nested_objects() {
    let result = Json::parse(r#"{"user": {"id": 1, "name": "Alice"}, "active": true}"#).unwrap();

    if let JsonElement::Object(outer_map) = result.root {
        assert_eq!(outer_map.len(), 2);
        assert_eq!(outer_map.get("active"), Some(&JsonElement::Boolean(true)));

        if let Some(JsonElement::Object(inner_map)) = outer_map.get("user") {
            assert_eq!(inner_map.len(), 2);
            assert_eq!(inner_map.get("id"), Some(&JsonElement::Number(1.0)));
            assert_eq!(
                inner_map.get("name"),
                Some(&JsonElement::String("Alice".to_string()))
            );
        } else {
            panic!("Expected nested Object");
        }
    } else {
        panic!("Expected Object");
    }
}

#[test]
fn test_parse_object_with_array() {
    let result = Json::parse(r#"{"items": [1, 2, 3], "count": 3}"#).unwrap();

    if let JsonElement::Object(map) = result.root {
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("count"), Some(&JsonElement::Number(3.0)));

        if let Some(JsonElement::Array(arr)) = map.get("items") {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], JsonElement::Number(1.0));
            assert_eq!(arr[1], JsonElement::Number(2.0));
            assert_eq!(arr[2], JsonElement::Number(3.0));
        } else {
            panic!("Expected Array");
        }
    } else {
        panic!("Expected Object");
    }
}

#[test]
fn test_parse_complex_structure() {
    let json_str = r#"{
        "project": "Saffron",
        "version": 0.1,
        "features": ["fast", "minimal", "rust"],
        "config": {
            "port": 8080,
            "ssl": true
        }
    }"#;

    let result = Json::parse(json_str).unwrap();

    if let JsonElement::Object(map) = result.root {
        assert_eq!(
            map.get("project"),
            Some(&JsonElement::String("Saffron".to_string()))
        );
        assert_eq!(map.get("version"), Some(&JsonElement::Number(0.1)));

        if let Some(JsonElement::Array(features)) = map.get("features") {
            assert_eq!(features.len(), 3);
        } else {
            panic!("Expected features array");
        }

        if let Some(JsonElement::Object(config)) = map.get("config") {
            assert_eq!(config.get("port"), Some(&JsonElement::Number(8080.0)));
            assert_eq!(config.get("ssl"), Some(&JsonElement::Boolean(true)));
        } else {
            panic!("Expected config object");
        }
    } else {
        panic!("Expected Object");
    }
}

#[test]
fn test_parse_whitespace_handling() {
    let result = Json::parse("  \n\t  {  \n  \"key\"  :  \"value\"  \n  }  \n  ").unwrap();

    if let JsonElement::Object(map) = result.root {
        assert_eq!(
            map.get("key"),
            Some(&JsonElement::String("value".to_string()))
        );
    } else {
        panic!("Expected Object");
    }
}

#[test]
fn test_error_unterminated_string() {
    let result = Json::parse(r#""unclosed string"#);
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_token() {
    let result = Json::parse("undefined");
    assert!(result.is_err());
}

#[test]
fn test_error_missing_colon() {
    let result = Json::parse(r#"{"key" "value"}"#);
    assert!(result.is_err());
}

#[test]
fn test_error_missing_comma_in_object() {
    let result = Json::parse(r#"{"key1": "value1" "key2": "value2"}"#);
    assert!(result.is_err());
}

#[test]
fn test_error_missing_comma_in_array() {
    let result = Json::parse("[1 2 3]");
    assert!(result.is_err());
}

#[test]
fn test_error_unclosed_object() {
    let result = Json::parse(r#"{"key": "value""#);
    assert!(result.is_err());
}

#[test]
fn test_error_unclosed_array() {
    let result = Json::parse("[1, 2, 3");
    assert!(result.is_err());
}

#[test]
fn test_error_trailing_comma_object() {
    let result = Json::parse(r#"{"key": "value",}"#);
    assert!(result.is_err());
}

#[test]
fn test_error_trailing_comma_array() {
    let result = Json::parse("[1, 2, 3,]");
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_number() {
    let result = Json::parse("123.456.789");
    assert!(result.is_err());
}

#[test]
fn test_error_object_key_not_string() {
    let result = Json::parse("{123: \"value\"}");
    assert!(result.is_err());
}

#[test]
fn test_parse_single_quotes() {
    let result = Json::parse("'hello'").unwrap();
    assert_eq!(result.root, JsonElement::String("hello".to_string()));
}

#[test]
fn test_parse_object_with_single_quotes() {
    let result = Json::parse("{'key': 'value'}").unwrap();

    if let JsonElement::Object(map) = result.root {
        assert_eq!(
            map.get("key"),
            Some(&JsonElement::String("value".to_string()))
        );
    } else {
        panic!("Expected Object");
    }
}

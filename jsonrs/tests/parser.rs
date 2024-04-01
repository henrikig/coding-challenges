use jsonrs::{lexer::Lexer, parser::Parser};

#[test]
fn test_parse_empty_object() {
    let input = String::from("{}");
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    let result = p.parse();

    assert!(result.is_ok(), "{:?}", result.err());
}

#[test]
fn test_parse_single_key_value() {
    let input = String::from(r#"{"key": "value"}"#);
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    let result = p.parse();

    assert!(result.is_ok(), "{:?}", result.err());
}

#[test]
fn test_parse_double_key_value() {
    let input = String::from(
        r#"{
    "key": "value",
    "key2": "value"
}"#,
    );
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    let result = p.parse();

    assert!(result.is_ok(), "{:?}", result.err());
}

#[test]
fn test_parse_special_keywords() {
    let input = String::from(
        r#"{
  "key1": true,
  "key2": false,
  "key3": null,
  "key4": "some value",
  "key5": 101
}"#,
    );
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    let result = p.parse();

    assert!(result.is_ok(), "{:?}", result.err());
}

#[test]
fn test_parse_nested_object() {
    let input = String::from(
        r#"{
  "key1": "value",
  "key2": {
    "key3": "value"
  }
}"#,
    );
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    let result = p.parse();

    assert!(result.is_ok(), "{:?}", result.err());
}

#[test]
#[should_panic]
fn test_parse_invalid_json() {
    let input = String::from("{");
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    let result = p.parse();

    assert!(result.is_ok(), "{:?}", result.err());
}

#[test]
#[should_panic]
fn test_trailing_comma() {
    let input = String::from(r#"{"key": "value",}"#);
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    let result = p.parse();

    assert!(result.is_ok(), "{:?}", result.err());
}

#[test]
#[should_panic]
fn test_parse_invalid_json2() {
    let input = String::from(
        r#"{
  "key1": true,
  key2: false,
}"#,
    );
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    let result = p.parse();

    assert!(result.is_ok(), "{:?}", result.err());
}

#[test]
#[should_panic]
fn test_illegal_keywords() {
    let input = String::from(
        r#"{
  "key1": true,
  "key2": False,
  "key3": null,
  "key4": "value",
  "key5": 101
}
"#,
    );
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    let result = p.parse();

    assert!(result.is_ok(), "{:?}", result.err());
}

#[test]
fn test_single_list_value() {
    let input = String::from(
        r#"{
  "key-l": ["list value"]
}
"#,
    );
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    let result = p.parse();

    assert!(result.is_ok(), "{:?}", result.err());
}

#[test]
fn test_multiple_list_value() {
    let input = String::from(
        r#"{
  "key-l": ["list value"],
  "key-l2": [1, 2, 3]
}
"#,
    );
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);

    let result = p.parse();

    assert!(result.is_ok(), "{:?}", result.err());
}

use jsonrs::{
    lexer::Lexer,
    token::{Token, TokenType},
};

#[test]
fn test_simple_object() {
    let input = String::from("{}");
    let mut l = Lexer::new(input);

    let expected_tokens = vec![
        Token::new(TokenType::LBRACE, String::from("{")),
        Token::new(TokenType::RBRACE, String::from("}")),
    ];

    for expected in expected_tokens {
        let token = l.next_token();
        assert_eq!(token, expected, "{:?} does not equal {:?}", token, expected);
    }
}

#[test]
fn test_single_key_value() {
    let input = String::from(r#"{"key": "value"}"#);
    let mut l = Lexer::new(input);

    let expected_tokens = vec![
        Token::new(TokenType::LBRACE, String::from("{")),
        Token::new(TokenType::STRING, String::from("key")),
        Token::new(TokenType::COLON, String::from(":")),
        Token::new(TokenType::STRING, String::from("value")),
        Token::new(TokenType::RBRACE, String::from("}")),
        Token::new(TokenType::EOF, String::from("\0")),
    ];

    for expected in expected_tokens {
        let token = l.next_token();
        assert_eq!(token, expected, "got {:?}, expected {:?}", token, expected);
    }
}

#[test]
fn test_multiple_key_value() {
    let input = String::from(
        r#"{
  "key": "value",
  "key2": "value2"
}"#,
    );
    let mut l = Lexer::new(input);

    let expected_tokens = vec![
        Token::new(TokenType::LBRACE, String::from("{")),
        Token::new(TokenType::STRING, String::from("key")),
        Token::new(TokenType::COLON, String::from(":")),
        Token::new(TokenType::STRING, String::from("value")),
        Token::new(TokenType::COMMA, String::from(",")),
        Token::new(TokenType::STRING, String::from("key2")),
        Token::new(TokenType::COLON, String::from(":")),
        Token::new(TokenType::STRING, String::from("value2")),
        Token::new(TokenType::RBRACE, String::from("}")),
        Token::new(TokenType::EOF, String::from("\0")),
    ];

    for expected in expected_tokens {
        let token = l.next_token();
        assert_eq!(token, expected, "got {:?}, expected {:?}", token, expected);
    }
}

#[test]
fn test_special_keywords() {
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

    let expected_tokens = vec![
        Token::new(TokenType::LBRACE, String::from("{")),
        // true
        Token::new(TokenType::STRING, String::from("key1")),
        Token::new(TokenType::COLON, String::from(":")),
        Token::new(TokenType::TRUE, String::from("true")),
        Token::new(TokenType::COMMA, String::from(",")),
        // false
        Token::new(TokenType::STRING, String::from("key2")),
        Token::new(TokenType::COLON, String::from(":")),
        Token::new(TokenType::FALSE, String::from("false")),
        Token::new(TokenType::COMMA, String::from(",")),
        // null
        Token::new(TokenType::STRING, String::from("key3")),
        Token::new(TokenType::COLON, String::from(":")),
        Token::new(TokenType::NULL, String::from("null")),
        Token::new(TokenType::COMMA, String::from(",")),
        // string
        Token::new(TokenType::STRING, String::from("key4")),
        Token::new(TokenType::COLON, String::from(":")),
        Token::new(TokenType::STRING, String::from("some value")),
        Token::new(TokenType::COMMA, String::from(",")),
        // int
        Token::new(TokenType::STRING, String::from("key5")),
        Token::new(TokenType::COLON, String::from(":")),
        Token::new(TokenType::INT, String::from("101")),
        Token::new(TokenType::RBRACE, String::from("}")),
        Token::new(TokenType::EOF, String::from("\0")),
    ];

    for expected in expected_tokens {
        let token = l.next_token();
        assert_eq!(token, expected, "got {:?}, expected {:?}", token, expected);
    }
}

#[test]
fn test_illegal_idents() {
    let input = String::from(
        r#"{
  "key1": ttrue,
  "key2": falsy,
}"#,
    );
    let mut l = Lexer::new(input);

    let expected_tokens = vec![
        Token::new(TokenType::LBRACE, String::from("{")),
        // true
        Token::new(TokenType::STRING, String::from("key1")),
        Token::new(TokenType::COLON, String::from(":")),
        Token::new(TokenType::ILLEGAL, String::from("ttrue")),
        Token::new(TokenType::COMMA, String::from(",")),
        // false
        Token::new(TokenType::STRING, String::from("key2")),
        Token::new(TokenType::COLON, String::from(":")),
        Token::new(TokenType::ILLEGAL, String::from("falsy")),
        Token::new(TokenType::COMMA, String::from(",")),
    ];

    for expected in expected_tokens {
        let token = l.next_token();
        assert_eq!(token, expected, "got {:?}, expected {:?}", token, expected);
    }
}

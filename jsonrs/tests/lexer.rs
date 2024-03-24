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
        Token::new(TokenType::QUOTE, String::from(r#"""#)),
        Token::new(TokenType::IDENT, String::from("key")),
        Token::new(TokenType::QUOTE, String::from(r#"""#)),
        Token::new(TokenType::COLON, String::from(":")),
        Token::new(TokenType::QUOTE, String::from(r#"""#)),
        Token::new(TokenType::IDENT, String::from("value")),
        Token::new(TokenType::QUOTE, String::from(r#"""#)),
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
        Token::new(TokenType::QUOTE, String::from(r#"""#)),
        Token::new(TokenType::IDENT, String::from("key")),
        Token::new(TokenType::QUOTE, String::from(r#"""#)),
        Token::new(TokenType::COLON, String::from(":")),
        Token::new(TokenType::QUOTE, String::from(r#"""#)),
        Token::new(TokenType::IDENT, String::from("value")),
        Token::new(TokenType::QUOTE, String::from(r#"""#)),
        Token::new(TokenType::COMMA, String::from(",")),
        Token::new(TokenType::QUOTE, String::from(r#"""#)),
        Token::new(TokenType::IDENT, String::from("key2")),
        Token::new(TokenType::QUOTE, String::from(r#"""#)),
        Token::new(TokenType::COLON, String::from(":")),
        Token::new(TokenType::QUOTE, String::from(r#"""#)),
        Token::new(TokenType::IDENT, String::from("value2")),
        Token::new(TokenType::QUOTE, String::from(r#"""#)),
        Token::new(TokenType::RBRACE, String::from("}")),
        Token::new(TokenType::EOF, String::from("\0")),
    ];

    for expected in expected_tokens {
        let token = l.next_token();
        assert_eq!(token, expected, "got {:?}, expected {:?}", token, expected);
    }
}

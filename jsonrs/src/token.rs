#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,

    COLON,
    COMMA,

    STRING,
    INT,

    TRUE,
    FALSE,
    NULL,

    ILLEGAL,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    // TODO: operate on slices
    pub literal: String,
}

impl Token {
    pub fn new(r#type: TokenType, literal: String) -> Self {
        Token {
            token_type: r#type,
            literal,
        }
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            "true" => Token::new(TokenType::TRUE, String::from("true")),
            "false" => Token::new(TokenType::FALSE, String::from("false")),
            "null" => Token::new(TokenType::NULL, String::from("null")),
            _ => Token::new(TokenType::ILLEGAL, String::from(value)),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type && self.literal == other.literal
    }
}

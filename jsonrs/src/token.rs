#[derive(PartialEq, Debug)]
pub enum TokenType {
    LBRACE,
    RBRACE,

    QUOTE,
    COLON,
    COMMA,

    IDENT,
    INT,

    TRUE,
    FALSE,

    ILLEGAL,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    r#type: TokenType,
    // TODO: operate on slices
    literal: String,
}

impl Token {
    pub fn new(r#type: TokenType, literal: String) -> Self {
        Token { r#type, literal }
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            "true" => Token::new(TokenType::TRUE, String::from("true")),
            "false" => Token::new(TokenType::FALSE, String::from("false")),
            _ => todo!(),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.literal == other.literal
    }
}

use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '0',
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '{' => Token::new(TokenType::LBRACE, self.ch.into()),
            '}' => Token::new(TokenType::RBRACE, self.ch.into()),
            '[' => Token::new(TokenType::LBRACKET, self.ch.into()),
            ']' => Token::new(TokenType::RBRACKET, self.ch.into()),
            ':' => Token::new(TokenType::COLON, self.ch.into()),
            ',' => Token::new(TokenType::COMMA, self.ch.into()),
            '\0' => Token::new(TokenType::EOF, "\0".into()),
            '"' => {
                let string = self.read_string();
                return Token::new(TokenType::STRING, string);
            }
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_literal();
                    return Token::from(literal.as_ref());
                }
                if is_digit(self.ch) {
                    let literal = self.read_digit();
                    return Token::new(TokenType::INT, literal);
                }
                return Token::new(TokenType::ILLEGAL, self.ch.into());
            }
        };

        self.read_char();

        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            // TODO: find better way to index string
            self.ch = self.input.chars().nth(self.read_position).expect(&format!(
                "could not read character at position {}",
                self.read_position
            ));
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        self.read_char();
        while self.ch != '"' {
            self.read_char()
        }
        let res = self.input[position..self.position].into();
        self.read_char();
        res
    }

    fn read_digit(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char()
        }

        let res = self.input[position..self.position].into();
        res
    }

    fn read_literal(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) || is_digit(self.ch) {
            self.read_char()
        }

        let res = self.input[position..self.position].into();
        res
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}

fn is_letter(input: char) -> bool {
    matches!(input, 'A'..='Z' | 'a'..='z' | '_')
}

fn is_digit(input: char) -> bool {
    matches!(input, '0'..='9' | '.')
}

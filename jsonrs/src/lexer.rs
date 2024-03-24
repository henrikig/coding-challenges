use crate::token::{Token, TokenType};

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
            '{' => Token::new(TokenType::LBRACE, String::from(self.ch)),
            '}' => Token::new(TokenType::RBRACE, String::from(self.ch)),
            ':' => Token::new(TokenType::COLON, String::from(self.ch)),
            ',' => Token::new(TokenType::COMMA, String::from(self.ch)),
            '\0' => Token::new(TokenType::EOF, String::from("\0")),
            '"' => Token::new(TokenType::QUOTE, String::from(self.ch)),
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return Token::new(TokenType::IDENT, literal);
                }
                if is_digit(self.ch) {
                    let literal = self.read_digit();
                    return Token::new(TokenType::INT, literal);
                }
                return Token::new(TokenType::ILLEGAL, String::from(self.ch));
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

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch != '"' {
            self.read_char()
        }

        let res = String::from(&self.input[position..self.position]);
        res
    }

    fn read_digit(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char()
        }

        let res = String::from(&self.input[position..self.position]);
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
    matches!(input, '0'..='9')
}

#[cfg(test)]
mod tests {
    use crate::lexer::is_letter;

    #[test]
    fn it_works() {
        let res = is_letter('=');
        assert_eq!(res, false);
    }

    #[test]
    fn quote_not_whitespace() {
        assert!(!'"'.is_whitespace());
    }
}

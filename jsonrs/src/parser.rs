use anyhow::{anyhow, ensure, Context, Result};

use crate::{
    lexer::Lexer,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Parser<'a> {
    l: &'a mut Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer) -> Parser<'a> {
        let mut p = Parser {
            l,
            cur_token: Token::new(TokenType::EOF, String::from("\0")),
            peek_token: Token::new(TokenType::EOF, String::from("\0")),
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse(&mut self) -> Result<()> {
        ensure!(
            self.cur_token.token_type == TokenType::LBRACE,
            "JSON object should start with `{{`, got {:?}",
            self.cur_token.literal
        );

        self.parse_object().context("Could not parse object")?;
        self.next_token();
        ensure!(
            self.cur_token.token_type == TokenType::EOF,
            "expected EOF, got {:?}",
            self.cur_token.token_type
        );
        Ok(())
    }

    pub fn parse_object(&mut self) -> Result<()> {
        ensure!(
            self.cur_token.token_type == TokenType::LBRACE,
            "expected `{{`, got {:?}",
            self.cur_token.literal
        );

        self.next_token();
        while self.cur_token.token_type != TokenType::RBRACE {
            self.parse_pair()?;
        }

        ensure!(
            self.cur_token.token_type == TokenType::RBRACE,
            "objects should end with `}}`, got {:?}",
            self.cur_token.literal
        );
        Ok(())
    }

    fn parse_pair(&mut self) -> Result<()> {
        ensure!(
            self.cur_token.token_type == TokenType::STRING,
            "expected JSON key, got {:?}",
            self.cur_token
        );

        self.next_token();
        ensure!(
            self.cur_token.token_type == TokenType::COLON,
            "expected `:` after key, got {:?}",
            self.cur_token
        );

        self.next_token();
        self.parse_value()?;

        if self.peek_token.token_type == TokenType::COMMA {
            self.next_token();

            if self.peek_token.token_type == TokenType::RBRACE {
                return Err(anyhow!("json objects should not end with a comma"));
            }
        }
        self.next_token();

        Ok(())
    }

    fn parse_list(&mut self) -> Result<()> {
        ensure!(
            self.cur_token.token_type == TokenType::LBRACKET,
            "lists should start with `[`, got {:?}",
            self.cur_token.literal
        );

        self.next_token();

        loop {
            if self.cur_token.token_type == TokenType::RBRACKET {
                break;
            }
            self.parse_value()?;
            self.next_token();

            if self.cur_token.token_type != TokenType::COMMA {
                break;
            }
            self.next_token();
        }

        ensure!(
            self.cur_token.token_type == TokenType::RBRACKET,
            "lists should end with `]`, got {:?}",
            self.cur_token.literal
        );

        Ok(())
    }

    fn parse_value(&mut self) -> Result<()> {
        match self.cur_token.token_type {
            TokenType::LBRACE => self.parse_object()?,
            TokenType::LBRACKET => self.parse_list()?,
            TokenType::STRING
            | TokenType::INT
            | TokenType::TRUE
            | TokenType::FALSE
            | TokenType::NULL => (),
            _ => return Err(anyhow!("unexpected token {:?}", self.cur_token)),
        };

        Ok(())
    }
}

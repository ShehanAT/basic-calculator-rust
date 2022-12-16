use std::fmt;
use parser::token;
use parser::token::Token::*;

pub struct Lexer {
    pub curr: char,
    pub pos: usize, 
    pub src: String,
    pub eof: bool 
}

impl Lexer {
    pub fn new(src: &str) -> Lexer {
        let mut l = Lexer {
            curr: '\0',
            pos: 0,
            src: src.to_string(),
            eof: false
        };
        if l.pos >= src.len() {
            l.eof = true;
        } else {
            l.curr = src.chars().nth(0).unwrap();
        }
        l
    }

    pub fn next_token(&mut self) -> Result<token::Token, String> {
        if self.eof {
            return Ok(EOF);
        }
        self.consume_whitespace();
        match self.curr {
            '(' => { self.bump(); Ok(LPAREN) }
            ')' => { self.bump(); Ok(RPAREN) }
            c if c.is_digit(10) => {
                let start = self.pos;
                let mut end = start + 1;
                self.bump();
            }
        }
    }
}
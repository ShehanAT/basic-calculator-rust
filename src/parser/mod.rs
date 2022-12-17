pub use self::token::Token::*;

pub mod ast;
pub mod lexer;
pub mod token;


pub struct Parser {
    pub current: token::Token,
    pub lexer: lexer::Lexer,
    pub peeked: Option<token::Token>,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        let l = lexer::Lexer::new(input);
        let p = Parser {
            current: EOF,
            peeked: None, 
            lexer: l
        };
        p
    }

    pub fn parse(&mut self) -> Result<Box<ast::Node>, String> {
        self.expr(1)
    }

    pub fn expr(&mut self, prec: usize) -> Result<Box<ast::Node>, String> {
        let mut lhr = try!(self.atom());
        let mut rhs;
        loop {
            let curr = try!(self.peek_token());

            if token::is_eof(&curr) {
                break;
            }
            let info_tuple = curr.info();

        }
        
    }
}
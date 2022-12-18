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
            if info_tuple.is_none() {
                break;
            }
            let (op_prec, op_assoc) = info_tuple.unwrap();
            if op_prec < prec {
                break;
            }
            try!(self.next_token());
            match op_assoc {
                0 => {
                    rhs = try!(self.expr(op_prec + 1));
                }
                _ => {
                    rhs = try!(self.expr(op_prec));
                }
            }
            lhs = self.op(curr, lhs, rhs);

        }
        Ok(lhs)
    }

    pub fn atom(&mut self) -> Result<Box<ast::Node>, String> {
        match try!(self.peek_token()){
            EOF => { Ok(Box::new(ast::Num {num: 0f64 }))}
            LPAREN => {
                try!(self.expect('(')));
                let e = try!(self.expr(1));
                try!(self.expect(')'));
                Ok(e) 
            }
        }
    }
}
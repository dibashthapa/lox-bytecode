use crate::{
    scanner::Scanner,
    token::{Token, TokenType},
};

struct Parser {
    current: Token,
    previous: Token,
}

impl Parser {
    pub fn compile(&self, source: &String) -> bool {
        let mut scanner = Scanner::new(source);
        self.advance(&scanner);
        // let line = 0;
        // loop {
        //     let token = scanner.scan_token();

        //     if token.line != line {
        //         print!("{:4} ", token.line);
        //         line = token.line;
        //     } else {
        //         print!("   | ")
        //     }

        //     println!("'{}'", token.ttype);

        //     if token.ttype == TokenType::Eof {
        //         break;
        //     }
        // }
    }

    pub fn advance(&mut self, scanner: &Scanner) {
        self.previous = self.current;

        loop {
            self.current = scanner.scan_token();

            if self.current.ttype != TokenType::TokenError {
                break;
            }
        }
    }

    fn error_at_current(&self, message: &str) {

    }

    fn error_at(&self) {
        
    }
}

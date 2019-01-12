use crate::ast::{ Ast };
use crate::lexier::{ Lexier };
use crate::token::{ Token };

pub struct Parser {
    lexier: Lexier,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexier: Lexier) -> Parser {
        let mut parser = Parser {
            lexier: lexier,
            cur_token:  Token::Illegal("".to_string()),
            peek_token: Token::Illegal("".to_string()),
            errors: Vec::new(),
        };

        parser.next_token();
        parser.next_token();
        
        parser        
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexier.next_token();
    }
    
    pub fn parse_program(&mut self) -> Option<Ast> {
        let mut program = Ast::Program { statements: Box::new(Vec::new()) };

        while !self.cur_token_is("Eof".to_string()) {
            if let Some(value) = self.parse_statement() {
                if let Ast::Program { ref mut statements } = program {
                    statements.push(Box::new(value));
                }
            }
            else {
                self.next_token();
            }
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Ast> {
        match self.cur_token {
            Token::Let(_)    => self.parse_let_statement(),
            Token::Return(_) => return None, // ToDo
            _                => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Ast> {
        if !self.expect_peek("Ident".to_string()) {
            return None;
        }

        let identifier = Box::new(Ast::Identifier {
            value: Box::new(self.cur_token.clone().literal()),
        });

        if !self.expect_peek("Assign".to_string()) {
            return None;
        }

        self.next_token();
        
        let value = match self.parse_expression() {
            Some(value) => Box::new(value),
            None        => return None,
        };

        if self.peek_token_is("Semicolon".to_string()) {
            self.next_token();
        }
        
        Some(Ast::LetStatement {
            identifier: identifier,
            value: value,
        })
    }
    
    fn parse_expression_statement(&mut self) -> Option<Ast> {
        let expression = match self.parse_expression() {
            Some(value) => Box::new(value),
            None        => {
                self.parse_error();
                return None;
            },
        };

        if self.peek_token == Token::Semicolon(";".to_string()) {
            self.next_token();
        }

        Some(Ast::ExpressionStatement { expression: expression })
    }

    fn parse_expression(&mut self) -> Option<Ast> {
        // ToDo
        let left_exp = match self.cur_token {
            Token::Integer (_) => self.parse_integer_literal(),
            _                  => return None,
        };
        
        self.next_token();

        left_exp
    }

    fn parse_integer_literal(&self) -> Option<Ast> {
        let value = match self.cur_token.literal().parse::<i64>() {
            Ok(value) => value,
            Err(_)    => return None,
        };

        Some(Ast::Integer { value: value })
    }

    fn cur_token_is(&self, kind: String) -> bool {
        self.cur_token.kind() == kind
    }

    fn peek_token_is(&self, kind: String) -> bool {
        self.peek_token.kind() == kind
    }

    fn expect_peek(&mut self, kind: String) -> bool {
        if self.peek_token_is(kind) {
            self.next_token();
            return true;
        }

        false
    }
    
    fn parse_error(&mut self) {
        let msg = format!("there is no matching pattern for {} found", self.cur_token.clone().literal());
        self.errors.push(msg);
    }
    
}

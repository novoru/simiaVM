use crate::ast::{ Ast };
use crate::lexier::{ Lexier };
use crate::token::{ Token, TokenKind };

#[derive(PartialEq,Clone)]
enum Precedence {
    Lowest,
    Equals,         // '='
    LessGreater,    // '<' | '>'
    Sum,            // '+'
    Product,        // '*'
    Prefix,         // '-X' | '!X'
    Call,           // function(X)
    Index,          // [X]
}

fn precedences(kind: TokenKind) -> Precedence {
    match kind {
        TokenKind::Eq       |
        TokenKind::NotEq    => Precedence::Equals,
        TokenKind::Lt       |
        TokenKind::Gt       => Precedence::LessGreater,
        TokenKind::Plus     |
        TokenKind::Minus    => Precedence::Sum,
        TokenKind::Slash    |
        TokenKind::Asterisk => Precedence::Product,
        TokenKind::Lparen   => Precedence::Call,
        TokenKind::Lbracket => Precedence::Index,
        _                   => Precedence::Lowest, 
        
    }    
}

pub struct Parser {
    lexier: Lexier,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(lexier: Lexier) -> Parser {
        let mut parser = Parser {
            lexier: lexier,
            cur_token:  Token { kind: TokenKind::Illegal, literal: "".to_string() },
            peek_token: Token { kind: TokenKind::Illegal, literal: "".to_string() },
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

        while !self.cur_token_is(TokenKind::Eof) {
            if let Some(value) = self.parse_statement() {
                if let Ast::Program { ref mut statements } = program {
                    statements.push(Box::new(value));
                }
            }
            else {
                self.next_token();
                continue;
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Ast> {
        match self.cur_token.kind {
            TokenKind::Let    => self.parse_let_statement(),
            TokenKind::Return => self.parse_return_statement(),
            _                 => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Ast> {
        if !self.expect_peek(TokenKind::Identifier) {
            return None;
        }

        let identifier = Box::new(Ast::Identifier {
            value: Box::new(self.cur_token.clone().literal()),
        });

        if !self.expect_peek(TokenKind::Assign) {
            return None;
        }

        self.next_token();
        
        let value = match self.parse_expression(Precedence::Lowest) {
            Some(value) => Box::new(value),
            None        => return None,
        };

        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }
        
        Some(Ast::LetStatement {
            identifier: identifier,
            value: value,
        })
    }

    fn parse_return_statement(&mut self) -> Option<Ast> {
        self.next_token();

        let return_value = match self.parse_expression(Precedence::Lowest) {
            Some(value) => Box::new(value),
            None        => return None,
        };

        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        Some(Ast::ReturnStatement {
            return_value: return_value,
        })
    }
    
    fn parse_expression_statement(&mut self) -> Option<Ast> {
        let expression = match self.parse_expression(Precedence::Lowest) {
            Some(value) => Box::new(value),
            None        => {
                return None;
            },
        };

        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        Some(Ast::ExpressionStatement { expression: expression })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Ast> {
        // ToDo
        let mut left_exp = match self.cur_token.kind.clone() {
            TokenKind::Integer  => self.parse_integer_literal(),
            TokenKind::True     |
            TokenKind::False    => self.parse_boolean_literal(),
            TokenKind::String   => self.parse_string_literal(),
            TokenKind::Bang     |
            TokenKind::Minus    => self.parse_prefix_expression(),
            _                   => {
                self.no_prefix_parse_fn_error(self.cur_token.kind.clone());
                return None;
            },
        };

        while !self.peek_token_is(TokenKind::Semicolon) && (precedence.clone() as u8) < (self.peek_precedence() as u8) {
            left_exp = match self.peek_token.kind {
                TokenKind::Plus     |
                TokenKind::Minus    |
                TokenKind::Asterisk |
                TokenKind::Slash    |
                TokenKind::Eq       |
                TokenKind::NotEq    => self.parse_infix_expression(left_exp),
                _ =>  return left_exp,
            };
        }
        
        left_exp
    }

    fn parse_integer_literal(&self) -> Option<Ast> {
        let value = match self.cur_token.literal().parse::<i64>() {
            Ok(value) => value,
            Err(_)    => return None,
        };

        Some(Ast::Integer { value: value })
    }

    fn parse_boolean_literal(&self) -> Option<Ast> {
        Some(Ast::Boolean { value: self.cur_token_is(TokenKind::True) })
    }

    fn parse_string_literal(&self) -> Option<Ast> {
        Some(Ast::StringLiteral { value: Box::new(self.cur_token.literal()) })
    }

    fn parse_prefix_expression(&mut self) -> Option<Ast> {
        let operator = Box::new(self.cur_token.literal());

        self.next_token();
        
        let right = match self.parse_expression(Precedence::Prefix) {
            Some(value) => Box::new(value),
            None        => return None,
        };
        
        Some(Ast::PrefixExpression {
            operator: operator,
            right: right,
        })

    }

    fn parse_infix_expression(&mut self, left_exp: Option<Ast>) -> Option<Ast> {
        self.next_token();
        
        let left = match left_exp {
            Some(value) => Box::new(value),
            None => return None,
        };

        let operator = Box::new(self.cur_token.literal());
        let precedence = self.cur_precedence().clone();
        
        self.next_token();

        let right = match self.parse_expression(precedence) {
            Some(value) => Box::new(value),
            None => return None,
        };

        Some(Ast::InfixExpression {
            left: left,
            operator: operator,
            right: right,
        })
    }
    
    fn cur_token_is(&self, kind: TokenKind) -> bool {
        self.cur_token.kind == kind
    }

    fn peek_token_is(&self, kind: TokenKind) -> bool {
        self.peek_token.kind == kind
    }
    
    fn cur_precedence(&self) -> Precedence {
        precedences(self.cur_token.kind.clone())
    }
    
    fn peek_precedence(&self) -> Precedence {
        precedences(self.peek_token.kind.clone())
    }
    
    fn expect_peek(&mut self, kind: TokenKind) -> bool {
        if self.peek_token_is(kind.clone()) {
            self.next_token();
            return true;
        }

        self.peek_error(kind);
        
        false
    }

    fn peek_error(&mut self, kind: TokenKind) {
        let msg = format!("expected next token to be {}, got {} instead",
                          self.cur_token.kind.literal(), kind.literal());
        self.errors.push(msg);
    }
    
    fn parse_error(&mut self) {
        let msg = format!("there is no matching pattern for {} found", self.cur_token.clone().literal());
        self.errors.push(msg);
    }

    fn no_prefix_parse_fn_error(&mut self, kind: TokenKind) {
        let msg = format!("no prefix parse function for {} found", kind.literal());
        self.errors.push(msg);
    }
    
    pub fn check_parser_errors(&self) {
        if self.errors.len() == 0 {
            return;
        }

        println!("parser has {} errors", self.errors.len());
        
        for msg in self.errors.iter() {
            println!("parser error: {}", msg);
        }

        panic!();
    }
    
}

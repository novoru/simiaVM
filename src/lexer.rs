use crate::token::{ Token, TokenKind };

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        lexer.read_char();
        lexer
    }
    
    pub fn next_token(&mut self) -> Token {
        let token: Token;
        self.skip();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token { kind: TokenKind::Eq, literal: "==".to_string() };
                }
                else {
                    token = Token { kind: TokenKind::Assign, literal: self.ch.to_string() };
                }
            },
            '+' => token = Token { kind: TokenKind::Plus, literal: self.ch.to_string() },
            '-' => token = Token { kind: TokenKind::Minus, literal: self.ch.to_string() },
            '*' => token = Token { kind: TokenKind::Asterisk, literal: self.ch.to_string() },
            '/' => token = Token { kind: TokenKind::Slash, literal: self.ch.to_string() },
            '!' =>  {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token { kind: TokenKind::NotEq, literal: "!=".to_string() };
                }
                else {
                    token = Token { kind: TokenKind::Bang, literal: self.ch.to_string() };
                }
            },
            '<' => token = Token { kind: TokenKind::Lt, literal: self.ch.to_string() },
            '>' => token = Token { kind: TokenKind::Gt, literal: self.ch.to_string() },
            ';' => token = Token { kind: TokenKind::Semicolon, literal: self.ch.to_string() },
            ',' => token = Token { kind: TokenKind::Comma, literal: self.ch.to_string() },
            '(' => token = Token { kind: TokenKind::Lparen, literal: self.ch.to_string() },
            ')' => token = Token { kind: TokenKind::Rparen, literal: self.ch.to_string() },
            '{' => token = Token { kind: TokenKind::Lbrace, literal: self.ch.to_string() },
            '}' => token = Token { kind: TokenKind::Rbrace, literal: self.ch.to_string() },
            '[' => token = Token { kind: TokenKind::Lbracket, literal: self.ch.to_string() },
            ']' => token = Token { kind: TokenKind::Rbracket, literal: self.ch.to_string() },
            '0' ... '9' => return Token { kind: TokenKind::Integer, literal: self.read_integer() },
            'a' ... 'z' |
            'A' ... 'Z' |
            '_'  => {
                let ident = self.read_identifier();
                if ident == "let" {
                    return Token { kind: TokenKind::Let, literal: ident };
                }
                else if ident == "fn" {
                    return Token { kind: TokenKind::Function, literal: ident };
                }
                else if ident == "if" {
                    return Token { kind: TokenKind::If, literal: ident };
                }
                else if ident == "else" {
                    return Token { kind: TokenKind::Else, literal: ident };
                }
                else if ident == "return" {
                    return Token { kind: TokenKind::Return, literal: ident };
                }
                else if ident == "true" {
                    return Token { kind: TokenKind::True, literal: ident };
                }
                else if ident == "false" {
                    return Token { kind: TokenKind::False, literal: ident };
                }

                return Token { kind: TokenKind::Identifier, literal: ident }
            },
            '"'  => token = Token { kind: TokenKind::String, literal: self.read_string() },
            '\0' => return  Token { kind: TokenKind::Eof, literal: self.ch.to_string() },
            _ => token = Token { kind: TokenKind::Illegal, literal: self.ch.to_string() },
        }
        
        self.read_char();
        token
    }

    fn skip(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\r' ||self.ch =='\n'  {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position + 1 >= self.input.len() {
            return '\0';
        }
        self.input.chars()
            .skip(self.read_position).next().unwrap()
    }
    
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        }
        else {
            self.ch = self.input.chars()
                .skip(self.read_position).next().unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_integer(&mut self) -> String {
        let start = self.position;
        
        while self.ch.is_digit(10) {
            self.read_char();
        }

        self.input[start..self.position].to_string()
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }

        self.input[start..self.position].to_string()
    }

    fn read_string(&mut self) -> String {
        let start = self.position + 1;
        self.read_char();
        
        loop {
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
            self.read_char();
        }
                
        self.input[start..self.position].to_string()
    }
}

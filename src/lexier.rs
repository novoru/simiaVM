use crate::token::{ Token };

pub struct Lexier {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexier {
    pub fn new(input: String) -> Lexier {
        let mut lexier = Lexier {
            input: input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        lexier.read_char();
        lexier
    }
    
    pub fn next_token(&mut self) -> Token {
        let token: Token;
        self.skip();

        match self.ch {
            '=' => token = Token::Assign(self.ch.to_string()),
            '+' => token = Token::Plus(self.ch.to_string()),
            '-' => token = Token::Minus(self.ch.to_string()),
            '*' => token = Token::Asterisk(self.ch.to_string()),
            '/' => token = Token::Slash(self.ch.to_string()),
            '!' => token = Token::Bang(self.ch.to_string()),
            '<' => token = Token::Lt(self.ch.to_string()),
            '>' => token = Token::Gt(self.ch.to_string()),
            ';' => token = Token::Semicolon(self.ch.to_string()),
            ',' => token = Token::Comma(self.ch.to_string()),
            '(' => token = Token::Lparen(self.ch.to_string()),
            ')' => token = Token::Rparen(self.ch.to_string()),
            '{' => token = Token::Lbrace(self.ch.to_string()),
            '}' => token = Token::Rbrace(self.ch.to_string()),
            '[' => token = Token::Lbracket(self.ch.to_string()),
            ']' => token = Token::Rbracket(self.ch.to_string()),
            '0' ... '9' => return Token::Integer(self.read_integer()),
            'a' ... 'z' |
            'A' ... 'Z' |
            '_'  => {
                let ident = self.read_identifier();
                if ident == "let" {
                    return Token::Let(ident);
                }
                else if ident == "fn" {
                    return Token::Function(ident);
                }
                else if ident == "if" {
                    return Token::If(ident);
                }
                else if ident == "else" {
                    return Token::Else(ident);
                }
                else if ident == "return" {
                    return Token::Return(ident);
                }
                else if ident == "true" {
                    return Token::True(ident);
                }
                else if ident == "false" {
                    return Token::False(ident);
                }

                return Token::Identifier(ident)
            },
            '"'  => token = Token::String(self.read_string()),
            '\0' => return  Token::Eof(self.ch.to_string()),
            _ => token = Token::Illegal(self.ch.to_string()),
        }
        
        self.read_char();
        token
    }

    fn skip(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\r' ||self.ch =='\n'  {
            self.read_char();
        }
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

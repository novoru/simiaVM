extern crate simiaVM;

use simiaVM::token::{ Token, TokenKind };
use simiaVM::lexer::{ Lexer };

#[test]
fn test_next_token() {
    let input = "     \
=+-*/!<>;,(){}[]==!=  \
100 200;              \
foo bar;              \
\"foo\" \"bar\" \"\"; \
true false;           \
return ;              \
if (true) {           \
    return a;         \
}                     \
fn(a) {               \
    return a + b;     \
}                     \
[1, 2, a];            \
".to_string();
    let mut lexer = Lexer::new(input);

    let tests = [ Token { kind: TokenKind::Assign, literal: "=".to_string() },
                  Token { kind: TokenKind::Plus, literal: "+".to_string() },
                  Token { kind: TokenKind::Minus, literal: "-".to_string() },
                  Token { kind: TokenKind::Asterisk, literal: "*".to_string() },
                  Token { kind: TokenKind::Slash, literal: "/".to_string() },
                  Token { kind: TokenKind::Bang, literal: "!".to_string() },
                  Token { kind: TokenKind::Lt, literal: "<".to_string() },
                  Token { kind: TokenKind::Gt, literal: ">".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },
                  Token { kind: TokenKind::Comma, literal: ",".to_string() },
                  Token { kind: TokenKind::Lparen, literal: "(".to_string() },
                  Token { kind: TokenKind::Rparen, literal: ")".to_string() },
                  Token { kind: TokenKind::Lbrace, literal: "{".to_string() },
                  Token { kind: TokenKind::Rbrace, literal: "}".to_string() },
                  Token { kind: TokenKind::Lbracket, literal: "[".to_string() },
                  Token { kind: TokenKind::Rbracket, literal: "]".to_string() },
                  Token { kind: TokenKind::Eq, literal: "==".to_string() },
                  Token { kind: TokenKind::NotEq, literal: "!=".to_string() },

                  Token { kind: TokenKind::Integer, literal: "100".to_string() },
                  Token { kind: TokenKind::Integer, literal: "200".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

                  Token { kind: TokenKind::Identifier, literal: "foo".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "bar".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::String, literal: "foo".to_string() },
                  Token { kind: TokenKind::String, literal: "bar".to_string() },
                  Token { kind: TokenKind::String, literal: "".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

                  Token { kind: TokenKind::True, literal: "true".to_string() },
                  Token { kind: TokenKind::False, literal: "false".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

                  Token { kind: TokenKind::Return, literal: "return".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

                  Token { kind: TokenKind::If, literal: "if".to_string() },
                  Token { kind: TokenKind::Lparen, literal: "(".to_string() },
                  Token { kind: TokenKind::True, literal: "true".to_string() },
                  Token { kind: TokenKind::Rparen, literal: ")".to_string() },
                  Token { kind: TokenKind::Lbrace, literal: "{".to_string() },
                  
                  Token { kind: TokenKind::Return, literal: "return".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "a".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

                  Token { kind: TokenKind::Rbrace, literal: "}".to_string() },
                  

                  Token { kind: TokenKind::Function, literal: "fn".to_string() },
                  Token { kind: TokenKind::Lparen, literal: "(".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "a".to_string() },
                  Token { kind: TokenKind::Rparen, literal: ")".to_string() },
                  Token { kind: TokenKind::Lbrace, literal: "{".to_string() },
                  
                  Token { kind: TokenKind::Return, literal: "return".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "a".to_string() },
                  Token { kind: TokenKind::Plus, literal: "+".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "b".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

                  Token { kind: TokenKind::Rbrace, literal: "}".to_string() },

                  Token { kind: TokenKind::Lbracket, literal: "[".to_string() },
                  Token { kind: TokenKind::Integer, literal: "1".to_string() },
                  Token { kind: TokenKind::Comma, literal: ",".to_string() },
                  Token { kind: TokenKind::Integer, literal: "2".to_string() },
                  Token { kind: TokenKind::Comma, literal: ",".to_string() },
                  Token { kind: TokenKind::Identifier, literal: "a".to_string() },
                  Token { kind: TokenKind::Rbracket, literal: "]".to_string() },
                  Token { kind: TokenKind::Semicolon, literal: ";".to_string() },
                  
                  Token { kind: TokenKind::Eof, literal: "\0".to_string() },
    ];

    for test in tests.iter() {
        let token = lexer.next_token();
        assert!(token.kind == test.kind);
        assert_eq!(token.literal, test.literal);
    }
    
}

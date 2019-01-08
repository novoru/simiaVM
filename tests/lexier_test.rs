extern crate simiaVM;

use simiaVM::token::{ Token };
use simiaVM::lexier::{ Lexier };

#[test]
fn test_next_token() {
    let input = "     \
=+-*/!<>;,(){}[]      \
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
    let mut lexier = Lexier::new(input);

    let tests = [ Token::Assign("=".to_string()),
                  Token::Plus("+".to_string()),
                  Token::Minus("-".to_string()),
                  Token::Asterisk("*".to_string()),
                  Token::Slash("/".to_string()),
                  Token::Bang("!".to_string()),
                  Token::Lt("<".to_string()),
                  Token::Gt(">".to_string()),
                  Token::Semicolon(";".to_string()),
                  Token::Comma(",".to_string()),
                  Token::Lparen("(".to_string()),
                  Token::Rparen(")".to_string()),
                  Token::Lbrace("{".to_string()),
                  Token::Rbrace("}".to_string()),
                  Token::Lbracket("[".to_string()),
                  Token::Rbracket("]".to_string()),

                  Token::Integer("100".to_string()),
                  Token::Integer("200".to_string()),
                  Token::Semicolon(";".to_string()),

                  Token::Identifier("foo".to_string()),
                  Token::Identifier("bar".to_string()),
                  Token::Semicolon(";".to_string()),
                  
                  Token::String("foo".to_string()),
                  Token::String("bar".to_string()),
                  Token::String("".to_string()),
                  Token::Semicolon(";".to_string()),

                  Token::True("true".to_string()),
                  Token::False("false".to_string()),
                  Token::Semicolon(";".to_string()),

                  Token::Return("return".to_string()),
                  Token::Semicolon(";".to_string()),

                  Token::If("if".to_string()),
                  Token::Lparen("(".to_string()),
                  Token::True("true".to_string()),
                  Token::Rparen(")".to_string()),
                  Token::Lbrace("{".to_string()),
                  
                  Token::Return("return".to_string()),
                  Token::Identifier("a".to_string()),
                  Token::Semicolon(";".to_string()),

                  Token::Rbrace("}".to_string()),
                  

                  Token::Function("fn".to_string()),
                  Token::Lparen("(".to_string()),
                  Token::Identifier("a".to_string()),
                  Token::Rparen(")".to_string()),
                  Token::Lbrace("{".to_string()),
                  
                  Token::Return("return".to_string()),
                  Token::Identifier("a".to_string()),
                  Token::Plus("+".to_string()),
                  Token::Identifier("b".to_string()),
                  Token::Semicolon(";".to_string()),

                  Token::Rbrace("}".to_string()),

                  Token::Lbracket("[".to_string()),
                  Token::Integer("1".to_string()),
                  Token::Comma(",".to_string()),
                  Token::Integer("2".to_string()),
                  Token::Comma(",".to_string()),
                  Token::Identifier("a".to_string()),
                  Token::Rbracket("]".to_string()),
                  Token::Semicolon(";".to_string()),
                  
                  Token::Eof("\0".to_string()),
    ];

    for test in tests.iter() {
        let token = lexier.next_token();
        assert_eq!(token.kind(), test.kind());
        assert_eq!(token.literal(), test.literal());
    }
    
}

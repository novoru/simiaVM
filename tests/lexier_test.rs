extern crate simiaVM;

use simiaVM::token::{ Token };
use simiaVM::lexier::{ Lexier };

#[test]
fn test_next_token() {
    let input = "=+-*/!<>;,(){}[]100 200 foo bar".to_string();
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

                  Token::Identifier("foo".to_string()),
                  Token::Identifier("bar".to_string()),
    ];

    for test in tests.iter() {
        let token = lexier.next_token();
        assert_eq!(token.kind(), test.kind());
        assert_eq!(token.literal(), test.literal());
    }
    
}

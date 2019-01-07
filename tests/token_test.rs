#[warn(non_snake_case)]
extern crate simiaVM;

use simiaVM::token::{ Token };

#[test]
fn test_token() {
    let token = Token::Identifier("foo".to_string());

    assert_eq!(token.kind(), "Identifier".to_string());
    assert_eq!(token.literal(), "foo".to_string());
}

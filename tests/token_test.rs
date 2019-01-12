#[warn(non_snake_case)]
extern crate simiaVM;

use simiaVM::token::{ Token, TokenKind };

#[test]
fn test_token() {
    let token = Token { kind: TokenKind::Identifier, literal: "foo".to_string() };

    assert_eq!(token.kind, TokenKind::Identifier);
    assert_eq!(token.literal, "foo".to_string());
}

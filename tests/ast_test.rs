extern crate simiaVM;

use simiaVM::token::{ Token };
use simiaVM::ast::{ Ast };

#[test]
fn test_ast() {
    let mut expr_stmt = Ast::ExpressionStatement{
        expression: Box::new(
            Ast::Identifier {
                value: Box::new("foo".to_string()),
            },
        ),
    };

    if let Ast::ExpressionStatement { expression } = expr_stmt.clone() {
        assert_eq!(expr_stmt.clone().kind(), "ExpressionStatement".to_string());
        if let Ast::Identifier { value } = *expression {
            assert_eq!(*value, "foo".to_string());
        }
        else {
            panic!("expr_stmt.expression not Ast::Identifier");
        }
    }
    else {
        panic!("expr_stmt not Ast::ExpressionStatement.");
    }
}

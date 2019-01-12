extern crate simiaVM;

use simiaVM::ast::{ Ast };
use simiaVM::lexier::{ Lexier };
use simiaVM::parser::{ Parser };
use simiaVM::token::{ Token };

#[test]
fn test_integer_literal_expression() {
    let tests = [("foo;", 5),
                 ("bar;", 100)
    ];
    
    for test in tests.iter() {
        let lexier = Lexier::new(test.0.to_string());
        let mut parser = Parser::new(lexier);
        let program = parser.parse_program();

        if let Ast::Program { ref mut statements } = program.unwrap() {
            for (i, statement) in statements.iter().enumerate() {
                if let Ast::ExpressionStatement { ref expression } = **statement {
                    if let Ast::Integer { value } = **expression {
                        assert_eq!(value, test.1);
                    }
                }
            }
        }
    }
    
}

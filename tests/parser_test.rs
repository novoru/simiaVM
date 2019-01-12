extern crate simiaVM;

use simiaVM::ast::{ Ast };
use simiaVM::lexier::{ Lexier };
use simiaVM::parser::{ Parser };
use simiaVM::token::{ Token };

#[test]
fn test_parse_integer_literal_expression() {
    let tests = [("5;", 5),
                 ("10;", 10)
    ];
    
    for test in tests.iter() {
        let lexier = Lexier::new(test.0.to_string());
        let mut parser = Parser::new(lexier);
        let program = parser.parse_program();

        if let Ast::Program { ref mut statements } = program.unwrap() {
            assert_eq!(statements.len(), 1);
            for statement in statements.iter() {
                if let Ast::ExpressionStatement { ref expression } = (**statement).clone() {
                    if let Ast::Integer { value } = **expression {
                        assert_eq!(value, test.1);
                    }
                }
            }
        }
    }
}

#[test]
fn test_parse_let_statement() {
    let tests = [("let a = 0;", "a", 0),
                 ("let foo = 100;", "foo", 10)
    ];

    for test in tests.iter() {
        let lexier = Lexier::new(test.0.to_string());
        let mut parser = Parser::new(lexier);
        let program = parser.parse_program();

        if let Ast::Program { ref mut statements } = program.unwrap() {
            assert_eq!(statements.len(), 1);
            for statement in statements.iter() {
                if let Ast::ExpressionStatement { ref expression } = **statement {
                    if let Ast::LetStatement { ref identifier, ref value } = **expression {
                        if let Ast::Identifier { ref value } = **identifier {
                            assert_eq!(**value, test.1.to_string());
                        }
                        if let Ast::Integer { value } = **value {
                            assert_eq!(value, test.2);
                        }
                    }
                }
            }
        }
    }
}

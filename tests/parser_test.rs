extern crate simiaVM;

use simiaVM::ast::{ Ast };
use simiaVM::lexier::{ Lexier };
use simiaVM::parser::{ Parser };
use simiaVM::token::{ Token, TokenKind };

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
                    else {
                        panic!();
                    }
                }
                else {
                    panic!();
                }
            }
        }
        else {
            panic!();
        }
    }
}

#[test]
fn test_parse_let_statement() {
    let tests = [("let a = 0;", "a", 0),
                 ("let foo = 100;", "foo", 100)
    ];

    for test in tests.iter() {
        let lexier = Lexier::new(test.0.to_string());
        let mut parser = Parser::new(lexier);
        let program = parser.parse_program();

        if let Ast::Program { ref mut statements } = program.unwrap() {
            assert_eq!(statements.len(), 1);
            for statement in statements.iter() {
                if let Ast::LetStatement { ref identifier, ref value } = **statement {
                    if let Ast::Identifier { ref value } = **identifier {
                        assert_eq!(**value, test.1.to_string());
                    }
                    else {
                        panic!();
                    }
                    if let Ast::Integer { value } = **value {
                        assert_eq!(value, test.2);
                    }
                    else{
                        panic!();
                    }

                }
                else {
                    panic!();
                }
            }
        }
        else {
            panic!();
        }
    }
}

#[test]
fn test_parse_return_statement() {
    let tests = [("return 0;", 0),
                 ("return 10;", 10)
    ];

    for test in tests.iter() {
        let lexier = Lexier::new(test.0.to_string());
        let mut parser = Parser::new(lexier);
        let program = parser.parse_program();

        if let Ast::Program { ref mut statements } = program.unwrap() {
            assert_eq!(statements.len(), 1);
            for statement in statements.iter() {
                if let Ast::ReturnStatement { ref return_value } = **statement {
                    if let Ast::Integer { ref value } = **return_value {
                        assert_eq!(*value, test.1);
                    }
                    else {
                        panic!();
                    }
                }
                else {
                    panic!();
                }
            }
        }
        else {
            panic!();
        }
    }
}

extern crate simiaVM;

use simiaVM::ast::{ Ast };
use simiaVM::lexer::{ Lexer };
use simiaVM::parser::{ Parser };
use simiaVM::token::{ Token, TokenKind };

#[derive(Clone)]
enum TestType {
    Integer(i64),
    String(String),
    Boolean(bool),
}

fn test_literal(literal: Ast, expected: TestType) -> bool {
    match literal.clone() {
        Ast::Integer { value } => {
            if let TestType::Integer (expected) = expected {
                if value == expected {
                    return true;
                }
                eprintln!("integer literal not {}, got={}", expected, value);
                return false;
            }
            else {
                panic!("mismatched type");
            }
        },
        Ast::StringLiteral  { value } => {
            if let TestType::String (expected) = expected {
                if *value == expected {
                    return true;
                }
                eprintln!("string literal not {}, got={}", expected, value);
                return false;
            }
            else {
                panic!("mismatched type");
            }
        },
        Ast::Boolean { value } => {
            if let TestType::Boolean (expected) = expected {
                if value == expected {
                    return true;
                }
                eprintln!("boolean literal not {}, got={}", expected, value);
                return false;
            }
            else {
                panic!("mismatched type");
            }
        },
        _ => panic!("invalid Ast kind: {}", literal.clone().kind()),
    }
}

#[test]
fn test_parse_integer_literal_expression() {
    let tests = [("5;", TestType::Integer(5)),
                 ("10;", TestType::Integer(10))
    ];
    
    for test in tests.iter() {
        let lexer = Lexer::new(test.0.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        parser.check_parser_errors();
        
        if let Ast::Program { ref mut statements } = program.unwrap() {
            assert_eq!(statements.len(), 1);
            for statement in statements.iter() {
                if let Ast::ExpressionStatement { ref expression } = (**statement).clone() {
                    assert!(test_literal((**expression).clone(), test.1.clone()));
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
    let tests = [("let a = 0;", "a", TestType::Integer(0)),
                 ("let foo = 100;", "foo", TestType::Integer(100))
    ];

    for test in tests.iter() {
        let lexer = Lexer::new(test.0.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        parser.check_parser_errors();

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
                    assert!(test_literal((**value).clone(), test.2.clone()));
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
    let tests = [("return 0;", TestType::Integer(0)),
                 ("return 10;", TestType::Integer(10))
    ];

    for test in tests.iter() {
        let lexer = Lexer::new(test.0.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        parser.check_parser_errors();
        
        if let Ast::Program { ref mut statements } = program.unwrap() {
            assert_eq!(statements.len(), 1);
            for statement in statements.iter() {
                if let Ast::ReturnStatement { ref return_value } = **statement {
                    assert!(test_literal((**return_value).clone(), test.1.clone()));
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
fn test_parse_boolean_literal() {
    let tests = [("true;", TestType::Boolean(true)),
                 ("false;", TestType::Boolean(false))
    ];

    for test in tests.iter() {
        let lexer = Lexer::new(test.0.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        parser.check_parser_errors();
        
        if let Ast::Program { ref mut statements } = program.unwrap() {
            assert_eq!(statements.len(), 1);
            for statement in statements.iter() {
                if let Ast::ExpressionStatement { ref expression } = **statement {
                    assert!(test_literal((**expression).clone(), test.1.clone()));
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
fn test_parse_string_literal() {
    let tests = [("\"foo\";", TestType::String("foo".to_string())),
                 ("\"bar\";", TestType::String("bar".to_string()))
    ];

    for test in tests.iter() {
        let lexer = Lexer::new(test.0.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        parser.check_parser_errors();
        
        if let Ast::Program { ref mut statements } = program.unwrap() {
            assert_eq!(statements.len(), 1);
            for statement in statements.iter() {
                if let Ast::ExpressionStatement { ref expression } = **statement {
                    assert!(test_literal((**expression).clone(), test.1.clone()));
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
fn test_parse_prefix_expression() {
    let tests = [("!true;", "!", TestType::Boolean(true)),
                 ("!false;", "!" , TestType::Boolean(false)),
                 ("-1;", "-" , TestType::Integer(1)),
                 ("-10;", "-" , TestType::Integer(10)),
    ];

    for test in tests.iter() {
        let lexer = Lexer::new(test.0.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        parser.check_parser_errors();
        
        if let Ast::Program { ref mut statements } = program.unwrap() {
            assert_eq!(statements.len(), 1);
            for statement in statements.iter() {
                if let Ast::ExpressionStatement { ref expression } = **statement {
                    if let Ast::PrefixExpression { ref operator, ref right } = **expression {
                        assert_eq!(**operator, test.1.to_string());
                        assert!(test_literal((**right).clone(), test.2.clone()));
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
fn test_parse_infix_expression() {
    let tests = [("0 + 1;", TestType::Integer(0), "+", TestType::Integer(1)),
                 ("0 - 1;", TestType::Integer(0), "-", TestType::Integer(1)),
                 ("0 * 1;", TestType::Integer(0), "*", TestType::Integer(1)),
                 ("0 / 1;", TestType::Integer(0), "/", TestType::Integer(1)),
                 ("0 == 1;", TestType::Integer(0), "==", TestType::Integer(1)),
                 ("0 != 1;", TestType::Integer(0), "!=", TestType::Integer(1))

    ];

    for test in tests.iter() {
        let lexer = Lexer::new(test.0.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        parser.check_parser_errors();
        
        if let Ast::Program { ref mut statements } = program.unwrap() {
            assert_eq!(statements.len(), 1);
            for statement in statements.iter() {
                if let Ast::ExpressionStatement { ref expression } = **statement {
                    if let Ast::InfixExpression { ref left, ref operator, ref right } = **expression {
                        assert!(test_literal((**left).clone(), test.1.clone()));
                        assert_eq!(**operator, test.2.to_string());
                        assert!(test_literal((**right).clone(), test.3.clone()));
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
fn test_parse_function_expression() {
    let tests = ["fn(a,b){ return true; }"
    ];

    for test in tests.iter() {
        let lexer = Lexer::new(test.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        parser.check_parser_errors();

        assert_eq!(program.unwrap().clone().inspect(), test.to_string());
    }
}

#[test]
fn test_parse_if_expression() {
    let tests = ["if(true){ true } else{ false }",
                 "if(false){ true }"
    ];

    for test in tests.iter() {
        let lexer = Lexer::new(test.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        parser.check_parser_errors();

        assert_eq!(program.unwrap().clone().inspect(), test.to_string());
    }
}

#[test]
fn test_parse_array_literal() {
    let tests = ["[1,2,3,4]"
    ];

    for test in tests.iter() {
        let lexer = Lexer::new(test.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        parser.check_parser_errors();

        assert_eq!(program.unwrap().clone().inspect(), test.to_string());
    }
}

#[test]
fn test_parse_call_expression() {
    let tests = ["add(a,b)"
    ];

    for test in tests.iter() {
        let lexer = Lexer::new(test.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        parser.check_parser_errors();

        assert_eq!(program.unwrap().clone().inspect(), test.to_string());
    }
}

#[test]
fn test_parse_index_expression() {
    let tests = ["[a,b][0]",
                 "[a,b][(1 + 1)]"
    ];

    for test in tests.iter() {
        let lexer = Lexer::new(test.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        parser.check_parser_errors();

        assert_eq!(program.unwrap().clone().inspect(), test.to_string());
    }
}

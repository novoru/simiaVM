extern crate simiaVM;

use simiaVM::lexer::{ Lexer };
use simiaVM::object::{ Object };
use simiaVM::parser::{ Parser };
use simiaVM::eval::{ eval };

fn test_eval(input: String) -> Object {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();

    eval(program.unwrap())
}

fn test_integer_object(o: Object, expected: i64) {
    match o {
        Object::Integer { value }   => assert_eq!(value, expected),
        _                           => panic!("object is not Integer. got={}",o.kind()),
    }
}

fn test_boolean_object(o: Object, expected: bool) {
    match o {
        Object::Boolean { value }   => assert_eq!(value, expected),
        _                           => panic!("object is not Boolean. got={}",o.kind()),
    }
}

#[test]
fn test_eval_integer_expression() {
    let tests = [  
                    ("5", 5 ),
                    ("10", 10 ),
                    ("-5", -5),
                    ("-10", -10),
                    ("5 + 5 + 5 + 5 - 10", 10),
                    ("2 * 2 * 2 * 2 * 2", 32),
                    ("-50 + 100 +  -50", 0),
                    ("5 * 2 + 10", 20),
                    ("5 + 2 * 10", 25),
                    ("20 + 2 * -10", 0),
                    ("50 / 2 * 2 + 10", 60),
                    ("2 * (5 + 10)", 30),
                    ("3 * 3 * 3 + 10", 37),
                    ("3 * (3 * 3) + 10", 37),
                    ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for test in &tests {
        let evaluated = test_eval(test.0.to_string());
        test_integer_object(evaluated, test.1);
    }
                
}

#[test]
fn test_eval_boolean_expression() {
    let tests = [
                    ("true", true),
                    ("false", false),
                    ("1 < 2", true),
                    ("1 > 2", false),
                    ("1 < 1", false),
                    ("1 > 1", false),
                    ("1 == 1", true),
                    ("1 != 1", false),
                    ("1 == 2", false),
                    ("1 != 2", true),
                    ("true == true", true),
                    ("false == false", true),
                    ("true == false", false),
                    ("true != false", true),
                    ("false != true", true),
                    ("(1 < 2) == true", true),
                    ("(1 < 2) == false", false),
                    ("(1 > 2) == true", false),
                    ("(1 > 2) == false", true),
    ];

    for test in &tests {
        let evaluated = test_eval(test.0.to_string());
        test_boolean_object(evaluated, test.1);
    }
}

#[test]
fn test_bang_operator() {
    let tests = [
                    ("!true", false),
                    ("!false", true),
                    ("!5", false),
                    ("!!true", true),
                    ("!!false", false),
                    ("!!5", true),
    ];

    for test in &tests {
        let evaluated = test_eval(test.0.to_string());
        test_boolean_object(evaluated, test.1);
    }
}
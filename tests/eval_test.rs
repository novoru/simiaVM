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

#[test]
fn test_eval_integer_expression() {
    let tests = [  
                    ("5", 5 ),
                    ("10", 10 ),
                ];

    for test in &tests {
        let evaluated = test_eval(test.0.to_string());
        test_integer_object(evaluated, test.1);
    }
                
}
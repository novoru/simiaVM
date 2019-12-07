use crate::ast::{ Ast };
use crate::object::{ Object };

pub fn eval(node: Ast) -> Object {
    match node {
        Ast::Program { statements }             => eval_statements(*statements).unwrap(),
        Ast::ExpressionStatement { expression } => eval(*expression),
        Ast::Integer { value }                  => Object::Integer { value: value },
        Ast::Boolean { value }                  => Object::Boolean { value: value },
        Ast::PrefixExpression { operator, right} => {
            let _right = eval(*right);
            return eval_prefix_expression(*operator, _right).unwrap();
        },
        _                                       => Object::Null,
    }
}

pub fn eval_statements(statements: Vec<Box<Ast>>) -> Option<Object> {
    let mut result = Object::Null;

    for statement in statements {
        result = eval(*statement);
    }

    Some(result)
}

pub fn eval_prefix_expression(operator: String, right: Object) -> Option<Object> {
    match operator.as_ref() {
        "!" => Some(eval_bang_operator_expression(right)),
        "-" => Some(eval_minus_prefix_operator_expression(right)),
        _   => None,
    }
}

pub fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        Object::Boolean { value } => {
            if value {
                return Object::Boolean { value: false };
            }
            return Object::Boolean { value: true };
        },
        Object::Null    => Object::Boolean { value: true },
        _               => Object::Boolean { value: false },
    }
}

pub fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer { value }   => Object::Integer { value: -value },
        _                           => Object::Null,
    }
}
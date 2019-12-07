use crate::ast::{ Ast };
use crate::object::{ Object };

pub fn eval(node: Ast) -> Object {
    match node {
        Ast::Program { statements }                 => eval_statements(*statements).unwrap(),
        Ast::ExpressionStatement { expression }     => eval(*expression),
        Ast::Integer { value }                      => Object::Integer { value: value },
        Ast::Boolean { value }                      => Object::Boolean { value: value },
        Ast::PrefixExpression { operator, right}    => {
            let _right = eval(*right);
            return eval_prefix_expression(*operator, _right).unwrap();
        },
        Ast::InfixExpression { left, operator, right } => {
            let _left = eval(*left);
            let _right = eval(*right);
            return eval_infix_expression(*operator, _left, _right);
        },
        _   => Object::Null,
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

pub fn eval_infix_expression(operator: String, left: Object, right: Object) -> Object {
    if let Object::Integer { value: lvalue } = left {
        if let Object::Integer { value: rvalue } = right {
            return eval_integer_infix_expression(operator, lvalue, rvalue)
        }
    }

    if let Object::Boolean { value: lvalue } = left {
        if let Object::Boolean { value: rvalue } = right {
            return eval_boolean_infix_expression(operator, lvalue, rvalue)
        }
    }

    Object::Null
}

pub fn eval_integer_infix_expression(operator: String, lvalue: i64, rvalue: i64) -> Object {
    match operator.as_ref() {
        "+"     => Object::Integer { value: lvalue + rvalue },
        "-"     => Object::Integer { value: lvalue - rvalue },
        "*"     => Object::Integer { value: lvalue * rvalue },
        "/"     => Object::Integer { value: lvalue / rvalue },
        "<"     => Object::Boolean { value: lvalue < rvalue },
        ">"     => Object::Boolean { value: lvalue > rvalue },
        "=="    => Object::Boolean { value: lvalue == rvalue },
        "!="    => Object::Boolean { value: lvalue != rvalue },
        _       => Object::Null,
    }
}

pub fn eval_boolean_infix_expression(operator: String, lvalue: bool, rvalue: bool) -> Object {
    match operator.as_ref() {
        "=="    => Object::Boolean { value: lvalue == rvalue },
        "!="    => Object::Boolean { value: lvalue != rvalue },
        _       => Object::Null,
    }
}
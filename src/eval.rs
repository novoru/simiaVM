use crate::ast::{ Ast };
use crate::object::{ Object };

pub fn eval(node: Ast) -> Object {
    match node {
        Ast::Program { statements }             => eval_statements(*statements).unwrap(),
        Ast::ExpressionStatement { expression } => eval(*expression),
        Ast::Integer { value }                  => Object::Integer{ value: value },
        _ => Object::Null,
    }
}

pub fn eval_statements(statements: Vec<Box<Ast>>) -> Option<Object> {
    let mut result = Object::Null;

    for statement in statements {
        result = eval(*statement);
    }

    Some(result)
}
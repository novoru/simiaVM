#[derive(Clone)]
pub enum Ast {

    Program {
        statements: Box<Vec<Box<Ast>>>
    },

    ExpressionStatement {
        expression: Box<Ast>,
    },

    LetStatement {
        identifier: Box<Ast>,
        value: Box<Ast>,
    },

    ReturnStatement {
        return_value: Box<Ast>,
    },

    Identifier {
        value: Box<String>,
    },

    GroupedExpression {
        expression_list: Box<Vec<Box<Ast>>>,
    },

    IfExpression {
        condition: Box<Ast>,    // BlockStatement
        body: Box<Ast>,         // BlockStatement
        alternative: Box<Ast>,  // None | BlockStatement
    },

    BlockStatement {
        statements: Box<Vec<Box<Ast>>>,
    },

    FunctionLiteral {
        arguments: Box<Vec<Box<Ast>>>,  // Identifier List
        body: Box<Ast>,                 // Block Statement
    },

    ArrayLiteral {
        elements: Box<Vec<Box<Ast>>>,   // Expression List
    },

    PrefixExpression {
        operator: Box<String>,    // '!' | '-'
        right: Box<Ast>,          // Expression
    },

    InfixExpression {
        left: Box<Ast>,           // Expression
        operator: Box<String>,    // '+' | '-' | '*' | '/' | '==' | '!='
        right: Box<Ast>,          // Expression
    },

    CallExpression {
        function: Box<Ast>,             // Identifier
        arguments: Box<Vec<Box<Ast>>>,  // Expression List
    },

    IndexExpression {
        index: Box<Ast>,
    },

    Integer {
        value: i64,
    },

    Boolean {
        value: bool,
    },

    StringLiteral {
        value: Box<String>,
    },

}

impl Ast {
    pub fn kind(&self) -> String {
        match self {
            Ast::Program {..}             => "Program".to_string(),
            Ast::ExpressionStatement {..} => "ExpressionStatement".to_string(),
            Ast::LetStatement {..}        => "LetStatement".to_string(),
            Ast::ReturnStatement {..}     => "ReturnStatement".to_string(),
            Ast::Identifier {..}          => "Identifier".to_string(),
            Ast::GroupedExpression {..}   => "GroupedExpression".to_string(),
            Ast::IfExpression {..}        => "IfExpression".to_string(),
            Ast::BlockStatement {..}      => "BlockStatement".to_string(),
            Ast::FunctionLiteral {..}     => "FunctionLiteral".to_string(),
            Ast::ArrayLiteral {..}        => "ArrayLiteral".to_string(),
            Ast::PrefixExpression {..}     => "PrefixExpression".to_string(),
            Ast::InfixExpression {..}     => "InfixExpression".to_string(),
            Ast::CallExpression {..}      => "CallExpression".to_string(),
            Ast::IndexExpression {..}     => "IndexExpression".to_string(),
            Ast::Integer {..}             => "Integer".to_string(),
            Ast::Boolean {..}             => "Boolean".to_string(),
            Ast::StringLiteral {..}       => "StringLiteral".to_string(),
        }
    }
}

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

    IfExpression {
        condition: Box<Ast>,              // BlockStatement
        body: Box<Ast>,                   // BlockStatement
        alternative: Option<Box<Ast>>,  // None | BlockStatement
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
        left: Box<Ast>,
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

    pub fn inspect(&self) -> String {
        let mut string = "".to_string();
        match self {
            Ast::Program { statements } => {
                for statement in (*statements).iter() {
                    string = format!("{}{}", string, (*statement).inspect().to_string());
                }
            },
            Ast::ExpressionStatement { expression } => string = format!("{}", (*expression).inspect()), 
            Ast::LetStatement { identifier, value } => string = format!("let {} = {};", (*identifier).inspect(), (*value).inspect()), 
            Ast::ReturnStatement { return_value } => string = format!("return {};", (*return_value).inspect()),
            Ast::Identifier { value } => string = format!("{}", value),
            Ast::IfExpression { condition, body, alternative } => {
                string = format!("if({}){{ {} }}", (*condition).inspect(), (*body).inspect());
                if let Some(value) = alternative {
                    string = format!("{} else{{ {} }}", string, (*value).inspect());
                }
            },
            Ast::BlockStatement { statements } => {
                for (i, statement) in (*statements).iter().enumerate() {
                    if i == 0 {
                        string = format!("{}", (*statement).inspect());
                    }
                    else {
                        string = format!("{}{}", string, (*statement).inspect());
                    }
                }
            },
            Ast::FunctionLiteral { arguments, body} => {
                string = format!("fn(");
                for (i, argument) in arguments.iter().enumerate() {
                    if i == 0 {
                        string = format!("{}{}", string, (*argument).inspect());
                    }
                    else {
                        string = format!("{},{}", string, (*argument).inspect());
                    }
                }
                string = format!("{}){{ {} }}", string, (*body).inspect());
            },
            Ast::ArrayLiteral { elements }        => {
                string = format!("[");
                for (i, element) in elements.iter().enumerate() {
                    if i == 0 {
                        string = format!("{}{}", string, (*element).inspect());
                    }
                    else {
                        string = format!("{},{}", string, (*element).inspect());
                    }
                }
                string = format!("{}]", string);
            },
            Ast::PrefixExpression { operator, right} => string = format!("({}{})", *operator, (*right).inspect()),
            Ast::InfixExpression { left, operator, right} => string = format!("({} {} {})", (*left).inspect(), *operator, (*right).inspect()),
            Ast::CallExpression { function, arguments } => {
                string = format!("{}(", (*function).inspect());
                for (i, argument) in arguments.iter().enumerate() {
                    if i == 0 {
                        string = format!("{}{}", string, (*argument).inspect());
                    }
                    else {
                        string = format!("{},{}", string, (*argument).inspect());
                    }
                }
                string = format!("{})", string);
            },
            Ast::IndexExpression { left, index }     => string = format!("{}[{}]", (*left).inspect(), (*index).inspect()),
            Ast::Integer { value } => string = format!("{}", value),
            Ast::Boolean { value } => string = format!("{}", value),
            Ast::StringLiteral { value } => string = value.to_string(),
        }

        string
    }

    pub fn kind(&self) -> String {
        match self {
            Ast::Program {..}             => "Program".to_string(),
            Ast::ExpressionStatement {..} => "ExpressionStatement".to_string(),
            Ast::LetStatement {..}        => "LetStatement".to_string(),
            Ast::ReturnStatement {..}     => "ReturnStatement".to_string(),
            Ast::Identifier {..}          => "Identifier".to_string(),
            Ast::IfExpression {..}        => "IfExpression".to_string(),
            Ast::BlockStatement {..}      => "BlockStatement".to_string(),
            Ast::FunctionLiteral {..}     => "FunctionLiteral".to_string(),
            Ast::ArrayLiteral {..}        => "ArrayLiteral".to_string(),
            Ast::PrefixExpression {..}    => "PrefixExpression".to_string(),
            Ast::InfixExpression {..}     => "InfixExpression".to_string(),
            Ast::CallExpression {..}      => "CallExpression".to_string(),
            Ast::IndexExpression {..}     => "IndexExpression".to_string(),
            Ast::Integer {..}             => "Integer".to_string(),
            Ast::Boolean {..}             => "Boolean".to_string(),
            Ast::StringLiteral {..}       => "StringLiteral".to_string(),
        }
    }

}

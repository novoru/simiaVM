extern crate simiaVM;

use simiaVM::ast::{ Ast };

#[test]
fn test_ast() {
    let expr_stmt = Ast::ExpressionStatement{
        expression: Box::new(
            Ast::Identifier {
                value: Box::new("foo".to_string()),
            },
        ),
    };

    if let Ast::ExpressionStatement { expression } = expr_stmt.clone() {
        assert_eq!(expr_stmt.clone().kind(), "ExpressionStatement".to_string());
        if let Ast::Identifier { value } = *expression {
            assert_eq!(*value, "foo".to_string());
        }
        else {
            panic!("expr_stmt.expression not Ast::Identifier");
        }
    }
    else {
        panic!("expr_stmt not Ast::ExpressionStatement.");
    }
}

#[test]
fn test_ast_inspect() {

    // Identifier
    let ident = Ast::Identifier {
        value: Box::new("foo".to_string()),
    };

    if let Ast::Identifier { .. } = ident {
        assert_eq!(ident.inspect(), "foo".to_string())
    }

    let value = Box::new(Ast::Integer {
        value: 0,
    });

    // Let Statement
    let let_stmt = Ast::LetStatement {
        identifier: Box::new(ident.clone()),
        value: value.clone(),
    };

    if let Ast::LetStatement { .. } = let_stmt {
        assert_eq!(let_stmt.inspect(), "let foo = 0;".to_string())
    }

    // Return Statement
    let return_stmt = Ast::ReturnStatement {
        return_value: value.clone(),
    };

    if let Ast::ReturnStatement { .. } = return_stmt {
        assert_eq!(return_stmt.inspect(), "return 0;".to_string())
    }

    // Boolean
    let boolean = Ast::Boolean {
        value: true,
    };

    if let Ast::Boolean { .. } = boolean.clone() {
        assert_eq!(boolean.inspect(), "true".to_string())
    }

    // Block Statement
    let block_stmt = Ast::BlockStatement {
        statements: Box::new(vec![Box::new(let_stmt.clone()), Box::new(return_stmt.clone())]),
    };

    if let Ast::BlockStatement { .. } = block_stmt.clone() {
        assert_eq!(block_stmt.clone().inspect(), "let foo = 0;return 0;".to_string())
    }
        
    // If Expression
    let if_expr = Ast::IfExpression {
        condition: Box::new(boolean.clone()),
        body: Box::new(block_stmt.clone()),
        alternative: Some(Box::new(block_stmt.clone())),
    };

    if let Ast::IfExpression { .. } = if_expr {
        assert_eq!(if_expr.inspect(),
                   "if(true){ let foo = 0;return 0; } else{ let foo = 0;return 0; }".to_string());
    }

    // Function Literal    
    let func_lit = Ast::FunctionLiteral {
        arguments: Box::new(
            vec![Box::new(Ast::Identifier{value: Box::new("a".to_string())}),
                 Box::new(Ast::Identifier{value: Box::new("b".to_string())})]),
        body: Box::new(block_stmt.clone()),
    };

    if let Ast::FunctionLiteral { .. } = func_lit {
        assert_eq!(func_lit.inspect(), "fn(a,b){ let foo = 0;return 0; }".to_string());
    }

    // Array Literal
    let array_lit = Ast::ArrayLiteral {
        elements: Box::new(vec![Box::new(Ast::Integer{value:0}),
                                Box::new(Ast::StringLiteral{value:Box::new("bar".to_string())})]),
    };

    if let Ast::ArrayLiteral { .. } = array_lit.clone() {
        assert_eq!(array_lit.clone().inspect(), "[0,bar]".to_string());
    }

    // Prefix Expression
    let pref_expr = Ast::PrefixExpression {
        operator: Box::new("!".to_string()),
        right: Box::new(Ast::Boolean{value:true})
    };
    
    if let Ast::PrefixExpression { .. } = pref_expr.clone() {
        assert_eq!(pref_expr.clone().inspect(), "(!true)".to_string());
    }
    
    // Infix Expression
    let inf_expr = Ast::InfixExpression {
        left: Box::new(Ast::Integer{value:1}),
        operator: Box::new("+".to_string()),
        right: Box::new(Ast::Integer{value:2})
    };

    if let Ast::InfixExpression { .. } = inf_expr.clone() {
        assert_eq!(inf_expr.clone().inspect(), "(1 + 2)".to_string());
    }

    // Call Expression
    let call_expr = Ast::CallExpression {
        function: Box::new(Ast::Identifier{value:Box::new("add".to_string())}),
        arguments: Box::new(vec![Box::new(Ast::Integer{value:0}),
                                 Box::new(Ast::Integer{value:1})]),
    };

    if let Ast::CallExpression { .. } = call_expr.clone() {
        assert_eq!(call_expr.clone().inspect(), "add(0,1)".to_string());
    }

    // Index Expression
    let index_expr = Ast::IndexExpression {
        index: Box::new(Ast::Integer{value:0}),
    };

    if let Ast::IndexExpression { .. } = index_expr.clone() {
        assert_eq!(index_expr.clone().inspect(), "[0]".to_string());
    }
    
}

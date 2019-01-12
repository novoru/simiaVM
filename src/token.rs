#[derive(PartialEq,Debug,Clone)]
pub enum Token {

    Illegal(String),
    Eof(String),
    
    Identifier(String),
    Integer(String),
    String(String),

    // Operator
    Assign(String),       // '='
    Plus(String),         // '+'
    Minus(String),        // '-'
    Asterisk(String),     // '*'
    Slash(String),        // '/'
    Bang(String),         // '!'

    Eq(String),           // '=='
    NotEq(String),        // '!='
    Lt(String),           // '<'
    Gt(String),           // '>'

    // Keyword
    Let(String),          // 'let'
    Function(String),     // 'fn'
    If(String),           // 'if'
    Else(String),         // 'else'
    Return(String),       // 'return'
    True(String),         // 'true'
    False(String),        // 'false'
    
    // Delimeter
    Semicolon(String),    // ';'
    Comma(String),        // ','

    Lparen(String),       // '('
    Rparen(String),       // ')'
    Lbrace(String),       // '{'
    Rbrace(String),       // '}'
    Lbracket(String),     // '['
    Rbracket(String),     // ']'

}

impl Token {
    pub fn kind(&self) -> String {
        match self {
            Token::Illegal(_)    => "Illegal".to_string(),
            Token::Eof(_)        => "Eof".to_string(),
            Token::Identifier(_) => "Identifier".to_string(),
            Token::Integer(_)    => "Integer".to_string(),
            Token::String(_)     => "String".to_string(),
            Token::Assign(_)     => "Assign".to_string(),
            Token::Plus(_)       => "Plus".to_string(),
            Token::Minus(_)      => "Minus".to_string(),
            Token::Asterisk(_)   => "Asterisk".to_string(),
            Token::Slash(_)      => "Slash".to_string(),
            Token::Bang(_)       => "Bang".to_string(),
            Token::Eq(_)         => "Eq".to_string(),
            Token::NotEq(_)      => "NotEq".to_string(),
            Token::Lt(_)         => "Lt".to_string(),
            Token::Gt(_)         => "Gt".to_string(),
            Token::Let(_)        => "Let".to_string(),
            Token::Function(_)   => "Function".to_string(),
            Token::If(_)         => "If".to_string(),
            Token::Else(_)       => "Else".to_string(),
            Token::Return(_)     => "Return".to_string(),
            Token::True(_)       => "True".to_string(),
            Token::False(_)      => "False".to_string(),
            Token::Semicolon(_)  => "Semicolon".to_string(),
            Token::Comma(_)      => "Comma".to_string(),
            Token::Lparen(_)     => "Lparen".to_string(),
            Token::Rparen(_)     => "Rparen".to_string(),
            Token::Lbrace(_)     => "Lbrace".to_string(),
            Token::Rbrace(_)     => "Rbrace".to_string(),
            Token::Lbracket(_)   => "Lbracket".to_string(),
            Token::Rbracket(_)   => "Rbracket".to_string(),
        }
    }

    pub fn literal(&self) -> String {
        match self {
            Token::Illegal(literal)    |
            Token::Eof(literal)        |
            Token::Identifier(literal) |
            Token::Integer(literal)    |
            Token::String(literal)     |
            Token::Assign(literal)     |
            Token::Plus(literal)       |
            Token::Minus(literal)      |
            Token::Asterisk(literal)   |
            Token::Slash(literal)      |
            Token::Bang(literal)       |
            Token::Eq(literal)         |
            Token::NotEq(literal)      |
            Token::Lt(literal)         |
            Token::Gt(literal)         |
            Token::Let(literal)        |
            Token::Function(literal)   |
            Token::If(literal)         |
            Token::Else(literal)       |
            Token::Return(literal)     |
            Token::True(literal)       |
            Token::False(literal)      |
            Token::Semicolon(literal)  |
            Token::Comma(literal)      |
            Token::Lparen(literal)     |
            Token::Rparen(literal)     |
            Token::Lbrace(literal)     |
            Token::Rbrace(literal)     |
            Token::Lbracket(literal)   |
            Token::Rbracket(literal)   => literal.to_string(),
        }
    }
}

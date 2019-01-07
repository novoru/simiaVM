#[derive(Debug,Clone)]
pub enum Token {

    Illegal(String),
    Eof(String),
    
    Identifier(String),
    Integer(String),

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
            Token::Illegal(literal)    => literal.to_string(),
            Token::Eof(literal)        => literal.to_string(),
            Token::Identifier(literal) => literal.to_string(),
            Token::Integer(literal)    => literal.to_string(),
            Token::Assign(literal)     => literal.to_string(),
            Token::Plus(literal)       => literal.to_string(),
            Token::Minus(literal)      => literal.to_string(),
            Token::Asterisk(literal)   => literal.to_string(),
            Token::Slash(literal)      => literal.to_string(),
            Token::Bang(literal)       => literal.to_string(),
            Token::Eq(literal)         => literal.to_string(),
            Token::NotEq(literal)      => literal.to_string(),
            Token::Lt(literal)         => literal.to_string(),
            Token::Gt(literal)         => literal.to_string(),
            Token::Let(literal)        => literal.to_string(),
            Token::Function(literal)   => literal.to_string(),
            Token::If(literal)         => literal.to_string(),
            Token::Else(literal)       => literal.to_string(),
            Token::Semicolon(literal)  => literal.to_string(),
            Token::Comma(literal)      => literal.to_string(),
            Token::Lparen(literal)     => literal.to_string(),
            Token::Rparen(literal)     => literal.to_string(),
            Token::Lbrace(literal)     => literal.to_string(),
            Token::Rbrace(literal)     => literal.to_string(),
            Token::Lbracket(literal)   => literal.to_string(),
            Token::Rbracket(literal)   => literal.to_string(),
        }
    }
}

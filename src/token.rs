#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(f64),
    String(String),
    Identifier(String),
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    
    // Comparison operators
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    
    // Keywords
    Let,
    If,
    Else,
    While,
    Print,
    
    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    
    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}
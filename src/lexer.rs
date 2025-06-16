// src/lexer.rs
use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
        }
    }
    
    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }
    
    fn peek_next(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }
    
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        if ch == Some('\n') {
            self.line += 1;
        }
        self.position += 1;
        ch
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && self.peek_next() == Some('/') {
                // Skip line comment
                self.advance(); // Skip first '/'
                self.advance(); // Skip second '/'
                self.skip_line_comment();
            } else {
                break;
            }
        }
    }
    
    fn skip_line_comment(&mut self) {
        // Skip until end of line (we've already consumed the '//')
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }
    
    fn read_number(&mut self) -> f64 {
        let mut number = String::new();
        
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        number.parse().unwrap_or(0.0)
    }
    
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        identifier
    }
    
    fn read_string(&mut self) -> String {
        let mut string = String::new();
        
        // Skip opening quote
        self.advance();
        
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance(); // Skip closing quote
                break;
            } else if ch == '\\' {
                // Handle escape sequences
                self.advance(); // Skip backslash
                if let Some(escaped) = self.peek() {
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        _ => {
                            string.push('\\');
                            string.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }
        
        string
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        let line = self.line;
        
        match self.peek() {
            None => Token { token_type: TokenType::Eof, line },
            Some(ch) => {
                match ch {
                    '+' => {
                        self.advance();
                        Token { token_type: TokenType::Plus, line }
                    }
                    '-' => {
                        self.advance();
                        Token { token_type: TokenType::Minus, line }
                    }
                    '*' => {
                        self.advance();
                        Token { token_type: TokenType::Multiply, line }
                    }
                    '/' => {
                        self.advance();
                        // Check for comment
                        if self.peek() == Some('/') {
                            // this is a comment, skip it in whitespace handling
                            // we should never reach here because comments are handled in skip_whitespace
                            // just in case, i will still handle it because i have nothing else to do
                            self.advance(); // Skip second '/'
                            self.skip_line_comment();
                            self.next_token() // Get the next real token
                        } else {
                            Token { token_type: TokenType::Divide, line }
                        }
                    }
                    '=' => {
                        self.advance();
                        if self.peek() == Some('=') {
                            self.advance();
                            Token { token_type: TokenType::Equal, line }
                        } else {
                            Token { token_type: TokenType::Assign, line }
                        }
                    }
                    '!' => {
                        self.advance();
                        if self.peek() == Some('=') {
                            self.advance();
                            Token { token_type: TokenType::NotEqual, line }
                        } else {
                            panic!("Unexpected character '!' at line {}", line);
                        }
                    }
                    '<' => {
                        self.advance();
                        if self.peek() == Some('=') {
                            self.advance();
                            Token { token_type: TokenType::LessEqual, line }
                        } else {
                            Token { token_type: TokenType::Less, line }
                        }
                    }
                    '>' => {
                        self.advance();
                        if self.peek() == Some('=') {
                            self.advance();
                            Token { token_type: TokenType::GreaterEqual, line }
                        } else {
                            Token { token_type: TokenType::Greater, line }
                        }
                    }
                    '(' => {
                        self.advance();
                        Token { token_type: TokenType::LeftParen, line }
                    }
                    ')' => {
                        self.advance();
                        Token { token_type: TokenType::RightParen, line }
                    }
                    '{' => {
                        self.advance();
                        Token { token_type: TokenType::LeftBrace, line }
                    }
                    '}' => {
                        self.advance();
                        Token { token_type: TokenType::RightBrace, line }
                    }
                    ';' => {
                        self.advance();
                        Token { token_type: TokenType::Semicolon, line }
                    }
                    '"' => {
                        let string = self.read_string();
                        Token { token_type: TokenType::String(string), line }
                    }
                    _ if ch.is_ascii_digit() => {
                        let number = self.read_number();
                        Token { token_type: TokenType::Number(number), line }
                    }
                    _ if ch.is_alphabetic() || ch == '_' => {
                        let identifier = self.read_identifier();
                        let token_type = match identifier.as_str() {
                            "let" => TokenType::Let,
                            "if" => TokenType::If,
                            "else" => TokenType::Else,
                            "while" => TokenType::While,
                            "print" => TokenType::Print,
                            _ => TokenType::Identifier(identifier),
                        };
                        Token { token_type, line }
                    }
                    _ => panic!("Unexpected character '{}' at line {}", ch, line),
                }
            }
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token();
            let is_eof = matches!(token.token_type, TokenType::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenize_simple() {
        let mut lexer = Lexer::new("let x = 42;".to_string());
        let tokens = lexer.tokenize();
        
        assert!(matches!(tokens[0].token_type, TokenType::Let));
        assert!(matches!(tokens[1].token_type, TokenType::Identifier(_)));
        assert!(matches!(tokens[2].token_type, TokenType::Assign));
        assert!(matches!(tokens[3].token_type, TokenType::Number(42.0)));
        assert!(matches!(tokens[4].token_type, TokenType::Semicolon));
        assert!(matches!(tokens[5].token_type, TokenType::Eof));
    }
    
    #[test]
    fn test_tokenize_arithmetic() {
        let mut lexer = Lexer::new("3 + 4 * 2".to_string());
        let tokens = lexer.tokenize();
        
        assert!(matches!(tokens[0].token_type, TokenType::Number(3.0)));
        assert!(matches!(tokens[1].token_type, TokenType::Plus));
        assert!(matches!(tokens[2].token_type, TokenType::Number(4.0)));
        assert!(matches!(tokens[3].token_type, TokenType::Multiply));
        assert!(matches!(tokens[4].token_type, TokenType::Number(2.0)));
    }
    
    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("let x = 5; // This is a comment\nlet y = 10;".to_string());
        let tokens = lexer.tokenize();
        
        // Should skip the comment and tokenize normally
        assert!(matches!(tokens[0].token_type, TokenType::Let));
        assert!(matches!(tokens[1].token_type, TokenType::Identifier(_)));
        assert!(matches!(tokens[2].token_type, TokenType::Assign));
        assert!(matches!(tokens[3].token_type, TokenType::Number(5.0)));
        assert!(matches!(tokens[4].token_type, TokenType::Semicolon));
        assert!(matches!(tokens[5].token_type, TokenType::Let)); // Next line after comment
    }
    
    #[test]
    fn test_string_literals() {
        let mut lexer = Lexer::new("\"hello world\"".to_string());
        let tokens = lexer.tokenize();
        
        assert!(matches!(tokens[0].token_type, TokenType::String(_)));
        if let TokenType::String(s) = &tokens[0].token_type {
            assert_eq!(s, "hello world");
        }
    }
}
use crate::token::{Token, TokenType};
use crate::ast::{Expr, Stmt, BinaryOp, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token {
            token_type: TokenType::Eof,
            line: 0,
        })
    }
    
    fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.peek()
    }
    
    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn consume(&mut self, expected: TokenType, message: &str) -> Result<(), String> {
        if std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("{} at line {}", message, self.peek().line))
        }
    }
    
    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        
        while !matches!(self.peek().token_type, TokenType::Eof) {
            statements.push(self.statement()?);
        }
        
        Ok(statements)
    }
    
    fn statement(&mut self) -> Result<Stmt, String> {
        match &self.peek().token_type {
            TokenType::Let => self.let_statement(),
            TokenType::If => self.if_statement(),
            TokenType::While => self.while_statement(),
            TokenType::Print => self.print_statement(),
            TokenType::LeftBrace => self.block_statement(),
            TokenType::Identifier(_) => {
                // Check if it's an assignment
                let checkpoint = self.current;
                if let TokenType::Identifier(name) = &self.peek().token_type {
                    let name = name.clone();
                    self.advance();
                    if matches!(self.peek().token_type, TokenType::Assign) {
                        self.advance();
                        let value = self.expression()?;
                        self.consume(TokenType::Semicolon, "Expected ';' after assignment")?;
                        return Ok(Stmt::Assignment { name, value });
                    }
                }
                // not an assignment, revert and parse as expression
                self.current = checkpoint;
                let expr = self.expression()?;
                self.consume(TokenType::Semicolon, "Expected ';' after expression")?;
                Ok(Stmt::Expression(expr))
            }
            _ => {
                let expr = self.expression()?;
                self.consume(TokenType::Semicolon, "Expected ';' after expression")?;
                Ok(Stmt::Expression(expr))
            }
        }
    }
    
    fn let_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::Let, "Expected 'let'")?;
        
        let name = if let TokenType::Identifier(name) = &self.peek().token_type {
            let name = name.clone();
            self.advance();
            name
        } else {
            return Err(format!("Expected identifier after 'let' at line {}", self.peek().line));
        };
        
        self.consume(TokenType::Assign, "Expected '=' after variable name")?;
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after let statement")?;
        
        Ok(Stmt::Let { name, value })
    }
    
    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::If, "Expected 'if'")?;
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after if condition")?;
        
        let then_stmt = Box::new(self.statement()?);
        
        let else_stmt = if self.match_token(&TokenType::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        
        Ok(Stmt::If { condition, then_stmt, else_stmt })
    }
    
    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::While, "Expected 'while'")?;
        self.consume(TokenType::LeftParen, "Expected '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after while condition")?;
        
        let body = Box::new(self.statement()?);
        
        Ok(Stmt::While { condition, body })
    }
    
    fn print_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::Print, "Expected 'print'")?;
        self.consume(TokenType::LeftParen, "Expected '(' after 'print'")?;
        let expr = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after print expression")?;
        self.consume(TokenType::Semicolon, "Expected ';' after print statement")?;
        
        Ok(Stmt::Print(expr))
    }
    
    fn block_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftBrace, "Expected '{'")?;
        let mut statements = Vec::new();
        
        while !matches!(self.peek().token_type, TokenType::RightBrace | TokenType::Eof) {
            statements.push(self.statement()?);
        }
        
        self.consume(TokenType::RightBrace, "Expected '}' after block")?;
        Ok(Stmt::Block(statements))
    }
    
    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }
    
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        
        while matches!(self.peek().token_type, TokenType::Equal | TokenType::NotEqual) {
            let operator = match self.peek().token_type {
                TokenType::Equal => BinaryOp::Equal,
                TokenType::NotEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        
        while matches!(self.peek().token_type, TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual) {
            let operator = match self.peek().token_type {
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        
        while matches!(self.peek().token_type, TokenType::Minus | TokenType::Plus) {
            let operator = match self.peek().token_type {
                TokenType::Minus => BinaryOp::Subtract,
                TokenType::Plus => BinaryOp::Add,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        
        while matches!(self.peek().token_type, TokenType::Divide | TokenType::Multiply) {
            let operator = match self.peek().token_type {
                TokenType::Divide => BinaryOp::Divide,
                TokenType::Multiply => BinaryOp::Multiply,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expr, String> {
        if matches!(self.peek().token_type, TokenType::Minus) {
            self.advance();
            let operand = self.unary()?;
            Ok(Expr::Unary {
                operator: UnaryOp::Minus,
                operand: Box::new(operand),
            })
        } else {
            self.primary()
        }
    }
    
    fn primary(&mut self) -> Result<Expr, String> {
        match &self.peek().token_type {
            TokenType::Number(n) => {
                let n = *n;
                self.advance();
                Ok(Expr::Number(n))
            }
            TokenType::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(Expr::String(s))
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Identifier(name))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token at line {}", self.peek().line)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    
    #[test]
    fn test_parse_let_statement() {
        let mut lexer = Lexer::new("let x = 1 + 2;".to_string());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.len(), 1);
        assert!(matches!(ast[0], Stmt::Let { .. }));
    }
    
    #[test]
    fn test_parse_expression() {
        let mut lexer = Lexer::new("3 + 4 * 2;".to_string());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.len(), 1);
        assert!(matches!(ast[0], Stmt::Expression(_)));
    }
}
use std::collections::HashMap;
use crate::ast::{Expr, Stmt, BinaryOp, UnaryOp};
use crate::value::Value;

pub struct Interpreter {
    globals: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
        }
    }
    
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for stmt in statements {
            self.execute_stmt(stmt)?;
        }
        Ok(())
    }
    
    fn execute_stmt(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate_expr(expr)?;
            }
            Stmt::Let { name, value } => {
                let val = self.evaluate_expr(value)?;
                self.globals.insert(name, val);
            }
            Stmt::Assignment { name, value } => {
                let val = self.evaluate_expr(value)?;
                if self.globals.contains_key(&name) {
                    self.globals.insert(name, val);
                } else {
                    return Err(format!("Undefined variable '{}'", name));
                }
            }
            Stmt::If { condition, then_stmt, else_stmt } => {
                let condition_val = self.evaluate_expr(condition)?;
                if self.is_truthy(&condition_val) {
                    self.execute_stmt(*then_stmt)?;
                } else if let Some(else_stmt) = else_stmt {
                    self.execute_stmt(*else_stmt)?;
                }
            }
            Stmt::While { condition, body } => {
                loop {
                    let condition_val = self.evaluate_expr(condition.clone())?;
                    if !self.is_truthy(&condition_val) {
                        break;
                    }
                    self.execute_stmt((*body).clone())?;
                }
            }
            Stmt::Block(statements) => {
                for stmt in statements {
                    self.execute_stmt(stmt)?;
                }
            }
            Stmt::Print(expr) => {
                let val = self.evaluate_expr(expr)?;
                println!("{}", val);
            }
        }
        Ok(())
    }
    
    fn evaluate_expr(&self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(n)),
            Expr::String(s) => Ok(Value::String(s)),
            Expr::Identifier(name) => {
                self.globals.get(&name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable '{}'", name))
            }
            Expr::Binary { left, operator, right } => {
                let left_val = self.evaluate_expr(*left)?;
                let right_val = self.evaluate_expr(*right)?;
                self.apply_binary_op(left_val, operator, right_val)
            }
            Expr::Unary { operator, operand } => {
                let operand_val = self.evaluate_expr(*operand)?;
                self.apply_unary_op(operator, operand_val)
            }
        }
    }
    
    fn apply_binary_op(&self, left: Value, op: BinaryOp, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::Number(l + r)),
                    BinaryOp::Subtract => Ok(Value::Number(l - r)),
                    BinaryOp::Multiply => Ok(Value::Number(l * r)),
                    BinaryOp::Divide => {
                        if r == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    BinaryOp::Equal => Ok(Value::Boolean((l - r).abs() < f64::EPSILON)),
                    BinaryOp::NotEqual => Ok(Value::Boolean((l - r).abs() >= f64::EPSILON)),
                    BinaryOp::Less => Ok(Value::Boolean(l < r)),
                    BinaryOp::Greater => Ok(Value::Boolean(l > r)),
                    BinaryOp::LessEqual => Ok(Value::Boolean(l <= r)),
                    BinaryOp::GreaterEqual => Ok(Value::Boolean(l >= r)),
                }
            }
            (Value::String(l), Value::String(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(format!("{}{}", l, r))), // String concatenation
                    BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                    BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                    _ => Err("Invalid operation for strings".to_string()),
                }
            }
            // Handle string + number concatenation
            (Value::String(s), Value::Number(n)) => {
                match op {
                    BinaryOp::Add => {
                        // Convert number to string and concatenate
                        let n_str = if n.fract() == 0.0 {
                            format!("{}", n as i64) // Display as integer if no decimal part
                        } else {
                            format!("{}", n)
                        };
                        Ok(Value::String(format!("{}{}", s, n_str)))
                    }
                    _ => Err("Invalid operation for string and number".to_string()),
                }
            }
            // Handle number + string concatenation
            (Value::Number(n), Value::String(s)) => {
                match op {
                    BinaryOp::Add => {
                        // Convert number to string and concatenate
                        let n_str = if n.fract() == 0.0 {
                            format!("{}", n as i64) // Display as integer if no decimal part
                        } else {
                            format!("{}", n)
                        };
                        Ok(Value::String(format!("{}{}", n_str, s)))
                    }
                    _ => Err("Invalid operation for number and string".to_string()),
                }
            }
            _ => Err("Invalid operands for binary operation".to_string()),
        }
    }
    
    fn apply_unary_op(&self, op: UnaryOp, operand: Value) -> Result<Value, String> {
        match (op, operand) {
            (UnaryOp::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
            _ => Err("Invalid operand for unary operation".to_string()),
        }
    }
    
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
        }
    }
    
    // testing
    #[cfg(test)]
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.globals.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    
    #[test]
    fn test_interpreter_basic() {
        let mut lexer = Lexer::new("let x = 5; let y = x * 2;".to_string());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        
        interpreter.interpret(ast).unwrap();
        
        assert!(matches!(interpreter.get_variable("x"), Some(Value::Number(5.0))));
        assert!(matches!(interpreter.get_variable("y"), Some(Value::Number(10.0))));
    }
    
    #[test]
    fn test_interpreter_arithmetic() {
        let mut lexer = Lexer::new("let result = 3 + 4 * 2;".to_string());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        
        interpreter.interpret(ast).unwrap();
        
        // 3 + (4 * 2) = 11 totally works
        assert!(matches!(interpreter.get_variable("result"), Some(Value::Number(11.0))));
    }
    
    #[test]
    fn test_interpreter_comparison() {
        let mut lexer = Lexer::new("let result = 5 > 3;".to_string());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        
        interpreter.interpret(ast).unwrap();
        
        assert!(matches!(interpreter.get_variable("result"), Some(Value::Boolean(true))));
    }
}
// src/main.rs
mod lexer;
mod parser;
mod interpreter;
mod ast;
mod token;
mod value;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let program = if args.len() > 1 {
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", filename, e);
                process::exit(1);
            }
        }
    } else {

        r#"
            // Example Ferris program - you can modify this or pass a .ferris file as argument
            print("Hello, World!");
            
            let name = "Ferris";
            let greeting = "Hello, " + name + "!";
            print(greeting);
            
            let x = 10;
            let y = 20;
            let sum = x + y;
            print(sum);
            
            if (sum > 25) {
                print("Sum is greater than 25");
            } else {
                print("Sum is 25 or less");
            }
        "#.to_string()
    };
    
    run_program(program);
}

fn run_program(source: String) {
    // lexical analysis
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    // parsing
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            // interpretation
            let mut interpreter = Interpreter::new();
            match interpreter.interpret(ast) {
                Ok(()) => println!("\n🦀 Ferris program executed successfully! 🦀"),
                Err(e) => eprintln!("Runtime error: {}", e),
            }
        }
        Err(e) => eprintln!("Parse error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_full_program() {
        let program = "let x = 5; let y = x * 2;".to_string();
        
        let mut lexer = Lexer::new(program);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let mut interpreter = Interpreter::new();
        
        interpreter.interpret(ast).unwrap();
        
        // test would need access to interpreter state
        // this is just a basic integration test
    }
}
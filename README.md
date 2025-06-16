# The Ferris Programming Language 🦀

A simple, elegant programming language written in Rust and named after Ferris, the beloved Rust mascot. Ferris demonstrates the complete implementation of a programming language from lexical analysis to execution.


https://github.com/user-attachments/assets/cd44a33f-7447-49cf-a976-7fb4c2fe2a78

## Why Ferris?

Ferris is named after the Rust programming language's mascot - a friendly crab that represents the safety, speed, and reliability that Rust brings to systems programming. Like its namesake, the Ferris language aims to be approachable yet powerful.

## Features

### Language Features
- **Variables**: Variable declaration and assignment (`let x = 42;`)
- **Data Types**: Numbers (`42`, `3.14`) and Strings (`"Hello, World!"`)
- **Arithmetic Operations**: `+`, `-`, `*`, `/`
- **String Operations**: String concatenation with `+` operator
- **Comparison Operations**: `==`, `!=`, `<`, `>`, `<=`, `>=`
- **Control Flow**: `if`/`else` statements and `while` loops
- **Print Statements**: Output values to console (`print("Hello!");`)
- **Block Statements**: Group statements with `{}`
- **Comments**: Line comments with `//`
- **Escape Sequences**: Support for `\n`, `\t`, `\"`, `\\` in strings

### Technical Features
- **Lexical Analysis**: Tokenizes source code into meaningful tokens
- **Parsing**: Builds an Abstract Syntax Tree (AST) using recursive descent parsing
- **Interpretation**: Tree-walking interpreter that executes the AST directly
- **Error Handling**: Comprehensive error reporting with line numbers
- **Modular Design**: Clean separation of concerns across multiple modules

## Project Structure

```
src/
├── main.rs          # Main entry point and CLI
├── token.rs         # Token types and definitions
├── lexer.rs         # Lexical analyzer (tokenizer)
├── ast.rs           # Abstract Syntax Tree definitions
├── parser.rs        # Parser implementation
├── interpreter.rs   # Interpreter implementation
└── value.rs         # Runtime value types
```

## Example Programs

### Basic Arithmetic
```
let x = 10;
let y = 20;
let sum = x + y;
print(sum);  // Output: 30
```

### Conditionals
```ferris
let temperature = 75;

if (temperature > 80) {
    print("It's hot!");
} else {
    if (temperature > 60) {
        print("Perfect weather!");
    } else {
        print("It's cold!");
    }
}
```

### Loops
```ferris
// Countdown
let counter = 5;
while (counter > 0) {
    print("T-minus " + counter);
    counter = counter - 1;
}
print("Blast off!");

// Fibonacci sequence
let a = 0;
let b = 1;
let count = 0;
print("Fibonacci sequence:");
print(a);
print(b);

while (count < 8) {
    let next = a + b;
    print(next);
    a = b;
    b = next;
    count = count + 1;
}
```

### Comments and Documentation
```ferris
// This is a line comment
let x = 42;  // Comments can be at the end of lines

// Calculate the area of a rectangle
let width = 10;
let height = 5;
let area = width * height;
print("Area: " + area);
```

## Getting Started

### Prerequisites
- Rust 1.70 or later
- Cargo (comes with Rust)

### Installation
```bash
git clone https://github.com/yourusername/ferris-lang.git
cd ferris-lang
cargo build
```

### Running Ferris Programs
```bash
# Run the example program
cargo run

# Run a specific .ferris file
cargo run examples/hello.ferris
cargo run examples/fibonacci.ferris
cargo run examples/calculator.ferris

# Run tests
cargo test

# Run with optimizations
cargo run --release examples/factorial.ferris
```

## Creating Your First Ferris Program

1. **Create a new `.ferris` file:**
```bash
New-Item my_program.ferris
```

2. **Write your program:**
```ferris
// my_program.ferris
print("Hello from my first Ferris program!");

let name = "Developer";
let message = "Welcome to Ferris, " + name + "!";
print(message);

let numbers = 0;
while (numbers < 3) {
    print("Counting: " + numbers);
    numbers = numbers + 1;
}

print("Done! Happy coding!");
```

3. **Run your program:**
```bash
cargo run my_program.ferris
```

4. **Expected output:**
```
Hello from my first Ferris program! 
Welcome to Ferris, Developer!
Counting: 0
Counting: 1
Counting: 2
Done! Happy coding! 
🦀 Ferris program executed successfully! 🦀
```

## Writing Ferris Programs

Ferris programs use the `.ferris` file extension. Here's how to get started:

## Language Grammar

```
program        → statement* EOF

statement      → letStmt
               | ifStmt
               | whileStmt
               | printStmt
               | blockStmt
               | assignStmt
               | exprStmt

letStmt        → "let" IDENTIFIER "=" expression ";"
assignStmt     → IDENTIFIER "=" expression ";"
ifStmt         → "if" "(" expression ")" statement ("else" statement)?
whileStmt      → "while" "(" expression ")" statement
printStmt      → "print" "(" expression ")" ";"
blockStmt      → "{" statement* "}"
exprStmt       → expression ";"

expression     → equality
equality       → comparison (("==" | "!=") comparison)*
comparison     → term ((">" | ">=" | "<" | "<=") term)*
term           → factor (("-" | "+") factor)*
factor         → unary (("/" | "*") unary)*
unary          → ("-") unary | primary
primary        → NUMBER | STRING | IDENTIFIER | "(" expression ")"

// Lexical Grammar
NUMBER         → DIGIT+ ("." DIGIT+)?
STRING         → '"' (CHAR | ESCAPE)* '"'
IDENTIFIER     → ALPHA (ALPHA | DIGIT | "_")*
COMMENT        → "//" [^\n]*

ESCAPE         → "\" ("n" | "t" | "r" | "\" | '"')
ALPHA          → [a-zA-Z]
DIGIT          → [0-9]
CHAR           → Any character except '"' and '\'
```

## Architecture

### Lexer (lexer.rs)
- Converts source code into tokens
- Handles keywords, operators, literals, and identifiers
- Tracks line numbers for error reporting

### Parser (parser.rs)
- Implements recursive descent parsing
- Builds an Abstract Syntax Tree (AST)
- Handles operator precedence correctly
- Provides detailed error messages

### Interpreter (interpreter.rs)
- Tree-walking interpreter
- Manages variable scope with a global environment
- Executes statements and evaluates expressions
- Handles runtime errors gracefully

## Testing

Run the test suite:
```bash
cargo test
```

The test suite covers:
- **Lexical analysis** of various token types (numbers, strings, identifiers, operators)
- **Comment handling** and proper skipping of line comments
- **String literal parsing** with escape sequences
- **Parsing** of different statement and expression types
- **Interpreter execution** of complete programs
- **Error handling** scenarios and edge cases

### Example Test Output
```bash
$ cargo test
running 8 tests
test lexer::tests::test_comments ... ok
test lexer::tests::test_string_literals ... ok
test lexer::tests::test_tokenize_arithmetic ... ok
test lexer::tests::test_tokenize_simple ... ok
test parser::tests::test_parse_expression ... ok
test parser::tests::test_parse_let_statement ... ok
test interpreter::tests::test_interpreter_arithmetic ... ok
test interpreter::tests::test_interpreter_basic ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Future Enhancements

- [ ] **Functions and function calls** - `fn add(x, y) { return x + y; }`
- [ ] **Boolean data type** - `true`, `false`, logical operators (`&&`, `||`, `!`)
- [ ] **Arrays and indexing** - `let arr = [1, 2, 3]; print(arr[0]);`
- [ ] **For loops** - `for (let i = 0; i < 10; i = i + 1) { ... }`
- [ ] **Local variable scoping** - Block-level variable scope
- [ ] **Better error recovery** - Continue parsing after syntax errors
- [ ] **REPL (Read-Eval-Print Loop)** - Interactive mode for testing
- [ ] **Standard library functions** - Math, string manipulation, I/O
- [ ] **Import system** - `import "math"; import "strings";`
- [ ] **Bytecode compiler** - Compile to bytecode for better performance
- [ ] **Debugger integration** - Step-through debugging support
- [ ] **Package manager** - Install and manage Ferris libraries
- [ ] **Type system** - Optional static typing for better error catching
- [ ] **Pattern matching** - `match` expressions for complex conditionals

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Named after Ferris 🦀, the beloved Rust mascot created by Karen Rustad Tölva
- Inspired by "Crafting Interpreters" by Robert Nystrom
- Built with the amazing Rust programming language
- Thanks to the Rust community for excellent documentation and tools
- Special thanks to the Rust Foundation and all contributors to the Rust ecosystem

## Community

- 🦀 **Rust Community**: [rust-lang.org](https://rust-lang.org)
- 📚 **Learn Rust**: [doc.rust-lang.org/book](https://doc.rust-lang.org/book/)
- 💬 **Rust Discord**: [discord.gg/rust-lang](https://discord.gg/rust-lang)
- 🐦 **Rust Twitter**: [@rustlang](https://twitter.com/rustlang)

---

*"Fast, safe, productive - pick three. Ferris gives you all three!"* 🦀

**Happy coding with Ferris!** 🎉

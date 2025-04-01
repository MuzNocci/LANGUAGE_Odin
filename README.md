# Odin Programming Language

Odin is a programming language that can be interpreted or compiled. It is designed to be simple, fast, and efficient. This repository contains the source code for the Odin compiler and interpreter.

## Project Structure

```plaintext
odin/
├── Cargo.toml                        # Rust project configuration
├── src/
│   ├── main.rs                       # Entry point of the compiler/interpreter
│   ├── lexer/
│   │   ├── mod.rs                    # Lexer module definitions
│   │   ├── token.rs                  # Token definitions
│   │   └── lexer.rs                  # Lexical analyzer
│   ├── parser/
│   │   ├── mod.rs                    # Parser module definitions
│   │   ├── ast.rs                     # Abstract Syntax Tree (AST)
│   │   └── parser.rs                 # Syntax analyzer
│   ├── interpreter/
│   │   ├── mod.rs                    # Interpreter module definitions
│   │   ├── environment.rs            # Execution environment
│   │   ├── evaluator.rs              # Expression evaluator
│   │   └── object.rs                 # Object representation
│   ├── compiler/
│   │   ├── mod.rs                    # Compiler module definitions
│   │   ├── bytecode.rs               # Bytecode definition
│   │   └── compiler.rs               # Compiler
│   ├── vm/
│   │   ├── mod.rs                    # Virtual Machine (VM) module definitions
│   │   └── vm.rs                     # Virtual Machine
│   ├── stdlib/
│   │   ├── mod.rs                    # Standard library module definitions
│   │   ├── math.rs                   # Mathematical functions
│   │   ├── io.rs                     # Input/output functions
│   │   └── string.rs                 # String manipulation functions
│   └── repl/
│       ├── mod.rs                    # REPL module definitions
│       └── repl.rs                   # Read-Eval-Print Loop (REPL)
├── examples/                         # Example Odin programs
│   ├── hello_world.odin              # Basic "Hello World" example
│   ├── fibonacci.odin                # Fibonacci implementation
│   └── classes.odin                  # Example of classes and objects
└── tests/                            # Tests for the compiler/interpreter
    ├── lexer_tests.rs                # Tests for the lexer
    ├── parser_tests.rs               # Tests for the parser
    └── integration_tests.rs          # Integration tests
```

## Features

- **Lexical Analysis**: Tokenizes the source code.
- **Parsing**: Converts tokenized code into an Abstract Syntax Tree (AST).
- **Interpreter**: Evaluates Odin code directly.
- **Compiler**: Compiles Odin code to an intermediate representation (bytecode).
- **Virtual Machine**: Executes the compiled bytecode.
- **Standard Library**: Provides built-in utilities for math, I/O, and string handling.
- **REPL**: Allows interactive execution of Odin code.

## Getting Started

### Prerequisites
- Rust installed (latest stable version recommended)

### Build and Run
```sh
# Clone the repository
git clone https://github.com/your-repo/odin-lang.git
cd odin-lang

# Build the project
cargo build --release

# Run the interpreter
cargo run -- examples/hello_world.odin
```

### Running Tests
```sh
cargo test
```

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
mod ast;
mod lexer;
mod parser;
mod codegen;

use std::env;
use std::fs;


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { eprintln!("Por favor, forneça o arquivo a ser executado."); std::process::exit(1); };
    let file_path = &args[1];
    let source_code = fs::read_to_string(file_path).expect(&format!("Não foi possível ler o arquivo '{}'", file_path));
    let tokens = lexer::tokenize(&source_code);
    let ast = parser::parse(&tokens);

    codegen::generate_ir(&ast);
    
}
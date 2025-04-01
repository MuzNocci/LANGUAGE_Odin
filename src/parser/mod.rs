// parser/mod.rs
// Exporta os submódulos do parser

pub mod ast;
pub mod parser;



// Re-exportações para facilitar o uso
pub use self::ast::{
    Program, Statement, Expression, 
    LetStatement, ReturnStatement, ExpressionStatement, BlockStatement,
    Identifier, IntegerLiteral, BooleanLiteral, StringLiteral,
    PrefixExpression, InfixExpression, IfExpression, FunctionLiteral,
    CallExpression, ArrayLiteral, IndexExpression, HashLiteral,
    ForStatement, ClassStatement, MethodStatement, PropertyAccessExpression,
};
pub use self::parser::Parser;

// Função auxiliar para criar um parser a partir de tokens
use crate::lexer::token::Token;

pub fn new_parser(tokens: Vec<Token>) -> Parser {
    Parser::new(tokens)
}

// Função auxiliar para criar um parser diretamente de uma string de entrada
// usando o lexer para gerar os tokens
pub fn parse_from_source(input: &str) -> (Program, Vec<String>) {
    use crate::lexer::lexer::Lexer;
    
    let lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();
    
    (program, parser.errors().clone())
}
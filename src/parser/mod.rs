pub mod ast;
pub mod parser;

use crate::lexer::Token;
use self::ast::Program;
use self::parser::Parser;



/// Função auxiliar para analisar sintaticamente tokens em uma AST
pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}
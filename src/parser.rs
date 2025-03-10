use crate::lexer::Token;
use crate::ast::{ASTNode, Expr};



pub fn parse(tokens: &Vec<Token>) -> ASTNode {

    let mut ast = ASTNode::new();
    let mut current_token_index = 0;

    while current_token_index < tokens.len() {
        match &tokens[current_token_index] {
            Token::Identifier(id) => {
                ast.add_node(Expr::Identifier(id.clone()));
            }
            Token::Operator(op) => {
                ast.add_node(Expr::Operator(op.to_string()));
            }
            _ => {}
        }
        current_token_index += 1;
    }

    ast

}
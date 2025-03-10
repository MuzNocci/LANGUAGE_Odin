use crate::ast::{ASTNode, Expr};



pub fn analyze(ast: &ASTNode) -> Result<(), String> {

    for node in &ast.nodes {
        match node {
            Expr::Identifier(id) => {
                if id == "main" {
                    return Err("Função principal não pode ser chamada diretamente".to_string());
                }
            }
            _ => {}
        }
    }
    Ok(())
    
}
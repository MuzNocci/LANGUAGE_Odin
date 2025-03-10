use crate::ast::{ASTNode, Expr};



pub fn generate_ir(ast: &ASTNode) -> String {

    let mut ir_code = String::new();

    for node in &ast.nodes {
        match node {
            Expr::Identifier(id) => {
                ir_code.push_str(&format!("LOAD {}", id));
            }
            Expr::Operator(op) => {
                ir_code.push_str(&format!("OP {}", op));
            }
        }
    }

    ir_code
    
}

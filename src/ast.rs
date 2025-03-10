#[derive(Debug)]
pub enum Expr {
    Identifier(String),
    Operator(String),
}

#[derive(Debug)]
pub struct ASTNode {
    pub nodes: Vec<Expr>,
}

impl ASTNode {
    pub fn new() -> ASTNode {
        ASTNode { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, expr: Expr) {
        self.nodes.push(expr);
    }
}
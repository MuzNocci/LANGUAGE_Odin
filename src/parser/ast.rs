use std::fmt;
use crate::lexer::Token;

/// Representa um nó na Árvore de Sintaxe Abstrata (AST)
pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

/// Um programa é uma lista de declarações
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.string());
        }
        out
    }
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}

// ===== Statements =====

#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Function(FunctionStatement),
    Class(ClassStatement),
    Import(ImportStatement),
    Try(TryStatement),
    Pass(PassStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(s) => s.token_literal(),
            Statement::Return(s) => s.token_literal(),
            Statement::Expression(s) => s.token_literal(),
            Statement::Block(s) => s.token_literal(),
            Statement::If(s) => s.token_literal(),
            Statement::While(s) => s.token_literal(),
            Statement::For(s) => s.token_literal(),
            Statement::Function(s) => s.token_literal(),
            Statement::Class(s) => s.token_literal(),
            Statement::Import(s) => s.token_literal(),
            Statement::Try(s) => s.token_literal(),
            Statement::Pass(s) => s.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            Statement::Let(s) => s.string(),
            Statement::Return(s) => s.string(),
            Statement::Expression(s) => s.string(),
            Statement::Block(s) => s.string(),
            Statement::If(s) => s.string(),
            Statement::While(s) => s.string(),
            Statement::For(s) => s.string(),
            Statement::Function(s) => s.string(),
            Statement::Class(s) => s.string(),
            Statement::Import(s) => s.string(),
            Statement::Try(s) => s.string(),
            Statement::Pass(s) => s.string(),
        }
    }
}

// Let Statement: Declaração de variáveis
#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.string());
        out.push_str(" = ");
        
        if let Some(value) = &self.value {
            out.push_str(&value.string());
        }
        
        out.push_str("\n");
        out
    }
}

// Return Statement: Declaração de retorno
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Expression>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        
        if let Some(value) = &self.return_value {
            out.push_str(&value.string());
        }
        
        out.push_str("\n");
        out
    }
}

// Expression Statement: Uma expressão usada como statement
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Expression>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        if let Some(expr) = &self.expression {
            expr.string()
        } else {
            String::new()
        }
    }
}

// Block Statement: Um bloco de código com múltiplos statements
#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.string());
        }
        out
    }
}

// If Statement: Estrutura condicional
#[derive(Debug, Clone)]
pub struct IfStatement {
    pub token: Token,
    pub condition: Expression,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
    pub elif_branches: Vec<(Expression, BlockStatement)>,
}

impl Node for IfStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("if ");
        out.push_str(&self.condition.string());
        out.push_str(":\n");
        out.push_str(&self.consequence.string());
        
        for (condition, block) in &self.elif_branches {
            out.push_str("elif ");
            out.push_str(&condition.string());
            out.push_str(":\n");
            out.push_str(&block.string());
        }
        
        if let Some(alt) = &self.alternative {
            out.push_str("else:\n");
            out.push_str(&alt.string());
        }
        
        out
    }
}

// While Statement: Loop while
#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub token: Token,
    pub condition: Expression,
    pub body: BlockStatement,
}

impl Node for WhileStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("while ");
        out.push_str(&self.condition.string());
        out.push_str(":\n");
        out.push_str(&self.body.string());
        out
    }
}

// For Statement: Loop for
#[derive(Debug, Clone)]
pub struct ForStatement {
    pub token: Token,
    pub iterator: Expression,
    pub iterable: Expression,
    pub body: BlockStatement,
}

impl Node for ForStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("for ");
        out.push_str(&self.iterator.string());
        out.push_str(" in ");
        out.push_str(&self.iterable.string());
        out.push_str(":\n");
        out.push_str(&self.body.string());
        out
    }
}

// Function Statement: Declaração de função
#[derive(Debug, Clone)]
pub struct FunctionStatement {
    pub token: Token,
    pub name: Identifier,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Node for FunctionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("func ");
        out.push_str(&self.name.string());
        out.push_str("(");
        
        let params: Vec<String> = self.parameters.iter().map(|p| p.string()).collect();
        out.push_str(&params.join(", "));
        
        out.push_str("):\n");
        out.push_str(&self.body.string());
        out
    }
}

// Class Statement: Declaração de classe
#[derive(Debug, Clone)]
pub struct ClassStatement {
    pub token: Token,
    pub name: Identifier,
    pub parent: Option<Identifier>,
    pub methods: Vec<FunctionStatement>,
}

impl Node for ClassStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("class ");
        out.push_str(&self.name.string());
        
        if let Some(parent) = &self.parent {
            out.push_str("(");
            out.push_str(&parent.string());
            out.push_str(")");
        }
        
        out.push_str(":\n");
        
        for method in &self.methods {
            out.push_str(&method.string());
            out.push_str("\n");
        }
        
        out
    }
}

// Import Statement: Declaração de importação
#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub token: Token,
    pub module: String,
    pub items: Vec<(String, Option<String>)>, // (item, alias)
    pub is_from: bool,
}

impl Node for ImportStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        
        if self.is_from {
            out.push_str("from ");
            out.push_str(&self.module);
            out.push_str(" import ");
            
            let items: Vec<String> = self.items.iter()
                .map(|(item, alias)| {
                    if let Some(alias_name) = alias {
                        format!("{} as {}", item, alias_name)
                    } else {
                        item.clone()
                    }
                })
                .collect();
            
            out.push_str(&items.join(", "));
        } else {
            out.push_str("import ");
            out.push_str(&self.module);
            
            if let Some((item, alias)) = self.items.first() {
                if let Some(alias_name) = alias {
                    out.push_str(" as ");
                    out.push_str(alias_name);
                }
            }
        }
        
        out.push_str("\n");
        out
    }
}

// Try Statement: Estrutura try-except
#[derive(Debug, Clone)]
pub struct TryStatement {
    pub token: Token,
    pub try_block: BlockStatement,
    pub except_blocks: Vec<(Option<Expression>, BlockStatement)>,
    pub finally_block: Option<BlockStatement>,
}

impl Node for TryStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("try:\n");
        out.push_str(&self.try_block.string());
        
        for (exception, block) in &self.except_blocks {
            out.push_str("except");
            if let Some(ex) = exception {
                out.push_str(" ");
                out.push_str(&ex.string());
            }
            out.push_str(":\n");
            out.push_str(&block.string());
        }
        
        if let Some(finally) = &self.finally_block {
            out.push_str("finally:\n");
            out.push_str(&finally.string());
        }
        
        out
    }
}

// Pass Statement: Declaração pass (nenhuma operação)
#[derive(Debug, Clone)]
pub struct PassStatement {
    pub token: Token,
}

impl Node for PassStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        String::from("pass\n")
    }
}

// ===== Expressions =====

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    FloatLiteral(FloatLiteral),
    StringLiteral(StringLiteral),
    Boolean(Boolean),
    None(NoneLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    If(IfExpression),
    FunctionLiteral(FunctionLiteral),
    Call(CallExpression),
    Index(IndexExpression),
    Array(ArrayLiteral),
    Dict(DictLiteral),
    Attribute(AttributeExpression),
    Assignment(AssignmentExpression),
    Lambda(LambdaExpression),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(e) => e.token_literal(),
            Expression::IntegerLiteral(e) => e.token_literal(),
            Expression::FloatLiteral(e) => e.token_literal(),
            Expression::StringLiteral(e) => e.token_literal(),
            Expression::Boolean(e) => e.token_literal(),
            Expression::None(e) => e.token_literal(),
            Expression::Prefix(e) => e.token_literal(),
            Expression::Infix(e) => e.token_literal(),
            Expression::If(e) => e.token_literal(),
            Expression::FunctionLiteral(e) => e.token_literal(),
            Expression::Call(e) => e.token_literal(),
            Expression::Index(e) => e.token_literal(),
            Expression::Array(e) => e.token_literal(),
            Expression::Dict(e) => e.token_literal(),
            Expression::Attribute(e) => e.token_literal(),
            Expression::Assignment(e) => e.token_literal(),
            Expression::Lambda(e) => e.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            Expression::Identifier(e) => e.string(),
            Expression::IntegerLiteral(e) => e.string(),
            Expression::FloatLiteral(e) => e.string(),
            Expression::StringLiteral(e) => e.string(),
            Expression::Boolean(e) => e.string(),
            Expression::None(e) => e.string(),
            Expression::Prefix(e) => e.string(),
            Expression::Infix(e) => e.string(),
            Expression::If(e) => e.string(),
            Expression::FunctionLiteral(e) => e.string(),
            Expression::Call(e) => e.string(),
            Expression::Index(e) => e.string(),
            Expression::Array(e) => e.string(),
            Expression::Dict(e) => e.string(),
            Expression::Attribute(e) => e.string(),
            Expression::Assignment(e) => e.string(),
            Expression::Lambda(e) => e.string(),
        }
    }
}

// Identifier: Identificador (nome de variável, função, etc.)
#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

// IntegerLiteral: Literal inteiro
#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

// FloatLiteral: Literal de ponto flutuante
#[derive(Debug, Clone)]
pub struct FloatLiteral {
    pub token: Token,
    pub value: f64,
}

impl Node for FloatLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

// StringLiteral: Literal de string
#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl Node for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("\"{}\"", self.value)
    }
}

// Boolean: Valor booleano
#[derive(Debug, Clone)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

// NoneLiteral: Valor None
#[derive(Debug, Clone)]
pub struct NoneLiteral {
    pub token: Token,
}

impl Node for NoneLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        String::from("None")
    }
}

// PrefixExpression: Expressão prefixada (ex: -5, !true)
#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("({}{})", self.operator, self.right.string())
    }
}

// InfixExpression: Expressão infixada (ex: 5 + 10, a == b)
#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.string(),
            self.operator,
            self.right.string()
        )
    }
}

// IfExpression: Expressão condicional
#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("if");
        out.push_str(&self.condition.string());
        out.push_str(" ");
        out.push_str(&self.consequence.string());
        
        if let Some(alt) = &self.alternative {
            out.push_str("else ");
            out.push_str(&alt.string());
        }
        
        out
    }
}

// FunctionLiteral: Literal de função
#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str("(");
        
        let params: Vec<String> = self.parameters.iter().map(|p| p.string()).collect();
        out.push_str(&params.join(", "));
        
        out.push_str(") ");
        out.push_str(&self.body.string());
        
        out
    }
}

// CallExpression: Chamada de função
#[derive(Debug, Clone)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.function.string());
        out.push_str("(");
        
        let args: Vec<String> = self.arguments.iter().map(|a| a.string()).collect();
        out.push_str(&args.join(", "));
        
        out.push_str(")");
        out
    }
}

// IndexExpression: Acesso de índice (array[index] ou dict[key])
#[derive(Debug, Clone)]
pub struct IndexExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub index: Box<Expression>,
}

impl Node for IndexExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");
        out.push_str(&self.left.string());
        out.push_str("[");
        out.push_str(&self.index.string());
        out.push_str("])");
        out
    }
}

// ArrayLiteral: Literal de array
#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Vec<Expression>,
}

impl Node for ArrayLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("[");
        
        let elements: Vec<String> = self.elements.iter().map(|e| e.string()).collect();
        out.push_str(&elements.join(", "));
        
        out.push_str("]");
        out
    }
}

// DictLiteral: Literal de dicionário
#[derive(Debug, Clone)]
pub struct DictLiteral {
    pub token: Token,
    pub pairs: Vec<(Expression, Expression)>,
}

impl Node for DictLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("{");
        
        let pairs: Vec<String> = self.pairs
            .iter()
            .map(|(k, v)| format!("{}: {}", k.string(), v.string()))
            .collect();
        
        out.push_str(&pairs.join(", "));
        
        out.push_str("}");
        out
    }
}

// AttributeExpression: Acesso a atributo (objeto.atributo)
#[derive(Debug, Clone)]
pub struct AttributeExpression {
    pub token: Token,
    pub object: Box<Expression>,
    pub attribute: Identifier,
}

impl Node for AttributeExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("{}.{}", self.object.string(), self.attribute.string())
    }
}

// AssignmentExpression: Expressão de atribuição
#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub value: Box<Expression>,
    pub operator: String, // "=", "+=", "-=", etc.
}

impl Node for AssignmentExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("{} {} {}", self.left.string(), self.operator, self.value.string())
    }
}

// LambdaExpression: Expressão lambda
#[derive(Debug, Clone)]
pub struct LambdaExpression {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: Box<Expression>,
}

impl Node for LambdaExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("lambda ");
        
        let params: Vec<String> = self.parameters.iter().map(|p| p.string()).collect();
        out.push_str(&params.join(", "));
        
        out.push_str(": ");
        out.push_str(&self.body.string());
        
        out
    }
}

// Precedência para operadores
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Assignment,  // =, +=, -=, etc.
    Logical,     // and, or
    Equals,      // ==, !=
    LessGreater, // >, <, >=, <=
    Sum,         // +, -
    Product,     // *, /, %
    Power,       // **
    Prefix,      // -X, !X
    Call,        // myFunction(X)
    Index,       // array[index]
    Attribute,   // obj.attribute
}

impl fmt::Display for Precedence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Precedence::Lowest => write!(f, "LOWEST"),
            Precedence::Assignment => write!(f, "ASSIGNMENT"),
            Precedence::Logical => write!(f, "LOGICAL"),
            Precedence::Equals => write!(f, "EQUALS"),
            Precedence::LessGreater => write!(f, "LESSGREATER"),
            Precedence::Sum => write!(f, "SUM"),
            Precedence::Product => write!(f, "PRODUCT"),
            Precedence::Power => write!(f, "POWER"),
            Precedence::Prefix => write!(f, "PREFIX"),
            Precedence::Call => write!(f, "CALL"),
            Precedence::Index => write!(f, "INDEX"),
            Precedence::Attribute => write!(f, "ATTRIBUTE"),
        }
    }
}
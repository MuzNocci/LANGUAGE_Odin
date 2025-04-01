#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Tokens especiais
    Illegal,
    Eof,
    
    // Identificadores e literais
    Identifier,
    Int,
    Float,
    String,
    
    // Operators
    Assign,      // =
    Plus,        // +
    Minus,       // -
    Bang,        // !
    Asterisk,    // *
    Slash,       // /
    Percent,     // %
    Power,       // **
    
    Eq,          // ==
    NotEq,       // !=
    Lt,          // <
    Gt,          // >
    LtEq,        // <=
    GtEq,        // >=
    
    // Operadores de atribuição composta
    PlusAssign,  // +=
    MinusAssign, // -=
    SlashAssign, // /=
    StarAssign,  // *=
    
    // Operadores lógicos
    And,         // and
    Or,          // or
    Not,         // not
    
    // Delimitadores
    Comma,       // ,
    Semicolon,   // ;
    Colon,       // :
    Dot,         // .
    
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]
    
    // Palavras-chave
    Function,    // func
    Let,         // let
    True,        // True
    False,       // False
    If,          // if
    Else,        // else
    Elif,        // elif
    Return,      // return
    While,       // while
    For,         // for
    In,          // in
    Break,       // break
    Continue,    // continue
    None,        // None
    Class,       // class
    Import,      // import
    From,        // from
    As,          // as
    Try,         // try
    Except,      // except
    Finally,     // finally
    With,        // with
    Raise,       // raise
    Pass,        // pass
    Yield,       // yield
    Lambda,      // lambda
    
    // Indentação (específico para linguagens baseadas em indentação como Python)
    Indent,
    Dedent,
    Newline,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            literal,
            line,
            column,
        }
    }
}

/// Mapeia palavras-chave para seus tipos de token correspondentes
pub fn lookup_identifier(identifier: &str) -> TokenType {
    match identifier {
        "func" => TokenType::Function,
        "let" => TokenType::Let,
        "True" => TokenType::True,
        "False" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "elif" => TokenType::Elif,
        "return" => TokenType::Return,
        "while" => TokenType::While,
        "for" => TokenType::For,
        "in" => TokenType::In,
        "break" => TokenType::Break,
        "continue" => TokenType::Continue,
        "None" => TokenType::None,
        "class" => TokenType::Class,
        "import" => TokenType::Import,
        "from" => TokenType::From,
        "as" => TokenType::As,
        "try" => TokenType::Try,
        "except" => TokenType::Except,
        "finally" => TokenType::Finally,
        "with" => TokenType::With,
        "raise" => TokenType::Raise,
        "pass" => TokenType::Pass,
        "yield" => TokenType::Yield,
        "lambda" => TokenType::Lambda,
        "and" => TokenType::And,
        "or" => TokenType::Or,
        "not" => TokenType::Not,
        _ => TokenType::Identifier,
    }
}
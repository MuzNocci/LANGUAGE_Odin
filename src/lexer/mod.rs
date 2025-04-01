pub mod token;
pub mod lexer;

pub use self::lexer::Lexer;
pub use self::token::{Token, TokenType};



pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token()?;
        let is_eof = token.token_type == TokenType::Eof;
        
        tokens.push(token);
        
        if is_eof {
            break;
        }
    }
    
    Ok(tokens)
}
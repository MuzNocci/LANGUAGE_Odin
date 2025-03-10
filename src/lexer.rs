#[derive(Debug)]
pub enum Token {

    #[allow(dead_code)]
    Keyword(String),
    #[allow(dead_code)]
    Number(i32),
    Identifier(String),
    Operator(char),

}

pub fn tokenize(source_code: &str) -> Vec<Token> {

    let mut tokens = Vec::new();
    let mut chars = source_code.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            'a'..='z' | 'A'..='Z' => {
                let mut identifier = c.to_string();
                while let Some(&next_char) = chars.peek() {
                    if next_char.is_alphanumeric() {
                        identifier.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                // Check if the identifier is a keyword
                if is_keyword(&identifier) {
                    tokens.push(Token::Keyword(identifier));
                } else {
                    tokens.push(Token::Identifier(identifier));
                }
            }
            '0'..='9' => {
                let mut number = c.to_digit(10).unwrap() as i32;
                while let Some(&next_char) = chars.peek() {
                    if let Some(digit) = next_char.to_digit(10) {
                        number = number * 10 + digit as i32;
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number));
            }
            '+' | '-' | '*' | '/' => tokens.push(Token::Operator(c)),
            _ => (),
        }
    }

    tokens

}

fn is_keyword(identifier: &str) -> bool {

    match identifier {
        "if" | "else" | "while" | "for" | "return" => true,
        _ => false,
    }

}
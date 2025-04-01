use super::token::{Token, TokenType, lookup_identifier};



pub struct Lexer {
    input: Vec<char>,
    position: usize,      // posição atual no input (aponta para o char atual)
    read_position: usize, // posição de leitura atual (aponta para o próximo char)
    ch: char,             // caractere atual sendo examinado
    line: usize,          // linha atual
    column: usize,        // coluna atual
    indent_levels: Vec<usize>, // pilha de níveis de indentação
    current_indent: usize,     // nível de indentação atual
    pending_tokens: Vec<Token>, // tokens pendentes (usado para indentação/desindentação)
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0', // valor inicial - será atualizado por read_char()
            line: 1,
            column: 0,
            indent_levels: vec![0], // começamos no nível de indentação 0
            current_indent: 0,
            pending_tokens: Vec::new(),
        };
        
        lexer.read_char();
        lexer
    }
    
    // Lê o próximo caractere e atualiza as posições
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'; // EOF
        } else {
            self.ch = self.input[self.read_position];
        }
        
        self.position = self.read_position;
        self.read_position += 1;
        
        if self.ch == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }
    
    // Espia o próximo caractere sem avançar o ponteiro
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }
    
    // Avança até que encontre um caractere não-whitespace
    fn skip_whitespace(&mut self) -> bool {
        let mut found_newline = false;
        
        while self.ch.is_whitespace() && self.ch != '\n' {
            self.read_char();
        }
        
        // Se chegamos a uma nova linha, precisamos processar a indentação
        if self.ch == '\n' {
            found_newline = true;
            self.read_char();
            
            // Conta os espaços no início da linha
            let mut space_count = 0;
            while self.ch == ' ' || self.ch == '\t' {
                if self.ch == '\t' {
                    space_count += 4; // Assumindo que um tab equivale a 4 espaços
                } else {
                    space_count += 1;
                }
                self.read_char();
            }
            
            // Atualiza o nível de indentação atual
            self.current_indent = space_count;
        }
        
        found_newline
    }
    
    // Gera tokens de indentação/desindentação quando necessário
    fn process_indentation(&mut self) -> Option<Token> {
        if self.pending_tokens.len() > 0 {
            return Some(self.pending_tokens.remove(0));
        }
        
        // Compara o nível de indentação atual com o nível anterior
        let last_indent = *self.indent_levels.last().unwrap_or(&0);
        
        if self.current_indent > last_indent {
            // Indentação - adiciona um nível na pilha
            self.indent_levels.push(self.current_indent);
            return Some(Token::new(
                TokenType::Indent,
                String::from("INDENT"),
                self.line,
                self.column
            ));
        } else if self.current_indent < last_indent {
            // Desindentação - remove um ou mais níveis da pilha
            while self.current_indent < *self.indent_levels.last().unwrap_or(&0) {
                self.indent_levels.pop();
                self.pending_tokens.push(Token::new(
                    TokenType::Dedent,
                    String::from("DEDENT"),
                    self.line,
                    self.column
                ));
            }
            
            if self.pending_tokens.len() > 0 {
                return Some(self.pending_tokens.remove(0));
            }
        }
        
        None
    }
    
    // Retorna o próximo token
    pub fn next_token(&mut self) -> Result<Token, String> {
        // Verifica se há tokens pendentes (indentação/desindentação)
        if let Some(token) = self.process_indentation() {
            return Ok(token);
        }
        
        // Pula espaços em branco e processa novas linhas
        let found_newline = self.skip_whitespace();
        
        // Se encontramos uma nova linha, retornar NEWLINE
        if found_newline {
            // Depois de uma nova linha, verifique se há indentação/desindentação
            if let Some(token) = self.process_indentation() {
                return Ok(token);
            }
            
            // Se não houver indentação/desindentação, retorna NEWLINE
            return Ok(Token::new(
                TokenType::Newline,
                String::from("\\n"),
                self.line - 1, // A linha já foi incrementada quando processamos o \n
                self.column
            ));
        }
        
        // Processa o token com base no caractere atual
        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::Eq, String::from("=="), self.line, self.column - 1)
                } else {
                    Token::new(TokenType::Assign, String::from("="), self.line, self.column)
                }
            },
            '+' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::PlusAssign, String::from("+="), self.line, self.column - 1)
                } else {
                    Token::new(TokenType::Plus, String::from("+"), self.line, self.column)
                }
            },
            '-' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::MinusAssign, String::from("-="), self.line, self.column - 1)
                } else {
                    Token::new(TokenType::Minus, String::from("-"), self.line, self.column)
                }
            },
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::NotEq, String::from("!="), self.line, self.column - 1)
                } else {
                    Token::new(TokenType::Bang, String::from("!"), self.line, self.column)
                }
            },
            '*' => {
                if self.peek_char() == '*' {
                    self.read_char();
                    Token::new(TokenType::Power, String::from("**"), self.line, self.column - 1)
                } else if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::StarAssign, String::from("*="), self.line, self.column - 1)
                } else {
                    Token::new(TokenType::Asterisk, String::from("*"), self.line, self.column)
                }
            },
            '/' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::SlashAssign, String::from("/="), self.line, self.column - 1)
                } else if self.peek_char() == '/' {
                    // Comentário de linha única
                    self.skip_comment();
                    return self.next_token();
                } else {
                    Token::new(TokenType::Slash, String::from("/"), self.line, self.column)
                }
            },
            '%' => Token::new(TokenType::Percent, String::from("%"), self.line, self.column),
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::LtEq, String::from("<="), self.line, self.column - 1)
                } else {
                    Token::new(TokenType::Lt, String::from("<"), self.line, self.column)
                }
            },
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::GtEq, String::from(">="), self.line, self.column - 1)
                } else {
                    Token::new(TokenType::Gt, String::from(">"), self.line, self.column)
                }
            },
            ',' => Token::new(TokenType::Comma, String::from(","), self.line, self.column),
            ';' => Token::new(TokenType::Semicolon, String::from(";"), self.line, self.column),
            ':' => Token::new(TokenType::Colon, String::from(":"), self.line, self.column),
            '.' => Token::new(TokenType::Dot, String::from("."), self.line, self.column),
            '(' => Token::new(TokenType::LParen, String::from("("), self.line, self.column),
            ')' => Token::new(TokenType::RParen, String::from(")"), self.line, self.column),
            '{' => Token::new(TokenType::LBrace, String::from("{"), self.line, self.column),
            '}' => Token::new(TokenType::RBrace, String::from("}"), self.line, self.column),
            '[' => Token::new(TokenType::LBracket, String::from("["), self.line, self.column),
            ']' => Token::new(TokenType::RBracket, String::from("]"), self.line, self.column),
            '"' | '\'' => self.read_string()?,
            '#' => {
                // Comentário de linha única com #
                self.skip_comment();
                return self.next_token();
            },
            '\0' => {
                // Antes de retornar EOF, precisamos gerar DEDENTs para todos os níveis de indentação pendentes
                if self.indent_levels.len() > 1 {
                    self.current_indent = 0;
                    if let Some(token) = self.process_indentation() {
                        return Ok(token);
                    }
                }
                Token::new(TokenType::Eof, String::from(""), self.line, self.column)
            },
            _ => {
                if is_letter(self.ch) {
                    return Ok(self.read_identifier());
                } else if is_digit(self.ch) {
                    return Ok(self.read_number()?);
                } else {
                    Token::new(TokenType::Illegal, self.ch.to_string(), self.line, self.column)
                }
            }
        };
        
        self.read_char();
        Ok(token)
    }
    
    // Lê um identificador (nome de variável, função, etc.)
    fn read_identifier(&mut self) -> Token {
        let position = self.position;
        
        while is_letter(self.ch) || is_digit(self.ch) {
            self.read_char();
        }
        
        let literal: String = self.input[position..self.position].iter().collect();
        let token_type = lookup_identifier(&literal);
        
        Token::new(token_type, literal, self.line, self.column - (self.position - position))
    }
    
    // Lê um número (inteiro ou ponto flutuante)
    fn read_number(&mut self) -> Result<Token, String> {
        let position = self.position;
        let mut is_float = false;
        
        while is_digit(self.ch) {
            self.read_char();
        }
        
        // Verifica se é um número de ponto flutuante
        if self.ch == '.' && is_digit(self.peek_char()) {
            is_float = true;
            self.read_char(); // Consome o ponto
            
            while is_digit(self.ch) {
                self.read_char();
            }
        }
        
        let literal: String = self.input[position..self.position].iter().collect();
        let column = self.column - (self.position - position);
        
        // Verifica se o literal pode ser convertido para um número válido
        if is_float {
            if literal.parse::<f64>().is_err() {
                return Err(format!("Número de ponto flutuante inválido: {} na linha {} coluna {}", literal, self.line, column));
            }
            Ok(Token::new(TokenType::Float, literal, self.line, column))
        } else {
            if literal.parse::<i64>().is_err() {
                return Err(format!("Número inteiro inválido: {} na linha {} coluna {}", literal, self.line, column));
            }
            Ok(Token::new(TokenType::Int, literal, self.line, column))
        }
    }
    
    // Lê uma string (entre aspas simples ou duplas)
    fn read_string(&mut self) -> Result<Token, String> {
        let quote_type = self.ch; // ' ou "
        let start_line = self.line;
        let start_column = self.column;
        
        self.read_char(); // Consome a aspas de abertura
        let position = self.position;
        
        // Lê até encontrar a aspas correspondente de fechamento
        while self.ch != quote_type && self.ch != '\0' {
            // Se encontrarmos um escape, avance um caractere adicional
            if self.ch == '\\' && (self.peek_char() == quote_type || self.peek_char() == '\\') {
                self.read_char();
            }
            self.read_char();
        }
        
        // Verifica se a string foi fechada corretamente
        if self.ch != quote_type {
            return Err(format!("String não fechada iniciada na linha {} coluna {}", start_line, start_column));
        }
        
        let literal: String = self.input[position..self.position].iter().collect();
        Ok(Token::new(TokenType::String, literal, start_line, start_column))
    }
    
    // Pula comentários (de # até o final da linha)
    fn skip_comment(&mut self) {
        while self.ch != '\n' && self.ch != '\0' {
            self.read_char();
        }
    }
}

// Funções auxiliares para verificar tipos de caracteres
fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}
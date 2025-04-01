use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::{
    BlockStatement, Expression, ExpressionStatement, Identifier, IfExpression, InfixExpression,
    LetStatement, PrefixExpression, Program, ReturnStatement, Statement, StringLiteral,
    IntegerLiteral, BooleanLiteral, FunctionLiteral, CallExpression, ArrayLiteral,
    IndexExpression, HashLiteral, ForStatement, ClassStatement, MethodStatement, 
    PropertyAccessExpression,
};
use std::collections::HashMap;



type PrefixParseFn = fn(&mut Parser) -> Option<Expression>;
type InfixParseFn = fn(&mut Parser, Expression) -> Option<Expression>;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
    Index,       // array[index]
    Member,      // obj.property
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            position: 0,
            current_token: Token::new(TokenType::Illegal, "".to_string()),
            peek_token: Token::new(TokenType::Illegal, "".to_string()),
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        // Register prefix parse functions
        parser.register_prefix(TokenType::Identifier, Parser::parse_identifier);
        parser.register_prefix(TokenType::Integer, Parser::parse_integer_literal);
        parser.register_prefix(TokenType::String, Parser::parse_string_literal);
        parser.register_prefix(TokenType::True, Parser::parse_boolean_literal);
        parser.register_prefix(TokenType::False, Parser::parse_boolean_literal);
        parser.register_prefix(TokenType::Bang, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::Minus, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::LParen, Parser::parse_grouped_expression);
        parser.register_prefix(TokenType::If, Parser::parse_if_expression);
        parser.register_prefix(TokenType::Function, Parser::parse_function_literal);
        parser.register_prefix(TokenType::LBracket, Parser::parse_array_literal);
        parser.register_prefix(TokenType::LBrace, Parser::parse_hash_literal);

        // Register infix parse functions
        parser.register_infix(TokenType::Plus, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Minus, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Slash, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Asterisk, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Eq, Parser::parse_infix_expression);
        parser.register_infix(TokenType::NotEq, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Lt, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Gt, Parser::parse_infix_expression);
        parser.register_infix(TokenType::LParen, Parser::parse_call_expression);
        parser.register_infix(TokenType::LBracket, Parser::parse_index_expression);
        parser.register_infix(TokenType::Dot, Parser::parse_property_access_expression);

        // Read two tokens to initialize current_token and peek_token
        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        
        if self.position < self.tokens.len() {
            self.peek_token = self.tokens[self.position].clone();
            self.position += 1;
        } else {
            self.peek_token = Token::new(TokenType::EOF, "".to_string());
        }
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token_type, self.peek_token.token_type
        );
        self.errors.push(msg);
    }

    fn register_prefix(&mut self, token_type: TokenType, function: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, function);
    }

    fn register_infix(&mut self, token_type: TokenType, function: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, function);
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: Vec::new() };

        while !self.current_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::For => self.parse_for_statement(),
            TokenType::Class => self.parse_class_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let let_token = self.current_token.clone();

        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }

        let name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        self.next_token();

        let value = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let(LetStatement {
            token: let_token,
            name,
            value,
        }))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let return_token = self.current_token.clone();

        self.next_token();

        let return_value = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Return(ReturnStatement {
            token: return_token,
            return_value,
        }))
    }

    fn parse_for_statement(&mut self) -> Option<Statement> {
        let for_token = self.current_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        self.next_token();
        let initialization = match self.parse_statement() {
            Some(stmt) => stmt,
            None => return None,
        };

        if !self.expect_peek(TokenType::Semicolon) {
            return None;
        }

        self.next_token();
        let condition = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_peek(TokenType::Semicolon) {
            return None;
        }

        self.next_token();
        let update = match self.parse_statement() {
            Some(stmt) => stmt,
            None => return None,
        };

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Statement::For(ForStatement {
            token: for_token,
            initialization: Box::new(initialization),
            condition,
            update: Box::new(update),
            body,
        }))
    }

    fn parse_class_statement(&mut self) -> Option<Statement> {
        let class_token = self.current_token.clone();

        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }

        let name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        // Optional inheritance
        let parent = if self.peek_token_is(TokenType::Extends) {
            self.next_token(); // consume 'extends'
            
            if !self.expect_peek(TokenType::Identifier) {
                return None;
            }
            
            Some(Identifier {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone(),
            })
        } else {
            None
        };

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let mut methods = Vec::new();
        self.next_token();

        while !self.current_token_is(TokenType::RBrace) && !self.current_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_method_statement() {
                if let Statement::Method(method) = stmt {
                    methods.push(method);
                }
            }
            self.next_token();
        }

        Some(Statement::Class(ClassStatement {
            token: class_token,
            name,
            parent,
            methods,
        }))
    }

    fn parse_method_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();

        // Must be an identifier for the method name
        if !self.current_token_is(TokenType::Identifier) {
            return None;
        }

        let name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        let parameters = self.parse_function_parameters();

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Statement::Method(MethodStatement {
            token,
            name,
            parameters,
            body,
        }))
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let stmt = ExpressionStatement {
            token: self.current_token.clone(),
            expression: match self.parse_expression(Precedence::Lowest) {
                Some(expr) => expr,
                None => return None,
            },
        };

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Expression(stmt))
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let token = self.current_token.clone();
        let mut statements = Vec::new();

        self.next_token();

        while !self.current_token_is(TokenType::RBrace) && !self.current_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        BlockStatement { token, statements }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // Try to get a prefix parsing function for the current token
        let prefix = match self.prefix_parse_fns.get(&self.current_token.token_type) {
            Some(prefix_fn) => *prefix_fn,
            None => {
                self.no_prefix_parse_fn_error(self.current_token.token_type.clone());
                return None;
            }
        };

        let mut left_exp = prefix(self);

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            // Check if there's an infix parsing function for the peek token
            let infix = match self.infix_parse_fns.get(&self.peek_token.token_type) {
                Some(infix_fn) => *infix_fn,
                None => return left_exp,
            };

            self.next_token();

            left_exp = match left_exp {
                Some(left) => infix(self, left),
                None => return None,
            };
        }

        left_exp
    }

    fn no_prefix_parse_fn_error(&mut self, token_type: TokenType) {
        let msg = format!("no prefix parse function for {:?} found", token_type);
        self.errors.push(msg);
    }

    fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Expression::Identifier(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();
        
        let value = match self.current_token.literal.parse::<i64>() {
            Ok(value) => value,
            Err(_) => {
                let msg = format!("could not parse {} as integer", self.current_token.literal);
                self.errors.push(msg);
                return None;
            }
        };

        Some(Expression::IntegerLiteral(IntegerLiteral { token, value }))
    }

    fn parse_string_literal(&mut self) -> Option<Expression> {
        Some(Expression::StringLiteral(StringLiteral {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        }))
    }

    fn parse_boolean_literal(&mut self) -> Option<Expression> {
        Some(Expression::BooleanLiteral(BooleanLiteral {
            token: self.current_token.clone(),
            value: self.current_token_is(TokenType::True),
        }))
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();
        let operator = self.current_token.literal.clone();

        self.next_token();

        let right = match self.parse_expression(Precedence::Prefix) {
            Some(expr) => expr,
            None => return None,
        };

        Some(Expression::PrefixExpression(PrefixExpression {
            token,
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let token = self.current_token.clone();
        let operator = self.current_token.literal.clone();

        let precedence = self.current_precedence();
        self.next_token();

        let right = match self.parse_expression(precedence) {
            Some(expr) => expr,
            None => return None,
        };

        Some(Expression::InfixExpression(InfixExpression {
            token,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let exp = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        exp
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        self.next_token();
        let condition = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let consequence = self.parse_block_statement();

        let alternative = if self.peek_token_is(TokenType::Else) {
            self.next_token();

            if !self.expect_peek(TokenType::LBrace) {
                return None;
            }

            Some(self.parse_block_statement())
        } else {
            None
        };

        Some(Expression::IfExpression(IfExpression {
            token,
            condition: Box::new(condition),
            consequence,
            alternative,
        }))
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        let parameters = self.parse_function_parameters();

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::FunctionLiteral(FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    fn parse_function_parameters(&mut self) -> Vec<Identifier> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return identifiers;
        }

        self.next_token();

        identifiers.push(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        });

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            identifiers.push(Identifier {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone(),
            });
        }

        if !self.expect_peek(TokenType::RParen) {
            return Vec::new();
        }

        identifiers
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        Some(Expression::CallExpression(CallExpression {
            token: self.current_token.clone(),
            function: Box::new(function),
            arguments: self.parse_expression_list(TokenType::RParen),
        }))
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        Some(Expression::ArrayLiteral(ArrayLiteral {
            token: self.current_token.clone(),
            elements: self.parse_expression_list(TokenType::RBracket),
        }))
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Vec<Expression> {
        let mut list = Vec::new();

        if self.peek_token_is(end) {
            self.next_token();
            return list;
        }

        self.next_token();
        if let Some(expr) = self.parse_expression(Precedence::Lowest) {
            list.push(expr);
        }

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            if let Some(expr) = self.parse_expression(Precedence::Lowest) {
                list.push(expr);
            }
        }

        if !self.expect_peek(end) {
            return Vec::new();
        }

        list
    }

    fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
        let token = self.current_token.clone();
        
        self.next_token();
        let index = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if !self.expect_peek(TokenType::RBracket) {
            return None;
        }

        Some(Expression::IndexExpression(IndexExpression {
            token,
            left: Box::new(left),
            index: Box::new(index),
        }))
    }

    fn parse_hash_literal(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();
        let mut pairs = HashMap::new();

        while !self.peek_token_is(TokenType::RBrace) {
            self.next_token();
            let key = match self.parse_expression(Precedence::Lowest) {
                Some(expr) => expr,
                None => return None,
            };

            if !self.expect_peek(TokenType::Colon) {
                return None;
            }

            self.next_token();
            let value = match self.parse_expression(Precedence::Lowest) {
                Some(expr) => expr,
                None => return None,
            };

            pairs.insert(key, value);

            if !self.peek_token_is(TokenType::RBrace) && !self.expect_peek(TokenType::Comma) {
                return None;
            }
        }

        if !self.expect_peek(TokenType::RBrace) {
            return None;
        }

        Some(Expression::HashLiteral(HashLiteral { token, pairs }))
    }

    fn parse_property_access_expression(&mut self, object: Expression) -> Option<Expression> {
        let token = self.current_token.clone();
        
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }
        
        let property = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        Some(Expression::PropertyAccess(PropertyAccessExpression {
            token,
            object: Box::new(object),
            property,
        }))
    }

    fn current_precedence(&self) -> Precedence {
        Self::token_precedence(&self.current_token.token_type)
    }

    fn peek_precedence(&self) -> Precedence {
        Self::token_precedence(&self.peek_token.token_type)
    }

    fn token_precedence(token_type: &TokenType) -> Precedence {
        match token_type {
            TokenType::Eq | TokenType::NotEq => Precedence::Equals,
            TokenType::Lt | TokenType::Gt => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Slash | TokenType::Asterisk => Precedence::Product,
            TokenType::LParen => Precedence::Call,
            TokenType::LBracket => Precedence::Index,
            TokenType::Dot => Precedence::Member,
            _ => Precedence::Lowest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lexer::Lexer;

    #[test]
    fn test_let_statements() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer.tokenize());
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 3);

        let expected_identifiers = vec!["x", "y", "foobar"];

        for (i, identifier) in expected_identifiers.iter().enumerate() {
            match &program.statements[i] {
                Statement::Let(let_stmt) => {
                    assert_eq!(let_stmt.name.value, *identifier);
                }
                _ => panic!("Expected let statement"),
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let input = "
            return 5;
            return 10;
            return 993322;
        ";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer.tokenize());
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 3);

        for stmt in program.statements {
            match stmt {
                Statement::Return(_) => {}
                _ => panic!("Expected return statement"),
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer.tokenize());
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::Expression(expr_stmt) => {
                match &expr_stmt.expression {
                    Expression::Identifier(ident) => {
                        assert_eq!(ident.value, "foobar");
                    }
                    _ => panic!("Expression is not an identifier"),
                }
            }
            _ => panic!("Statement is not an expression statement"),
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer.tokenize());
        let program = parser.parse_program();

        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            Statement::Expression(expr_stmt) => {
                match &expr_stmt.expression {
                    Expression::IntegerLiteral(int_lit) => {
                        assert_eq!(int_lit.value, 5);
                    }
                    _ => panic!("Expression is not an integer literal"),
                }
            }
            _ => panic!("Statement is not an expression statement"),
        }
    }

    fn check_parser_errors(parser: &Parser) {
        let errors = parser.errors();
        if errors.is_empty() {
            return;
        }

        eprintln!("Parser has {} errors", errors.len());
        for error in errors {
            eprintln!("Parser error: {}", error);
        }
        panic!("Parser had errors");
    }
}
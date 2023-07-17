use std::collections::HashMap;

use crate::error::Error;
use crate::token::Token;
use crate::token_type::{TokenType, self};


pub struct Scanner<'a> {
    pub source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
    error: &'a mut Error
}


impl<'a> Scanner<'a> {
    pub fn new(source: String, error: &'a mut Error) -> Self {
        let keywords = HashMap::from([
            ("and".to_string(), TokenType::And),
            ("class".to_string(), TokenType::Class),
            ("else".to_string(), TokenType::Else),
            ("false".to_string(), TokenType::False),
            ("for".to_string(), TokenType::For),
            ("if".to_string(), TokenType::If),
            ("nil".to_string(), TokenType::Nil),
            ("or".to_string(), TokenType::Or),
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return),
            ("super".to_string(), TokenType::Super),
            ("this".to_string(), TokenType::This),
            ("true".to_string(), TokenType::True),
            ("var".to_string(), TokenType::Var),
            ("while".to_string(), TokenType::While)
        ]);
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: keywords,
            error: error
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(
            Token::new(
                TokenType::EOF,
                "".to_string(),
                "".to_string(),
                self.line,
            )
        );
        self.tokens.to_vec()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            },
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of a line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            ' ' | '\r' | '\t' => {}, // Ignore whitespace.
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.error.error(self.line, "Unexpected character.");
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false }
        if self.source.chars().nth(self.current).unwrap() != expected { return false }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0' }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0' }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, "".to_string());
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: String) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(
            Token::new(
                token_type,
                text,
                literal,
                self.line,
            )
        );
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1 }
            self.advance();
        }

        if self.is_at_end() {
            self.error.error(self.line, "Unterminated string.");
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_literal(TokenType::String, value);
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) { self.advance(); }

        // Look for fractional part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) { self.advance(); }
        }

        let value = self.source[self.start..self.current].to_string();
        self.add_token_literal(TokenType::Number, value);
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) { self.advance(); }

        let text = self.source[self.start..self.current].to_string();
        match self.keywords.get(&text) {
            Some(token_type) => self.add_token(token_type.clone()),
            None => self.add_token(TokenType::Identifier),
        }
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') ||
            (c >= 'A' && c <= 'Z') ||
            c == '_';
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }
}
use crate::lexer::Token;
use crate::lexer::TokenType;

pub struct Scanner {
    pub file: Vec<char>,
    pub start: usize,
    pub current: usize,
    pub line: u32,
}

impl Scanner {
    pub fn scan_token(&mut self) -> Token {
        self.skip_white_space();
        self.start = self.current;
        let c = self.advance();

        if self.is_alpha(c) {
            return self.keyword();
        }

        if self.is_digit(c) {
            return self.number();
        }

        match c {
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),

            _ => self.error_token(String::from("Unexpected Character"))
        }
    }

    fn keyword(&mut self) -> Token {
        while self.is_alpha(self.peek()) {
            self.advance();
        }

        match self.file[self.start..self.current] {
            // Temporary Hack To Get The Types To Match
            ['i','m','p','o','r','t'] => self.make_token(TokenType::Import),
            ['p','l','a','y'] => self.make_token(TokenType::Play),
            ['t','u','n','e'] => self.make_token(TokenType::Tune),

            _ => self.make_token(TokenType::Identifier),
        }
    }

    fn number(&mut self) -> Token {
        while self.is_digit(self.peek()) || self.peek() == '.' {
            self.advance();
        }

        self.make_token(TokenType::Number)
    }

    fn skip_white_space(&mut self) {
        loop {
            let c = self.peek();

            match c {
                ' ' => { self.advance(); },
                '\r' => { self.advance(); },
                '\t' => { self.advance(); },
                '\n' => { self.line += 1; self.advance(); },
                _ => { return; }
            };
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.file[self.current - 1]
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.file[self.current]
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.file.len() || self.file[self.current] == '\0'
    }

    pub fn make_token(&self, ttype: TokenType) -> Token {
        Token {
            ttype,
            literal: self.file[self.start..self.current].iter().collect(),
            line: self.line,
        }
    }

    fn error_token(&self, msg: String) -> Token {
        Token {
            ttype: TokenType::Error,
            literal: msg,
            line: self.line,
        }
    }
}

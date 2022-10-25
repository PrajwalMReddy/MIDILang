use crate::error::ErrorHandler;

pub struct Scanner {
    pub file: Vec<char>,
    pub start: usize,
    pub current: usize,
    pub line: u32,
    pub errors: ErrorHandler,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
    pub line: u32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TokenType {
    Import, Tune, Var, Play, // Keywords
    Identifier, Number, // Data Tokens
    Equal, LeftBrace, RightBrace, Semicolon, // Punctuation
    Eof, Error, // Special Tokens
}

fn init_scanner(file: &String, errors: ErrorHandler) -> Scanner {
    Scanner {
        file: file.chars().collect(),
        start: 0,
        current: 0,
        line: 1,
        errors,
    }
}

impl Scanner {
    pub fn scan_token(&mut self) -> Token {
        self.skip_white_space();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        if self.is_alpha(c) {
            return self.keyword();
        }

        if self.is_digit(c) {
            return self.number();
        }

        match c {
            '=' => self.make_token(TokenType::Equal),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),

            _ => self.error_token(String::from("Unexpected Character"))
        }
    }

    fn keyword(&mut self) -> Token {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }

        match self.file[self.start..self.current] {
            // TODO Temporary Hack To Get The Types To Match
            ['i','m','p','o','r','t'] => self.make_token(TokenType::Import),
            ['p','l','a','y'] => self.make_token(TokenType::Play),
            ['t','u','n','e'] => self.make_token(TokenType::Tune),
            ['v','a','r'] => self.make_token(TokenType::Var),

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

                '/' => {
                    if self.peek() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    }
                },

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

    fn error_token(&mut self, msg: String) -> Token {
        self.errors.add_error(String::from("Lexer Error"), msg.clone(), self.line);

        Token {
            ttype: TokenType::Error,
            literal: msg,
            line: self.line,
        }
    }
}

pub fn lex(file: String, errors: ErrorHandler) -> (Vec<Token>, ErrorHandler) {
    let mut scanner = init_scanner(&file, errors);
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        tokens.push(scanner.scan_token());

        if tokens[tokens.len() - 1].ttype == TokenType::Eof {
            break;
        }
    }

    (tokens, scanner.errors)
}

use crate::scanner::Scanner;

pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
    pub line: u32,
}

pub enum TokenType {
    Play, Tune, Import, // Keywords
    Identifier, Number, // Data Tokens
    LeftBrace, RightBrace, Semicolon, // Punctuation
    Eof, Error, // Special Tokens
}

fn init_scanner(file: &String) -> Scanner {
    Scanner {
        file: file.chars().collect(),
        start: 0,
        current: 0,
        line: 0,
    }
}

pub fn lex(file: String) -> Vec<Token> {
    let mut scanner = init_scanner(&file);
    let mut tokens: Vec<Token> = Vec::new();

    while !scanner.is_at_end() {
        tokens.push(scanner.scan_token());
    }

    tokens.push(scanner.make_token(TokenType::Eof));
    tokens
}

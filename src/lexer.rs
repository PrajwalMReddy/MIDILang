pub struct Token {
    ttype: TokenType,
    literal: String,
    line: u32,
}

pub enum TokenType {
    TkPlay, TkTune, TkImport, // Keywords
    TkIdentifier, TkNumber, // Data Tokens
    TkLeftBrace, TkRightBrace, TkSemicolon, // Punctuation
}

struct Scanner {
    file: String,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
}

fn init_scanner(file: &String) -> Scanner {
    Scanner {
        file: String::from(file),
        start: 0,
        current: 0,
        line: 0,
    }
}

pub fn lex(file: String) -> Vec<Token> {
    let scanner = init_scanner(&file);
    let tokens: Vec<Token> = Vec::new();

    tokens
}

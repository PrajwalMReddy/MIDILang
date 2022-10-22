use crate::lexer::Token;
use crate::lexer::TokenType;

struct Parser {
    tokens: Vec<Token>,
    statements: Vec<PlayStmt>,
    current: usize,
}

pub struct PlayStmt {
    pub token: Token,
    pub note: Token,
    pub duration: Token,
    pub velocity: Token,
}

fn init_parser(tokens: Vec<Token>) -> Parser {
    Parser {
        tokens,
        statements: Vec::new(),
        current: 0,
    }
}

impl Parser {
    fn parse(&mut self) {
        while self.peek().ttype != TokenType::Eof {
            let play_stmt = self.play_statement();
            self.statements.push(play_stmt);
        }
    }

    fn play_statement(&mut self) -> PlayStmt {
        let token = self.advance();
        let note = self.advance();
        let duration = self.advance();
        let velocity = self.advance();

        self.advance(); // Advance Past The Semicolon

        PlayStmt {
            token,
            note,
            duration,
            velocity,
        }
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token {
        self.current = self.current + 1;
        self.tokens[self.current - 1].clone()
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<PlayStmt> {
    let mut parser = init_parser(tokens);
    parser.parse();

    parser.statements
}

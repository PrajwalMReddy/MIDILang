use crate::lexer::Token;

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

    fn advance(&mut self) -> Token {
        self.current = self.current + 1;
        self.tokens[self.current - 1].clone()
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<PlayStmt> {
    let mut parser = init_parser(tokens);

    let play_stmt = parser.play_statement();
    parser.statements.push(play_stmt);

    parser.statements
}

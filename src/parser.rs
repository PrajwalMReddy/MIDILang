use crate::lexer::Token;

struct Parser {
    tokens: Vec<Token>,
    statements: Vec<PlayStmt>,
    current: u32,
}

pub struct PlayStmt {
    token: Token,
    note: Token,
    duration: Token,
}

fn init_parser(tokens: Vec<Token>) -> Parser {
    Parser {
        tokens,
        statements: vec![],
        current: 0,
    }
}

impl Parser {
    fn play_statement(parser: &mut Parser) {
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<PlayStmt> {
    let mut parser = init_parser(tokens);
    Parser::play_statement(&mut parser);
    return parser.statements;
}

use crate::error::ErrorHandler;
use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::ast::{Program, Stmt, DeclStmt, ActStmt, VarStmt, PlayStmt};

struct Parser {
    tokens: Vec<Token>,
    program: Program,
    current: usize,
    errors: ErrorHandler,
}

fn init_parser(tokens: Vec<Token>, errors: ErrorHandler) -> Parser {
    Parser {
        tokens,
        program: Program {
            statements: Stmt {
                declaration_statements: Vec::new(),
                action_statements: Vec::new(),
            }
        },
        current: 0,
        errors,
    }
}

impl Parser {
    fn parse(&mut self) {
        while self.peek().ttype != TokenType::Eof {
            self.statement();
        }
    }

    fn statement(&mut self) {
        if self.peek().ttype == TokenType::Var {
            let var_stmt = self.variable_statement();
            self.program.statements.declaration_statements.push(var_stmt);
        } else if self.peek().ttype == TokenType::Play {
            let play_stmt = self.play_statement();
            self.program.statements.action_statements.push(play_stmt);
        }
    }

    fn variable_statement(&mut self) -> DeclStmt {
        let token = self.advance();
        let identifier = self.advance();
        self.advance(); // Advance Past The Equals Sign
        let value = self.advance();
        self.advance(); // Advance Past The Semicolon

        DeclStmt::VariableStatement(
            VarStmt {
                token,
                identifier,
                value,
            }
        )
    }

    fn play_statement(&mut self) -> ActStmt {
        let token = self.advance();
        let note = self.advance();
        let duration = self.advance();
        let velocity = self.advance();
        self.advance(); // Advance Past The Semicolon

        ActStmt::PlayStatement(
            PlayStmt {
                token,
                note,
                duration,
                velocity,
            }
        )
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token {
        self.current = self.current + 1;
        self.tokens[self.current - 1].clone()
    }

    fn new_error(&mut self, msg: &str, line: u32) {
        self.errors.add_error(String::from("Parser Error"), String::from(msg), line);
    }
}

pub fn parse(tokens: Vec<Token>, errors: ErrorHandler) -> (Program, ErrorHandler) {
    let mut parser = init_parser(tokens, errors);
    parser.parse();

    (parser.program, parser.errors)
}

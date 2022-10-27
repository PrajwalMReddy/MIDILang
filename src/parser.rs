use crate::error::ErrorHandler;
use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::ast::{Program, Stmt, DeclStmt, ActStmt, VarStmt, PlayStmt, LoopStmt};

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
        let (decl_stmt, act_stmt) = self.statement(TokenType::Eof);
        self.program.statements.declaration_statements = decl_stmt;
        self.program.statements.action_statements = act_stmt;
    }

    fn statement(&mut self, until: TokenType) -> (Vec<DeclStmt>, Vec<ActStmt>) {
        let mut decl_stmt: Vec<DeclStmt> = Vec::new();
        let mut act_stmt: Vec<ActStmt> = Vec::new();

        while self.peek().ttype != until
        {
            if self.peek().ttype == TokenType::Var {
                decl_stmt.push(self.variable_statement());
            } else if self.peek().ttype == TokenType::Loop {
                act_stmt.push(self.loop_statement());
            } else if self.peek().ttype == TokenType::Play {
                act_stmt.push(self.play_statement());
            }
        }

        (decl_stmt, act_stmt)
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

    fn loop_statement(&mut self) -> ActStmt {
        let token = self.advance();
        self.advance(); // Advance Past The Colon
        let iterations = self.advance();
        self.advance(); // Advance Past The Opening Left Brace
        let (decl_stmt, act_stmt) = self.statement(TokenType::RightBrace);
        self.advance(); // Advance Past The Closing Right Brace

        ActStmt::LoopStatement(
            LoopStmt {
                token,
                iterations,
                declaration_statements: decl_stmt,
                action_statements: act_stmt,
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

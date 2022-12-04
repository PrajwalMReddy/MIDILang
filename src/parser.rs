use crate::error::ErrorHandler;
use crate::lexer::Token;
use crate::lexer::TokenType;
use crate::ast::{Program, Stmt, DeclStmt, ActStmt, TuneStmt, VarStmt, PlayStmt, LoopStmt, PlayTuneStmt, AssgnStmt, Expression, BinExpr};

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

        while !self.check_next(until.clone()) && !self.check_next(TokenType::Eof) {
            if self.check_next(TokenType::Tune) {
                decl_stmt.push(self.tune_statement());
            } else if self.check_next(TokenType::Var) {
                decl_stmt.push(self.variable_statement());
            } else if self.check_next(TokenType::Loop) {
                act_stmt.push(self.loop_statement());
            } else if self.check_next(TokenType::Play) {
                act_stmt.push(self.play_statement());
            } else if self.check_next(TokenType::Identifier) {
                act_stmt.push(self.assignment_statement());
            }
        }

        (decl_stmt, act_stmt)
    }

    fn tune_statement(&mut self) -> DeclStmt {
        let token = self.advance(); // Capture The tune Token

        let identifier = self.advance();
        if identifier.ttype != TokenType::Identifier {
            self.new_error("Tune Names Must Be Identifiers", identifier.line);
        }

        let mut parameters: Vec<Token> = Vec::new();

        if self.check_next(TokenType::Colon) {
            self.advance(); // Advance Past The Colon

            while !self.check_next(TokenType::LeftBrace) && !self.check_next(TokenType::Eof) {
                parameters.push(self.advance());
            }
        }

        if !self.check_next(TokenType::LeftBrace) {
            self.new_error("An Opening Brace Was Expected After The Tune Identifier", identifier.line);
        } else {
            self.advance(); // Advance Past The Opening Left Brace
        }

        let (decl_stmt, act_stmt) = self.statement(TokenType::RightBrace);

        if !self.check_next(TokenType::RightBrace) {
            let line = self.peek().line;
            self.new_error("A Closing Brace Was Expected After The Tune Block", line);
        } else {
            self.advance(); // Advance Past The Closing Right Brace
        }

        DeclStmt::TuneStatement(
            TuneStmt {
                token,
                identifier,
                parameters,
                declaration_statements: decl_stmt,
                action_statements: act_stmt,
            }
        )
    }

    fn variable_statement(&mut self) -> DeclStmt {
        let token = self.advance(); // Capture The var Token

        let identifier = self.advance();
        if identifier.ttype != TokenType::Identifier {
            self.new_error("Variable Names Must Be Identifiers", identifier.line);
        }

        if !self.check_next(TokenType::Equal) {
            self.new_error("An Equals Sign Was Expected After The Variable Name", identifier.line);
        } else {
            self.advance(); // Advance Past The Equals Sign
        }

        let value = self.expression();

        if !self.check_next(TokenType::Semicolon) {
            self.new_error("A Semicolon Was Expected After The Variable Declaration", token.line);
        } else {
            self.advance(); // Advance Past The Semicolon
        }

        DeclStmt::VariableStatement(
            VarStmt {
                token,
                identifier,
                value,
            }
        )
    }

    fn loop_statement(&mut self) -> ActStmt {
        let token = self.advance(); // Capture The loop Token

        if !self.check_next(TokenType::Colon) {
            self.new_error("A Colon Was Expected After The Loop Keyword", token.line);
        } else {
            self.advance(); // Advance Past The Colon
        }

        let iterations = self.expression();

        if !self.check_next(TokenType::LeftBrace) {
            self.new_error("An Opening Brace Was Expected After The Iterations Expression", token.line);
        } else {
            self.advance(); // Advance Past The Opening Left Brace
        }

        let (decl_stmt, act_stmt) = self.statement(TokenType::RightBrace);

        if !self.check_next(TokenType::RightBrace) {
            let line = self.peek().line;
            self.new_error("A Closing Brace Was Expected After The Loop Block", line);
        } else {
            self.advance(); // Advance Past The Closing Right Brace
        }

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
        if self.peek_next().ttype == TokenType::Colon {
            self.play_note_stmt()
        } else {
            self.play_tune_stmt()
        }
    }

    fn play_note_stmt(&mut self) -> ActStmt {
        let token = self.advance(); // Capture The play Token

        if !self.check_next(TokenType::Colon) {
            self.new_error("A Colon Was Expected After The Play Keyword", token.line);
        } else {
            self.advance(); // Advance Past The Colon
        }

        let note = self.expression();
        let duration = self.expression();
        let velocity = self.expression();

        if !self.check_next(TokenType::Semicolon) {
            self.new_error("A Semicolon Was Expected After The Note, Duration, And Velocity Expressions", token.line);
        } else {
            self.advance(); // Advance Past The Semicolon
        }

        ActStmt::PlayStatement(
            PlayStmt {
                token,
                note,
                duration,
                velocity,
            }
        )
    }

    fn play_tune_stmt(&mut self) -> ActStmt {
        let token = self.advance(); // Capture The play Token
        let identifier = self.advance();

        let mut arguments: Vec<Expression> = Vec::new();

        if self.check_next(TokenType::Colon) {
            self.advance(); // Advance Past The Colon

            while !self.check_next(TokenType::Semicolon) && !self.check_next(TokenType::Eof) {
                arguments.push(self.expression());
            }
        }

        if !self.check_next(TokenType::Semicolon) {
            self.new_error("A Semicolon Was Expected After The Tune Arguments", token.line);
        } else {
            self.advance(); // Advance Past The Semicolon
        }

        ActStmt::PlayTuneStatement(
            PlayTuneStmt {
                token,
                tune: identifier,
                arguments,
            }
        )
    }

    fn assignment_statement(&mut self) -> ActStmt {
        let identifier = self.advance();

        if !self.check_next(TokenType::Equal) {
            self.new_error("An Equals Sign Was Expected After The Identifier Keyword", identifier.line);
        } else {
            self.advance(); // Advance Past The Equals Sign
        }

        let value = self.expression();

        if !self.check_next(TokenType::Semicolon) {
            self.new_error("A Semicolon Was Expected After The Assignment Value", identifier.line);
        } else {
            self.advance(); // Advance Past The Semicolon
        }

        ActStmt::AssignmentStatement(
            AssgnStmt {
                identifier,
                value,
            }
        )
    }

    fn expression(&mut self) -> Expression {
        let lvalue = self.advance().clone();

        if self.check_next(TokenType::Plus) || self.check_next(TokenType::Minus) || self.check_next(TokenType::Multiply) || self.check_next(TokenType::Divide) {
            let op = self.advance();
            let rvalue = self.advance();

            Expression::BinaryExpression(
                BinExpr {
                    lvalue,
                    operator: op,
                    rvalue,
                }
            )
        } else if lvalue.ttype == TokenType::Identifier {
            return Expression::Identifier(lvalue);
        } else {
            return Expression::Number(lvalue);
        }
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn peek_next(&mut self) -> Token {
        self.tokens[self.current + 1].clone()
    }

    fn check_next(&mut self, ttoken: TokenType) -> bool {
        return if self.peek().ttype == ttoken {
            true
        } else {
            false
        }
    }

    fn advance(&mut self) -> Token {
        self.current = self.current + 1;
        self.tokens[self.current - 1].clone()
    }

    fn new_error(&mut self, msg: &str, line: u32) {
        self.errors.add_error(String::from("Parser Error"), String::from(msg), line);

        // Prevents Ghost Errors And Parsing Breakdowns
        self.errors.display_if_has_errors();
    }
}

pub fn parse(tokens: Vec<Token>, errors: ErrorHandler) -> (Program, ErrorHandler) {
    let mut parser = init_parser(tokens, errors);
    parser.parse();

    (parser.program, parser.errors)
}

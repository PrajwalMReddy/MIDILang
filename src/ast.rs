use crate::lexer::Token;

pub struct Program {
    pub statements: Statement,
}

pub struct Statement {
    pub variable_statements: Vec<VarStmt>,
    pub action_statements: ActionStatement,
}

pub struct ActionStatement {
    pub play_statements: Vec<PlayStmt>,
}

pub struct VarStmt {
    pub token: Token,
    pub identifier: Token,
    pub value: Token,
}

pub struct PlayStmt {
    pub token: Token,
    pub note: Token,
    pub duration: Token,
    pub velocity: Token,
}

use crate::lexer::Token;

pub struct Program {
    pub statements: Stmt,
}

#[derive(Clone)]
pub struct Stmt {
    pub declaration_statements: Vec<DeclStmt>,
    pub action_statements: Vec<ActStmt>,
}

#[derive(Clone)]
pub enum DeclStmt {
    VariableStatement(VarStmt),
}

#[derive(Clone)]
pub struct VarStmt {
    pub token: Token,
    pub identifier: Token,
    pub value: Token,
}

#[derive(Clone)]
pub enum ActStmt {
    LoopStatement(LoopStmt),
    PlayStatement(PlayStmt),
}

#[derive(Clone)]
pub struct LoopStmt {
    pub token: Token,
    pub iterations: Token,
    pub declaration_statements: Vec<DeclStmt>,
    pub action_statements: Vec<ActStmt>,
}

#[derive(Clone)]
pub struct PlayStmt {
    pub token: Token,
    pub note: Token,
    pub duration: Token,
    pub velocity: Token,
}

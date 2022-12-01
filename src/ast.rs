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
    TuneStatement(TuneStmt),
    VariableStatement(VarStmt),
}

#[derive(Clone)]
pub struct TuneStmt {
    pub token: Token,
    pub identifier: Token,
    pub parameters: Vec<Token>,
    pub declaration_statements: Vec<DeclStmt>,
    pub action_statements: Vec<ActStmt>,
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
    PlayTuneStatement(PlayTuneStmt),
    AssignmentStatement(AssgnStmt),
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

#[derive(Clone)]
pub struct PlayTuneStmt {
    pub token: Token,
    pub tune: Token,
    pub arguments: Vec<Token>,
}

#[derive(Clone)]
pub struct AssgnStmt {
    pub identifier: Token,
    pub value: Token,
}

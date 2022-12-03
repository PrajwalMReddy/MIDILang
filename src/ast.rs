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
    pub value: Expression,
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
    pub iterations: Expression,
    pub declaration_statements: Vec<DeclStmt>,
    pub action_statements: Vec<ActStmt>,
}

#[derive(Clone)]
pub struct PlayStmt {
    pub token: Token,
    pub note: Expression,
    pub duration: Expression,
    pub velocity: Expression,
}

#[derive(Clone)]
pub struct PlayTuneStmt {
    pub token: Token,
    pub tune: Token,
    pub arguments: Vec<Expression>,
}

#[derive(Clone)]
pub struct AssgnStmt {
    pub identifier: Token,
    pub value: Expression,
}

#[derive(Clone)]
pub enum Expression {
    BinaryExpression(BinExpr),
    Identifier(Token),
    Number(Token),
}

#[derive(Clone)]
pub struct BinExpr {
    pub lvalue: Token,
    pub operator: Token,
    pub rvalue: Token,
}

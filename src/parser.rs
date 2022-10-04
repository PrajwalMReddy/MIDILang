use crate::lexer::Token;

pub enum CompilerResult {
    CrSuccess,
    CrFailure(String),
}

pub fn parse(tokens: Vec<Token>) -> CompilerResult {

    CompilerResult::CrSuccess
}

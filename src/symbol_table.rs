use std::collections::HashMap;
use crate::ast::TuneStmt;
use crate::lexer::Token;

pub struct SymbolTable {
    pub tunes: HashMap<String, TuneStmt>,
    pub variables: HashMap<String, u32>,
}

impl SymbolTable {
    // Core Tune Functions

    pub fn add_tune(&mut self, identifier: Token, tune_stmt: TuneStmt) -> bool {
        if self.tunes.contains_key(identifier.literal.as_str()) {
            return false;
        }

        self.tunes.insert(identifier.literal, tune_stmt);
        return true;
    }

    pub fn get_tune(&mut self, identifier: Token) -> Option<&TuneStmt> {
        self.tunes.get(identifier.literal.as_str())
    }

    pub fn drop_tune(&mut self, identifier: Token) {
        self.tunes.remove(identifier.literal.as_str());
    }

    // Core Variable Functions

    pub fn add_variable(&mut self, identifier: Token, value: u32) -> bool {
        if self.variables.contains_key(identifier.literal.as_str()) {
            return false;
        }

        self.variables.insert(identifier.literal, value);
        return true;
    }

    pub fn get_variable(&mut self, identifier: Token) -> Option<&u32> {
        self.variables.get(identifier.literal.as_str())
    }

    pub fn reassign_variable(&mut self, identifier: Token, value: u32) -> bool {
        if !self.variables.contains_key(identifier.literal.as_str()) {
            return false;
        }

        self.variables.insert(identifier.literal, value);
        return true;
    }

    pub fn drop_variable(&mut self, identifier: Token) {
        self.variables.remove(identifier.literal.as_str());
    }

    // Utility Functions

    pub fn has_tune(&mut self, identifier: Token) -> bool {
        return if self.tunes.contains_key(identifier.literal.as_str()) {
            true
        } else {
            false
        }
    }

    pub fn has_variable(&mut self, identifier: Token) -> bool {
        return if self.variables.contains_key(identifier.literal.as_str()) {
            true
        } else {
            false
        }
    }
}

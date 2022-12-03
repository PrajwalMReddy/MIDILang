use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use crate::error::ErrorHandler;
use crate::ast::{ActStmt, AssgnStmt, BinExpr, DeclStmt, Expression, LoopStmt, PlayStmt, PlayTuneStmt, Program, TuneStmt, VarStmt};
use crate::lexer::{Token, TokenType};
use crate::symbol_table::SymbolTable;

struct Compiler {
    file_bytes: Vec<u8>,
    file_length: u32,
    program: Program,
    symbol_table: SymbolTable,
    errors: ErrorHandler,
}

fn init_compiler(program: Program, errors: ErrorHandler) -> Compiler {
    Compiler {
        file_bytes: Vec::new(),
        file_length: 4, // Size Of End Track Event
        program,
        symbol_table: SymbolTable {
            tunes: HashMap::new(),
            variables: HashMap::new(),
        },
        errors,
    }
}

impl Compiler {
    fn header_chunk(&mut self) {
        let mut header: Vec<u8> = vec![
            /*-----Header-Data----//-------Value-|-Description--------*/

            0x4d, 0x54, 0x68, 0x64, // MThd | ASCII Header Chunk Type
            0x00, 0x00, 0x00, 0x06, // 6 | 32 Bit Header Size
            0x00, 0x00, // 0 | 16 Bit File Format | Single Track
            0x00, 0x01, // 1 | Number Of Track Chunks
            0x00, // 0 | Divisions In Terms Of Ticks Per Quarter Note
            0x62, // 98 | 98 Ticks Per Quarter Note
        ];

        self.file_bytes.append(&mut header);
    }

    fn track_chunk(&mut self) {
        let mut header: Vec<u8> = vec![
            /*-----Track-Data----//-------Value-|-Description--------*/

            0x4d, 0x54, 0x72, 0x6b, // MTrk | ASCII Track Chunk Type
            0x00, 0x00, 0x00, 0x00, // To Be Overwritten Later | Track Length
        ];

        let mut end_of_track: Vec<u8> = vec![
            0x00, 0xff, 0x2f, 0x00, // End Of Track Event
        ];

        self.file_bytes.append(&mut header);
        self.compile();
        self.file_bytes.append(&mut end_of_track);
        self.clean_up();
    }

    fn compile(&mut self) {
        self.decl_stmt();
        self.act_stmt();
    }

    fn clean_up(&mut self) {
        /* Overwriting The Track Length Bytes */ {
            let length_offset = 18;
            let track_length = self.file_length;
            let tlb: [u8; 4] = track_length.to_be_bytes();

            self.file_bytes[length_offset + 0] = tlb[0];
            self.file_bytes[length_offset + 1] = tlb[1];
            self.file_bytes[length_offset + 2] = tlb[2];
            self.file_bytes[length_offset + 3] = tlb[3];
        }
    }

    fn decl_stmt(&mut self) {
        let declaration_statements = &self.program.statements.declaration_statements.clone();

        for decl_stmt in declaration_statements {
            match decl_stmt {
                DeclStmt::TuneStatement(tune_stmt) => { self.tune_stmt(tune_stmt); }
                DeclStmt::VariableStatement(var_stmt) => { self.var_stmt(var_stmt); }
            }
        }
    }

    fn tune_stmt(&mut self, tune_stmt: &TuneStmt) {
        self.add_tune(tune_stmt.identifier.clone(), tune_stmt.clone());
    }

    fn var_stmt(&mut self, var_stmt: &VarStmt) {
        let value: u32 = match var_stmt.value.clone() {
            Expression::BinaryExpression(bin_expr) => self.evaluate_binary(&bin_expr),
            Expression::Identifier(ident) => self.get_variable(ident),
            Expression::Number(num) => num.literal.parse().unwrap(),
        };

        self.add_variable(var_stmt.identifier.clone(), value);
    }

    fn act_stmt(&mut self) {
        let action_statements = &self.program.statements.action_statements.clone();

        for act_stmt in action_statements {
            match act_stmt {
                ActStmt::LoopStatement(loop_stmt) => { self.loop_stmt(loop_stmt); }
                ActStmt::PlayStatement(play_stmt) => { self.play_stmt(play_stmt); }
                ActStmt::PlayTuneStatement(play_tune_stmt) => { self.play_tune_stmt(play_tune_stmt); }
                ActStmt::AssignmentStatement(assgn_stmt) => { self.assgn_stmt(assgn_stmt); }
            }
        }
    }

    fn loop_stmt(&mut self, loop_stmt: &LoopStmt) {
        let iterations: u32 = match loop_stmt.iterations.clone() {
            Expression::BinaryExpression(bin_expr) => self.evaluate_binary(&bin_expr),
            Expression::Identifier(ident) => self.get_variable(ident),
            Expression::Number(num) => num.literal.parse().unwrap(),
        };

        for _ in 0..iterations {
            // Keeps Track Of All New Variables Created In The Loop Block
            let mut new_tune: Vec<Token> = Vec::new();
            let mut new_var: Vec<Token> = Vec::new();

            for decl_stmt in &loop_stmt.declaration_statements {
                match decl_stmt {
                    DeclStmt::TuneStatement(tune_stmt) => {
                        if !self.symbol_table.has_tune(tune_stmt.clone().identifier) {
                            new_tune.push(tune_stmt.clone().identifier);
                        }

                        self.tune_stmt(tune_stmt);
                    }

                    DeclStmt::VariableStatement(var_stmt) => {
                        if !self.symbol_table.has_variable(var_stmt.clone().identifier) {
                            new_var.push(var_stmt.clone().identifier);
                        }

                        self.var_stmt(var_stmt);
                    }
                }
            }

            for act_stmt in &loop_stmt.action_statements {
                match act_stmt {
                    ActStmt::LoopStatement(loop_stmt) => { self.loop_stmt(loop_stmt); }
                    ActStmt::PlayStatement(play_stmt) => { self.play_stmt(play_stmt); }
                    ActStmt::PlayTuneStatement(play_tune_stmt) => { self.play_tune_stmt(play_tune_stmt); }
                    ActStmt::AssignmentStatement(assgn_stmt) => { self.assgn_stmt(assgn_stmt); }
                }
            }

            // Drops All Declarations Declared In The Loop

            for tune in new_tune {
                self.symbol_table.drop_tune(tune);
            }

            for var in new_var {
                self.symbol_table.drop_variable(var);
            }

            // Makes Sure Errors Are Only Reported Once
            if self.errors.has_errors() {
                break;
            }
        }
    }

    fn play_stmt(&mut self, play_stmt: &PlayStmt) {
        let line = play_stmt.token.line;

        let note: u32 = match play_stmt.note.clone() {
            Expression::BinaryExpression(bin_expr) => self.evaluate_binary(&bin_expr),
            Expression::Identifier(ident) => self.get_variable(ident),
            Expression::Number(num) => num.literal.parse().unwrap(),
        }; if note > 127 {
            self.new_error("Note Value Cannot Be More Than 127", line);
        }

        let duration: u32 = match play_stmt.duration.clone() {
            Expression::BinaryExpression(bin_expr) => self.evaluate_binary(&bin_expr),
            Expression::Identifier(ident) => self.get_variable(ident),
            Expression::Number(num) => num.literal.parse().unwrap(),
        };

        let line: u32 = match play_stmt.duration.clone() {
            Expression::BinaryExpression(bin_expr) => bin_expr.operator.line,
            Expression::Identifier(ident) => ident.line,
            Expression::Number(num) => num.line,
        };

        let mut duration_u8 = self.u32_to_vle(duration, line);
        let duration_len: u32 = duration_u8.len() as u32;

        let velocity: u32 = match play_stmt.velocity.clone() {
            Expression::BinaryExpression(bin_expr) => self.evaluate_binary(&bin_expr),
            Expression::Identifier(ident) => self.get_variable(ident),
            Expression::Number(num) => num.literal.parse().unwrap(),
        }; if velocity > 127 {
            self.new_error("Velocity Value Cannot Be More Than 127", line);
        }

        let mut track_event: Vec<u8> = vec![];
        let mut note_on: Vec<u8> = vec![
            /*----Event-Data---//----Value-|-Description-----*/

            0x00, // 0 | Elapsed Time From The Previous Event
            0x9_0, // 9_0 | Note On Event
            note as u8, // Note To Be Played
            velocity as u8, // Velocity To Be Played At
        ];
        let mut note_off: Vec<u8> = vec![
            // Duration To Be Inserted Here
            0x8_0, // 8 | Note Off Event
            note as u8, // Note To Be Turned Off
            0x00, // 0 | Velocity
        ];

        track_event.append(&mut note_on);
        track_event.append(&mut duration_u8);
        track_event.append(&mut note_off);

        self.file_length += 7 + duration_len;
        self.file_bytes.append(&mut track_event);
    }

    fn play_tune_stmt(&mut self, play_tune_stmt: &PlayTuneStmt) {
        let tune = self.get_tune(play_tune_stmt.clone().tune);

        if tune.parameters.len() != play_tune_stmt.arguments.len() {
            self.new_error(format!("Tune {} Expected {} Argument(s) But Received {}", tune.token.literal, tune.parameters.len(), play_tune_stmt.arguments.len()).as_str(), play_tune_stmt.token.line);
            return;
        }

        // Keeps Track Of All New Variables Created In The Tune Block
        let mut new_tune: Vec<Token> = Vec::new();
        let mut new_var: Vec<Token> = Vec::new();

        if tune.parameters.len() != 0 {
            for i in 0..tune.parameters.len() {
                let value: u32 = match play_tune_stmt.arguments[i].clone() {
                    Expression::BinaryExpression(bin_expr) => self.evaluate_binary(&bin_expr),
                    Expression::Identifier(ident) => self.get_variable(ident),
                    Expression::Number(num) => num.literal.parse().unwrap(),
                };

                self.add_variable(tune.parameters[i].clone(), value);
                new_var.push(tune.parameters[i].clone());
            }
        }

        for decl_stmt in &tune.declaration_statements {
            match decl_stmt {
                DeclStmt::TuneStatement(tune_stmt) => {
                    if !self.symbol_table.has_tune(tune_stmt.clone().identifier) {
                        new_tune.push(tune_stmt.clone().identifier);
                    }

                    self.tune_stmt(tune_stmt);
                }

                DeclStmt::VariableStatement(var_stmt) => {
                    if !self.symbol_table.has_variable(var_stmt.clone().identifier) {
                        new_var.push(var_stmt.clone().identifier);
                    }

                    self.var_stmt(var_stmt);
                }
            }
        }

        for act_stmt in &tune.action_statements {
            match act_stmt {
                ActStmt::LoopStatement(loop_stmt) => { self.loop_stmt(loop_stmt); }
                ActStmt::PlayStatement(play_stmt) => { self.play_stmt(play_stmt); }
                ActStmt::PlayTuneStatement(play_tune_stmt) => { self.play_tune_stmt(play_tune_stmt); }
                ActStmt::AssignmentStatement(assgn_stmt) => { self.assgn_stmt(assgn_stmt); }
            }
        }

        // Drops All Declarations Declared In The Tune

        for tune in new_tune {
            self.symbol_table.drop_tune(tune);
        }

        for var in new_var {
            self.symbol_table.drop_variable(var);
        }
    }

    fn assgn_stmt(&mut self, assgn_stmt: &AssgnStmt) {
        let value: u32 = match assgn_stmt.value.clone() {
            Expression::BinaryExpression(bin_expr) => self.evaluate_binary(&bin_expr),
            Expression::Identifier(ident) => self.get_variable(ident),
            Expression::Number(num) => num.literal.parse().unwrap(),
        };

        self.reassign_variable(assgn_stmt.identifier.clone(), value);
    }

    fn evaluate_binary(&mut self, expr: &BinExpr) -> u32 {
        let num1: u32 = match expr.lvalue.ttype {
            TokenType::Number => expr.lvalue.literal.parse().unwrap(),
            TokenType::Identifier => self.get_variable(expr.clone().lvalue),

            _ => 0,
        };

        let num2: u32 = match expr.rvalue.ttype {
            TokenType::Number => expr.rvalue.literal.parse().unwrap(),
            TokenType::Identifier => self.get_variable(expr.clone().rvalue),

            _ => 0,
        };

        match expr.operator.literal.clone().as_str() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,

            _ => 0
        }
    }

    fn u32_to_vle(&mut self, mut duration: u32, line: u32) -> Vec<u8> {
        let mut sub_result: Vec<u8> = vec![];

        if duration > (0x0fffffff as u32) {
            self.new_error("Duration Value Cannot Be More Than 268435455", line);
            sub_result.push(0);
            return sub_result;
        }

        if duration <= 127 {
            sub_result.push(duration as u8);
            return sub_result;
        }

        let mut result: Vec<u8> = vec![0; 4];
        for i in (0..=3).rev() {
            result[i] = (duration & 0x7f) as u8;

            if i < 3 {
                result[i] |= 0x80;
            }

            duration >>= 7;

            if duration < 1 {
                break;
            }
        }

        let mut index = 0;
        let final_result: Vec<u8>;

        for i in 0..result.len() {
            if result[i] != 0 {
                index = i;
                break;
            }
        }

        final_result = result[index..4].to_owned();
        final_result
    }

    fn add_tune(&mut self, identifier: Token, tune_stmt: TuneStmt) {
        let result = self.symbol_table.add_tune(identifier.clone(), tune_stmt.clone());

        if !result {
            self.new_error(format!("Tune '{}' Already Exists In This Scope", identifier.literal).as_str(), identifier.line)
        }
    }

    fn get_tune(&mut self, identifier: Token) -> TuneStmt {
        return match self.symbol_table.get_tune(identifier.clone()) {
            None => {
                self.new_error(format!("Tune '{}' Does Not Exist In This Scope", identifier.clone().literal).as_str(), identifier.clone().line);
                TuneStmt {
                    token: Token {
                        ttype: TokenType::Error,
                        literal: "".to_string(),
                        line: 0,
                    },
                    identifier,
                    parameters: Vec::new(),
                    declaration_statements: Vec::new(),
                    action_statements: Vec::new(),
                }
            }

            Some(tune) => { tune.clone() }
        }
    }

    fn add_variable(&mut self, identifier: Token, value: u32) {
        let result = self.symbol_table.add_variable(identifier.clone(), value);

        if !result {
            self.new_error(format!("Variable '{}' Already Exists In This Scope", identifier.literal).as_str(), identifier.line);
        }
    }

    fn get_variable(&mut self, identifier: Token) -> u32 {
        return match self.symbol_table.get_variable(identifier.clone()) {
            None => {
                self.new_error(format!("Variable '{}' Does Not Exist In This Scope", identifier.literal).as_str(), identifier.line);
                0
            }

            Some(value) => { *value }
        }
    }

    fn reassign_variable(&mut self, identifier: Token, value: u32) {
        let result = self.symbol_table.reassign_variable(identifier.clone(), value);

        if !result {
            self.new_error(format!("Variable '{}' Does Not Exist In This Scope", identifier.literal).as_str(), identifier.line);
        }
    }

    fn new_error(&mut self, msg: &str, line: u32) {
        self.errors.add_error(String::from("Compiler Error"), String::from(msg), line);
    }
}

pub fn compile(statements: Program, path: &str, errors: ErrorHandler) -> ErrorHandler {
    let mut compiler = init_compiler(statements, errors);
    compiler.header_chunk();
    compiler.track_chunk();

    let mut file = File::create(path.to_owned() + ".mid").expect("Unable To Create .mid File");
    file.write(&mut compiler.file_bytes).expect("Could Not Generate .mid File");

    compiler.errors
}

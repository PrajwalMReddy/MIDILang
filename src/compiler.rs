use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use crate::error::ErrorHandler;
use crate::ast::{ActStmt, DeclStmt, LoopStmt, PlayStmt, Program, VarStmt};
use crate::lexer::{Token, TokenType};

struct Compiler {
    file_bytes: Vec<u8>,
    file_length: u32,
    program: Program,
    symbol_table: SymbolTable,
    errors: ErrorHandler,
}

struct SymbolTable {
    pub variables: HashMap<String, u32>,
}

impl SymbolTable {
    // Core Functions

    fn add_variable(&mut self, identifier: Token, value: Token) -> bool {
        if self.variables.contains_key(identifier.literal.as_str()) {
            return false;
        }

        self.variables.insert(identifier.literal, value.literal.parse().unwrap());
        return true;
    }

    fn get_variable(&mut self, identifier: Token) -> Option<&u32> {
        self.variables.get(identifier.literal.as_str())
    }

    fn drop_variable(&mut self, identifier: Token) {
        self.variables.remove(identifier.literal.as_str());
    }

    // Utility Functions

    fn has_variable(&mut self, identifier: Token) -> bool {
        return if self.variables.contains_key(identifier.literal.as_str()) {
            true
        } else {
            false
        }
    }
}

fn init_compiler(program: Program, errors: ErrorHandler) -> Compiler {
    Compiler {
        file_bytes: Vec::new(),
        file_length: 4,
        program,
        symbol_table: SymbolTable {
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
                DeclStmt::VariableStatement(var_stmt) => { self.var_stmt(var_stmt); }
            }
        }
    }

    fn var_stmt(&mut self, var_stmt: &VarStmt) {
        self.add_variable(var_stmt.identifier.clone(), var_stmt.value.clone());
    }

    fn act_stmt(&mut self) {
        let action_statements = &self.program.statements.action_statements.clone();

        for act_stmt in action_statements {
            match act_stmt {
                ActStmt::LoopStatement(loop_stmt) => { self.loop_stmt(loop_stmt); }
                ActStmt::PlayStatement(play_stmt) => { self.play_stmt(play_stmt); }
            }
        }
    }

    fn loop_stmt(&mut self, loop_stmt: &LoopStmt) {
        let iterations: u32 = match loop_stmt.iterations.ttype {
            TokenType::Number => loop_stmt.iterations.literal.parse().unwrap(),
            TokenType::Identifier => self.get_variable(loop_stmt.clone().iterations),

            _ => 0,
        };

        for _ in 0..iterations {
            // Keeps Track Of All New Variables Created In The Loop Block
            let mut new_var: Vec<Token> = Vec::new();

            for decl_stmt in &loop_stmt.declaration_statements {
                match decl_stmt {
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
                }
            }

            // Drops All Local Variables Declared In The Loop
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

        let note: u32 = match play_stmt.note.ttype {
            TokenType::Number => play_stmt.note.literal.parse().unwrap(),
            TokenType::Identifier => self.get_variable(play_stmt.clone().note),

            _ => 0,
        };

        let duration: u32 = match play_stmt.duration.ttype {
            TokenType::Number => play_stmt.duration.literal.parse().unwrap(),
            TokenType::Identifier => self.get_variable(play_stmt.clone().duration),

            _ => 0,
        };

        let velocity: u32 = match play_stmt.velocity.ttype {
            TokenType::Number => play_stmt.velocity.literal.parse().unwrap(),
            TokenType::Identifier => self.get_variable(play_stmt.clone().velocity),

            _ => 0,
        };

        if note > 127 {
            self.new_error("Note Value Cannot Be More Than 127", line);
        } else if duration > 127 {
            // TODO Temporary Restriction
            self.new_error("Duration Value Cannot Be More Than 127", line);
        } else if velocity > 127 {
            self.new_error("Velocity Value Cannot Be More Than 127", line);
        }

        let mut track_event: Vec<u8> = vec![
            /*----Event-Data---//----Value-|-Description-----*/

            0x00, // 0 | Elapsed Time From The Previous Event
            0x9_0, // 9_0 | Note On Event
            note as u8, // Note To Be Played
            velocity as u8, // Velocity To Be Played At

            duration as u8, // Elapsed Time From The Previous Event
            0x8_0, // 8 | Note Off Event
            note as u8, // Note To Be Turned Off
            0x00, // 0 | Velocity
        ];

        self.file_length += 8;
        self.file_bytes.append(&mut track_event);
    }

    fn add_variable(&mut self, identifier: Token, value: Token) {
        let result = self.symbol_table.add_variable(identifier.clone(), value.clone());

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

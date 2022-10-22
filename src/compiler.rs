use std::fs::File;
use std::io::Write;
use crate::error::ErrorHandler;

use crate::parser::PlayStmt;

struct Compiler {
    file_bytes: Vec<u8>,
    statements: Vec<PlayStmt>,
    errors: ErrorHandler,
}

fn init_compiler(statements: Vec<PlayStmt>, errors: ErrorHandler) -> Compiler {
    Compiler {
        file_bytes: Vec::new(),
        statements,
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
        let track_length: u32 = ((self.statements.len() * 8) + 4) as u32;
        let tlb: [u8; 4] = track_length.to_be_bytes();

        let mut header: Vec<u8> = vec![
            /*-----Track-Data----//-------Value-|-Description--------*/

            0x4d, 0x54, 0x72, 0x6b, // MTrk | ASCII Track Chunk Type
            tlb[0], tlb[1], tlb[2], tlb[3], // Track Length
        ];

        self.file_bytes.append(&mut header);
        self.play_stmt();

        let mut end_of_track: Vec<u8> = vec![
            0x00, 0xff, 0x2f, 0x00, // End Of Track Event
        ];

        self.file_bytes.append(&mut end_of_track);
    }

    fn play_stmt(&mut self) {
        let mut temp_errors: Vec<(&str, u32)> = Vec::new();

        for play_stmt in &self.statements {
            let line = play_stmt.token.line;
            let note: u32 = play_stmt.note.literal.parse().unwrap();
            let duration: u32 = play_stmt.duration.literal.parse().unwrap();
            let velocity: u32 = play_stmt.velocity.literal.parse().unwrap();

            if note > 127 {
                temp_errors.push(("Note Value Cannot Be More Than 127", line));
            } else if duration > 127 {
                // TODO Temporary Restriction
                temp_errors.push(("Duration Value Cannot Be More Than 127", line));
            } else if velocity > 127 {
                temp_errors.push(("Velocity Value Cannot Be More Than 127", line));
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

            self.file_bytes.append(&mut track_event);
        }

        for (msg, line) in temp_errors {
            self.new_error(msg, line);
        }
    }

    fn new_error(&mut self, msg: &str, line: u32) {
        self.errors.add_error(String::from("Compiler Error"), String::from(msg), line);
    }
}

pub fn compile(statements: Vec<PlayStmt>, path: &str, errors: ErrorHandler) -> ErrorHandler {
    let mut compiler = init_compiler(statements, errors);
    compiler.header_chunk();
    compiler.track_chunk();

    let mut file = File::create(path.to_owned() + ".mid").expect("Unable To Create .midi File");
    file.write(&mut compiler.file_bytes).expect("Could Not Generate .midi File");

    compiler.errors
}

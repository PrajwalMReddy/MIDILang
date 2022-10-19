use std::alloc::handle_alloc_error;
use std::fs::File;
use std::io::Write;

use crate::lexer::Token;
use crate::parser::PlayStmt;

struct Compiler {
    file_bytes: Vec<u8>,
    statements: Vec<PlayStmt>,
    current: u32,
}

fn init_compiler(statements: Vec<PlayStmt>) -> Compiler {
    Compiler {
        file_bytes: Vec::new(),
        statements,
        current: 0,
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
            0x00, 0x00, 0x00, 0x0c, // TBOL | Track Length
        ];

        self.file_bytes.append(&mut header);
        self.play_stmt();

        let mut end_of_track: Vec<u8> = vec![
            0x00, 0xff, 0x2f, 0x00, // End Of Track Event
        ];

        self.file_bytes.append(&mut end_of_track);
    }

    fn play_stmt(&mut self) {
        let mut track_event: Vec<u8> = vec![
            /*----Event-Data---//----Value-|-Description-----*/

            0x00, // 0 | Elapsed Time From The Previous Event
            0x9_0, // 9_0 | Note On Event On Channel 1 - Piano
            0x3c, // 60 | Note - Middle C
            0x40, // 64 | Velocity

            0x62, // 98 | Elapsed Time From The Previous Event
            0x8_0, // 8 | Note Off Event On Channel 1 - Piano
            0x3c, // 60 | Note - Middle C
            0x00, // 0 | Velocity
        ];

        self.file_bytes.append(&mut track_event);
    }

    fn clean_up(&mut self) {
    }
}

pub fn compile(statements: Vec<PlayStmt>, path: &str) {
    let mut compiler = init_compiler(statements);
    compiler.header_chunk();
    compiler.track_chunk();
    compiler.clean_up();

    let mut file = File::create(path.to_owned() + ".mid").expect("Unable To Create .midi File");
    file.write(&mut compiler.file_bytes).expect("Could Not Generate .midi File");
}

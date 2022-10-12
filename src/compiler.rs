use std::fs::File;
use std::io::Write;

use crate::lexer::Token;
use crate::parser::PlayStmt;

struct Compiler {
    file: File,
    file_bytes: Vec<u8>,
    statements: Vec<PlayStmt>,
    current: u32,
}

fn init_compiler(statements: Vec<PlayStmt>, path: &str) -> Compiler {
    Compiler {
        file: File::create(path.to_owned() + ".mid").expect("Unable To Create .midi File"),
        file_bytes: vec![],
        statements,
        current: 0,
    }
}

impl Compiler {
    fn header_chunk(compiler: &mut Compiler) {
        let mut header: Vec<u8> = vec![
            /*-----Header-Data----//-------Value-|-Description--------*/

            0x4d, 0x54, 0x68, 0x64, // MThd | ASCII Header Chunk Type
            0x00, 0x00, 0x00, 0x06, // 6 | 32 Bit Header Size
            0x00, 0x00, // 0 | 16 Bit File Format | Single Track
            0x00, 0x01, // 1 | Number Of Track Chunks
            0b00000000_1100010_0, // 98_0 | 98 Ticks Per Quarter Note
        ];

        compiler.file_bytes.append(&mut header);
    }
}

pub fn compile(statements: Vec<PlayStmt>, path: &str) {
    let mut compiler = init_compiler(statements, path);
    Compiler::header_chunk(&mut compiler);
    compiler.file.write(&mut compiler.file_bytes).expect("Could Not Generate .midi File");
}

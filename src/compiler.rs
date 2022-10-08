use std::fs::File;
use std::io::Write;

use crate::lexer::Token;

pub fn compile(tokens: Vec<Token>, path: &str) {
    let mut file = File::create(path.to_owned() + ".mid").expect("Unable To Create .midi File");
    header_chunk(&mut file);
    parse(tokens, &mut file);
}

fn header_chunk(file: &mut File) {
    let mut header: Vec<u8> = vec![
        /*-----Header-Data----//-------Value-|-Description--------*/

        0x4d, 0x54, 0x68, 0x64, // MThd | ASCII Header Chunk Type
        0x00, 0x00, 0x00, 0x06, // 6 | 32 Bit Header Size
        0x00, 0x00, // 0 | 16 Bit File Format | Single Track
        0x00, 0x01, // 1 | Number Of Track Chunks
        0b00000000_1100010_0, // 98_0 | 98 Ticks Per Quarter Note
    ];

    file.write(&mut header).expect("Could Not Generate .midi File");
}

fn parse(tokens: Vec<Token>, file: &mut File) {
}

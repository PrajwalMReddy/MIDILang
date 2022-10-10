use std::fs::File;
use std::io::Write;

use crate::lexer::Token;

struct Parser {
    file: File,
    file_bytes: Vec<u8>,
    tokens: Vec<Token>,
    current: u32,
}

fn init_parser(tokens: Vec<Token>, path: &str) -> Parser {
    Parser {
        file: File::create(path.to_owned() + ".mid").expect("Unable To Create .midi File"),
        file_bytes: vec![],
        tokens,
        current: 0,
    }
}

pub fn compile(tokens: Vec<Token>, path: &str) {
    let mut parser = init_parser(tokens, path);

    header_chunk(&mut parser);
    parse(&mut parser);

    parser.file.write(&mut parser.file_bytes).expect("Could Not Generate .midi File");
}

fn header_chunk(parser: &mut Parser) {
    let mut header: Vec<u8> = vec![
        /*-----Header-Data----//-------Value-|-Description--------*/

        0x4d, 0x54, 0x68, 0x64, // MThd | ASCII Header Chunk Type
        0x00, 0x00, 0x00, 0x06, // 6 | 32 Bit Header Size
        0x00, 0x00, // 0 | 16 Bit File Format | Single Track
        0x00, 0x01, // 1 | Number Of Track Chunks
        0b00000000_1100010_0, // 98_0 | 98 Ticks Per Quarter Note
    ];

    parser.file_bytes.append(&mut header);
}

fn parse(parser: &mut Parser) {
    track_chunk(parser);
    program_node(parser)
}

// Sets Up The Track Chunk
fn track_chunk(parser: &mut Parser) {
    let mut track: Vec<u8> = vec![
        0x4d, 0x54, 0x72, 0x6b, // MTrk | ASCII Track Chunk Type
        0x00, 0x00, 0x00, 0x00, // TBOL | Number Of Bytes In The Track Chunk
    ];

    parser.file_bytes.append(&mut track);
}

// Compiles The Actual Track Events
fn program_node(parser: &mut Parser) {
}

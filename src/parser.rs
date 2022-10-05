use std::fs::File;
use std::io::Write;

use crate::lexer::Token;

pub fn parse(tokens: Vec<Token>, path: &str) {
    let mut file = File::create(path.to_owned() + ".midi").expect("Unable To Create .midi File");
    header_chunk(&mut file);
}

fn header_chunk(file: &mut File) {
    let mut header: Vec<u8> = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21];
    file.write(&mut header).expect("Could Not Generate .midi File");
}

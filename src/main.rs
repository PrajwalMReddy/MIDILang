mod tokenizer;
mod compiler;

use std::env;
use std::fs;

use crate::tokenizer::scanner;
use crate::compiler::parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = read_file(&args);

    scanner::lex();
    parser::parse();
}

fn read_file(args: &Vec<String>) -> String {
    if args.len() == 1 {
        panic!("A File Path Must Be Given");
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect(&("Unable To Read File ".to_owned() + file_path));

    contents
}

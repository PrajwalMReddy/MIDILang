mod lexer;
mod parser;

use std::env;
use std::fs;

use crate::parser::CompilerResult;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("A File Path Must Be Given");
    }

    let contents = read_file(&args);

    let tokens = lexer::lex(contents);
    match parser::parse(tokens) {
        CompilerResult::CrSuccess => {
            println!(".midi File Generated Successfully");
        }
        CompilerResult::CrFailure(msg) => {
            panic!("{}", msg);
        }
    };
}

fn read_file(args: &Vec<String>) -> String {
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect(&("Unable To Read File ".to_owned() + file_path));

    contents
}

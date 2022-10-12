use std::env;
use std::fs;
use std::path::Path;

mod lexer;
mod compiler;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("A File Path Must Be Given");
    }

    let contents = read_file(&args);
    let path = Path::new(&args[1]).file_stem().expect("Unable To Read File Stem").to_str().unwrap();

    let tokens = lexer::lex(contents);
    let statements = parser::parse(tokens);
    compiler::compile(statements, path);
}

fn read_file(args: &Vec<String>) -> String {
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect(&("Unable To Read File ".to_owned() + file_path));

    contents
}

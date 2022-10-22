use std::env;
use std::fs;
use std::path::Path;

mod lexer;
mod compiler;
mod parser;
mod error;
mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("A File Path Must Be Given");
    }


    let contents = read_file(&args);
    let path = Path::new(&args[1]).file_stem().expect("Unable To Read File Stem").to_str().unwrap();
    let errors = error::init_errors(&contents);

    let (tokens, errors) = lexer::lex(contents, errors); errors.display_if_has_errors();
    let (statements, errors) = parser::parse(tokens, errors); errors.display_if_has_errors();
    let errors = compiler::compile(statements, path, errors); errors.display_if_has_errors();

    println!("\nSuccessfully Generated {}.midi", path);
}

fn read_file(args: &Vec<String>) -> String {
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect(&("Unable To Read File ".to_owned() + file_path));

    contents
}

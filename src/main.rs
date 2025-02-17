use std::env;
use std::fs;

pub mod lexer;
pub mod parser;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file_content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lexer = lexer::Lexer::new(&file_content);
    let tokens = lexer.collect::<Vec<lexer::tokens::Token>>();
    println!("Tokens: {:?}", tokens);

    let syntax_tree = parser::parse(&tokens);
    println!("Syntax Tree: {:?}", syntax_tree);
}


mod lexer;
mod token;
mod scanner;
mod helps;
use crate::lexer::LexerTokenizer;
use crate::lexer::LexerCreate;
use std::path::Path;
#[macro_use]
extern crate lazy_static;

fn main() {
    let mut reader1 =lexer::Lexer::from_string("dfsdf");
    let mut reader2 =lexer::Lexer::from_file_name(Path::new("./sfgdf.drk"));
    reader1.get_token();
    reader2.get_token();
}
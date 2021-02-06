use std::path::Path;

mod lexer;
mod token;
mod token_regex;
mod utils;

use lexer::{LexerAnalyzer, MyLexerAnalyzer};
use crate::utils::lexer_serialize::serialize_lexer_to_file;

fn main() {
    let mut my_lexer =Box::new(MyLexerAnalyzer::from_file(Path::new("assignment1/lexpositivegrading.src")));

    serialize_lexer_to_file(my_lexer, "outlex.src");
}

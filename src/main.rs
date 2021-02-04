use std::path::Path;

mod lexer;
mod token;
mod token_regex;
mod utils;

use lexer::{LexerAnalyzer, MyLexerAnalyzer};

fn main() {
    let mut my_lexer = MyLexerAnalyzer::from_file(Path::new("assignment1/lexpositivegrading.src"));

    while let Some(token) = my_lexer.next_token()
    {
        println!("{:?}", token);
    }
}

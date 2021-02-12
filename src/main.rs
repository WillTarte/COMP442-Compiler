use std::path::PathBuf;

mod lexer;
mod token;
mod token_regex;
mod utils;

use crate::utils::lexer_serialize::serialize_lexer_to_file;
use lexer::MyLexerAnalyzer;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Lexer Driver")]
struct Opt {
    /// File to parse tokens from
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let my_lexer = Box::new(MyLexerAnalyzer::from_file(&opt.file));

    let file_name: &str = &opt.file.file_stem().unwrap().to_str().unwrap();

    serialize_lexer_to_file(my_lexer, file_name).unwrap()
}

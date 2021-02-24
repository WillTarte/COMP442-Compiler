use std::path::PathBuf;
use structopt::StructOpt;
use crate::lexer::lexer::MyLexerAnalyzer;
use crate::lexer::utils::lexer_serialize::serialize_lexer_to_file;

mod lexer;
mod parser;

#[derive(StructOpt, Debug)]
#[structopt(name = "Lexer Driver")]
struct Opt {
    /// File to parse tokens from
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let my_lexer = MyLexerAnalyzer::from_file(&opt.file);

    let file_name: &str = &opt.file.file_stem().unwrap().to_str().unwrap();

    serialize_lexer_to_file(my_lexer, file_name).unwrap()
}

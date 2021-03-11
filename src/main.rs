use crate::lexer::lexer::MyLexerAnalyzer;
use crate::lexer::utils::lexer_serialize::serialize_lexer_to_file;
use crate::parser::parser::parse;
use std::path::PathBuf;
use structopt::StructOpt;
use crate::parser::utils::{serialize_parsing_table_to_file, serialize_tree_to_file};

mod lexer;
mod parser;

#[derive(StructOpt, Debug)]
#[structopt(name = "Compiler Driver")]
struct Opt {
    /// File to parse tokens from
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
    #[structopt(short, long)]
    lexer: bool,
    #[structopt(short, long)]
    parser: bool,
}

fn main() {
    let opt = Opt::from_args();

    if opt.lexer {
        let my_lexer = MyLexerAnalyzer::from_file(&opt.file);

        let file_name: &str = &opt.file.file_stem().unwrap().to_str().unwrap();

        serialize_lexer_to_file(my_lexer, file_name).unwrap();
    }

    if opt.parser {
        let my_lexer = MyLexerAnalyzer::from_file(&opt.file);

        let file_name: &str = &opt.file.file_stem().unwrap().to_str().unwrap();

        match parse(my_lexer)
        {
            Ok((table, ast)) => {
                println!("success");
                serialize_parsing_table_to_file(table, file_name);
                serialize_tree_to_file(ast, file_name);
            }
            Err(_) => { println!("failure"); }
        }
    }
}

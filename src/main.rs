use crate::lexer::lexer::MyLexerAnalyzer;
use crate::lexer::utils::lexer_serialize::serialize_lexer_to_file;
use crate::parser::parse::parse;
use crate::parser::utils::{serialize_derivation_table_to_file, serialize_tree_to_file};
use dotenv::dotenv;
use env_logger;
use log::{error, info};
use std::path::PathBuf;
use structopt::StructOpt;

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
    dotenv().ok();
    env_logger::init();
    let opt = Opt::from_args();

    if opt.lexer {
        let file_name: &str = &opt.file.file_stem().unwrap().to_str().unwrap();
        let my_lexer = MyLexerAnalyzer::from_file(&opt.file);

        match serialize_lexer_to_file(my_lexer, file_name) {
            Ok(_) => {
                info!("Wrote output to file successfully.")
            }
            Err(_) => {
                error!("Failed to write output to file.")
            }
        }
    }

    if opt.parser {
        let my_lexer = MyLexerAnalyzer::from_file(&opt.file);

        let file_name: &str = &opt.file.file_stem().unwrap().to_str().unwrap();

        match parse(my_lexer) {
            Ok((table, ast)) => {
                info!(
                    "Successfully parsed token stream for {}",
                    &opt.file.file_name().unwrap().to_str().unwrap()
                );
                info!("Writing derivation table and abstract syntax tree to file");
                serialize_derivation_table_to_file(table, file_name)
                    .expect("Failed to serialize derivation table");
                serialize_tree_to_file(ast, file_name).expect("Failed to serialize AST to file");
            }
            Err(_) => {
                error!("Failed to parse token stream for {}", file_name);
            }
        }
    }
}

use crate::codegen::generator::MoonGenerator;
use crate::codegen::utils::write_moon_code_to_file;
use crate::lexer::lexer::MyLexerAnalyzer;
use crate::lexer::utils::lexer_serialize::serialize_lexer_to_file;
use crate::parser::parse::parse;
use crate::parser::utils::{serialize_derivation_table_to_file, serialize_tree_to_file};
use crate::semantics::checking::{SemanticError, WarningType};
use crate::semantics::symbol_table::{check_semantics, generate_symbol_table};
use crate::semantics::utils::{serialize_symbol_table_to_file, write_semantic_error_to_file};
use dotenv::dotenv;
use env_logger;
use log::{error, info};
use std::path::PathBuf;
use structopt::StructOpt;

mod codegen;
mod lexer;
mod parser;
mod semantics;

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
    #[structopt(short, long)]
    symbols: bool,
    #[structopt(short, long)]
    codegen: bool,
}

fn main() {
    dotenv().ok();
    env_logger::init();
    let opt = Opt::from_args();

    let file_name: &str = &opt.file.file_stem().unwrap().to_str().unwrap();
    let my_lexer = MyLexerAnalyzer::from_file(&opt.file);

    if opt.lexer {
        match serialize_lexer_to_file(my_lexer, file_name) {
            Ok(_) => {
                info!("Wrote output to file successfully.")
            }
            Err(_) => {
                error!("Failed to write output to file.")
            }
        }
    } else if opt.parser {
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
    } else if opt.symbols {
        match parse(my_lexer) {
            Ok((_, ast)) => {
                info!(
                    "Successfully parsed token stream for {}",
                    &opt.file.file_name().unwrap().to_str().unwrap()
                );
                //assert_eq!(ast.0.len(), 1);
                let root = ast.into_ast_root();
                if root.is_err() {
                    log::error!("Failed to generate Abstract Syntax Tree");
                    return;
                }
                let root = root.unwrap();

                let (symbol_table, mut errors) = generate_symbol_table(&root);

                errors.append(&mut check_semantics(&root, &symbol_table));

                write_semantic_error_to_file(errors, file_name);

                info!("Writing symbol tables to file");
                serialize_symbol_table_to_file(&symbol_table, file_name)
                    .expect("Failed to serialize symbol table to file");
            }
            Err(_) => {
                error!("Failed to parse token stream for {}", file_name);
            }
        }
    } else if opt.codegen {
        match parse(my_lexer) {
            Ok((_, ast)) => {
                info!(
                    "Successfully parsed token stream for {}",
                    &opt.file.file_name().unwrap().to_str().unwrap()
                );
                //assert_eq!(ast.0.len(), 1);
                let root = ast.into_ast_root();
                if root.is_err() {
                    log::error!("Failed to generate Abstract Syntax Tree");
                    return;
                }
                let root = root.unwrap();

                let (symbol_table, mut errors) = generate_symbol_table(&root);

                errors.append(&mut check_semantics(&root, &symbol_table));

                write_semantic_error_to_file(errors, file_name);

                if true // since we get false positive errors
                {
                    let mut code_generator = MoonGenerator::new();

                    code_generator.generate(&root, &symbol_table);

                    let output = code_generator.finish();

                    log::info!("Writing moon code to file");
                    write_moon_code_to_file(output, file_name).unwrap();

                    return;
                }
            }
            Err(_) => {
                error!("Failed to parse token stream for {}", file_name);
            }
        }
    }
}

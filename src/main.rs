use crate::lexer::lexer::MyLexerAnalyzer;
use crate::lexer::utils::lexer_serialize::serialize_lexer_to_file;
use crate::parser::parse::parse;
use crate::parser::utils::{serialize_derivation_table_to_file, serialize_tree_to_file};
use crate::semantics::checking::{SemanticError, WarningType};
use crate::semantics::symbol_table::{check_semantics, generate_symbol_table};
use crate::semantics::utils::serialize_symbol_table_to_file;
use crate::codegen::generator::{MoonGenerator, CodeGenOutput};
use dotenv::dotenv;
use env_logger;
use log::{error, info};
use std::path::PathBuf;
use structopt::StructOpt;

mod lexer;
mod parser;
mod semantics;
mod codegen;

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
    }
    else if opt.parser {
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
    else if opt.symbols {
        match parse(my_lexer) {
            Ok((_, mut ast)) => {
                info!(
                    "Successfully parsed token stream for {}",
                    &opt.file.file_name().unwrap().to_str().unwrap()
                );
                //assert_eq!(ast.0.len(), 1);
                let root = ast.into_ast_root();
                if root.is_err()
                {
                    log::error!("Failed to generate Abstract Syntax Tree");
                    return;
                }
                let root = root.unwrap();

                let (symbol_table, mut errors) = generate_symbol_table(&root);

                errors.append(&mut check_semantics(&root, &symbol_table));


                for err in errors.iter() {
                    match err {
                        SemanticError::Warning(warning) => match warning {
                            WarningType::OverloadWarning(msg)
                            | WarningType::ShadowedMemberWarning(msg) => {
                                log::warn!("{}", msg);
                            }
                        },
                        SemanticError::NoMemberFuncDefinition(msg)
                        | SemanticError::NoMemberFuncDeclaration(msg)
                        | SemanticError::MultipleDeclIdent(msg)
                        | SemanticError::InheritanceCycle(msg)
                        | SemanticError::UndeclaredClass(msg)
                        | SemanticError::UndeclaredVariable(msg)
                        | SemanticError::NotIndexable(msg)
                        | SemanticError::TooManyIndices(msg)
                        | SemanticError::FunctionNotFound(msg)
                        | SemanticError::InvalidParameters(msg)
                        | SemanticError::TypeMistmatch(msg)
                        | SemanticError::NotCallable(msg)
                        | SemanticError::RecursionNotSupported(msg)
                        | SemanticError::NotClassType(msg) => {
                            log::error!("{}", msg);
                        }
                    }
                }

                info!("Writing symbol tables to file");
                serialize_symbol_table_to_file(&symbol_table, file_name)
                    .expect("Failed to serialize symbol table to file");
            }
            Err(_) => {
                error!("Failed to parse token stream for {}", file_name);
            }
        }
    }
    else if opt.codegen
    {
        match parse(my_lexer) {
            Ok((_, mut ast)) => {
                info!(
                    "Successfully parsed token stream for {}",
                    &opt.file.file_name().unwrap().to_str().unwrap()
                );
                //assert_eq!(ast.0.len(), 1);
                let root = ast.into_ast_root();
                if root.is_err()
                {
                    log::error!("Failed to generate Abstract Syntax Tree");
                    return;
                }
                let root = root.unwrap();

                let (symbol_table, mut errors) = generate_symbol_table(&root);

                errors.append(&mut check_semantics(&root, &symbol_table));


                for err in errors.iter() {
                    match err {
                        SemanticError::Warning(warning) => match warning {
                            WarningType::OverloadWarning(msg)
                            | WarningType::ShadowedMemberWarning(msg) => {
                                log::warn!("{}", msg);
                            }
                        },
                        SemanticError::NoMemberFuncDefinition(msg)
                        | SemanticError::NoMemberFuncDeclaration(msg)
                        | SemanticError::MultipleDeclIdent(msg)
                        | SemanticError::InheritanceCycle(msg)
                        | SemanticError::UndeclaredClass(msg)
                        | SemanticError::UndeclaredVariable(msg)
                        | SemanticError::NotIndexable(msg)
                        | SemanticError::TooManyIndices(msg)
                        | SemanticError::FunctionNotFound(msg)
                        | SemanticError::InvalidParameters(msg)
                        | SemanticError::TypeMistmatch(msg)
                        | SemanticError::NotCallable(msg)
                        | SemanticError::RecursionNotSupported(msg)
                        | SemanticError::NotClassType(msg) => {
                            log::error!("{}", msg);
                        }
                    }
                }

                if true //todo if errors, dont generate code
                {
                    let mut code_generator = MoonGenerator::new();

                    code_generator.generate(&root, &symbol_table);

                    let output = code_generator.finish();

                    log::info!("{}", output);

                    return;
                }
            }
            Err(_) => {
                error!("Failed to parse token stream for {}", file_name);
            }
        }
    }
}

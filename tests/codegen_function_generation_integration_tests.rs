extern crate comp442_compiler;
use common::init;
use comp442_compiler::codegen::generator::MoonGenerator;
use comp442_compiler::semantics::symbol_table::generate_symbol_table;

mod common;

#[test]
fn codegen_function_code_generation_should_work1() {
    init();

    let lexer = common::setup_lexer_from_file("tests/codegen/assignment.src");

    let parse_res = comp442_compiler::parser::parse::parse(lexer);
    assert!(parse_res.is_ok());
    let root = parse_res.unwrap().1.into_ast_root();
    assert!(root.is_ok());
    let root = root.unwrap();
    let (symbol_table, errors) = generate_symbol_table(&root);
    assert_eq!(errors.len(), 0);

    let mut codegen = MoonGenerator::new();

    codegen.generate(&root, &symbol_table);

    let output = codegen.finish();

    log::info!("{}", output);
}

#[test]
fn codegen_function_code_generation_should_work2() {
    init();

    let lexer = common::setup_lexer_from_file("tests/codegen/assignment_expressions.src");

    let parse_res = comp442_compiler::parser::parse::parse(lexer);
    assert!(parse_res.is_ok());
    let root = parse_res.unwrap().1.into_ast_root();
    assert!(root.is_ok());
    let root = root.unwrap();
    let (symbol_table, errors) = generate_symbol_table(&root);
    assert_eq!(errors.len(), 0);

    let mut codegen = MoonGenerator::new();

    codegen.generate(&root, &symbol_table);

    let output = codegen.finish();

    log::info!("{}", output);
}

#[test]
fn codegen_function_code_generation_should_work3() {
    init();

    let lexer = common::setup_lexer_from_file("tests/codegen/array_indexing.src");

    let parse_res = comp442_compiler::parser::parse::parse(lexer);
    assert!(parse_res.is_ok());
    let root = parse_res.unwrap().1.into_ast_root();
    assert!(root.is_ok());
    let root = root.unwrap();
    let (symbol_table, errors) = generate_symbol_table(&root);
    assert_eq!(errors.len(), 0);

    let mut codegen = MoonGenerator::new();

    codegen.generate(&root, &symbol_table);

    let output = codegen.finish();

    log::info!("{}", output);
}

#[test]
fn codegen_function_code_generation_should_work4() {
    init();

    let lexer = common::setup_lexer_from_file("tests/codegen/while_loop.src");

    let parse_res = comp442_compiler::parser::parse::parse(lexer);
    assert!(parse_res.is_ok());
    let root = parse_res.unwrap().1.into_ast_root();
    assert!(root.is_ok());
    let root = root.unwrap();
    let (symbol_table, errors) = generate_symbol_table(&root);
    assert_eq!(errors.len(), 0);

    let mut codegen = MoonGenerator::new();

    codegen.generate(&root, &symbol_table);

    let output = codegen.finish();

    log::info!("{}", output);
}

#[test]
fn codegen_function_code_generation_should_work5() {
    init();

    let lexer = common::setup_lexer_from_file("tests/codegen/if_statement.src");

    let parse_res = comp442_compiler::parser::parse::parse(lexer);
    assert!(parse_res.is_ok());
    let root = parse_res.unwrap().1.into_ast_root();
    assert!(root.is_ok());
    let root = root.unwrap();
    let (symbol_table, errors) = generate_symbol_table(&root);
    assert_eq!(errors.len(), 0);

    let mut codegen = MoonGenerator::new();

    codegen.generate(&root, &symbol_table);

    let output = codegen.finish();

    log::info!("{}", output);
}

#[test]
fn codegen_function_code_generation_should_work6() {
    init();

    let lexer = common::setup_lexer_from_file("tests/codegen/function_call.src");

    let parse_res = comp442_compiler::parser::parse::parse(lexer);
    assert!(parse_res.is_ok());
    let root = parse_res.unwrap().1.into_ast_root();
    assert!(root.is_ok());
    let root = root.unwrap();
    let (symbol_table, errors) = generate_symbol_table(&root);
    assert_eq!(errors.len(), 0);

    let mut codegen = MoonGenerator::new();

    codegen.generate(&root, &symbol_table);

    let output = codegen.finish();

    log::info!("{}", output);
}

extern crate comp442_compiler;
use comp442_compiler::semantics::symbol_table::generate_symbol_table;
use comp442_compiler::codegen::generator::MoonGenerator;
use common::init;

mod common;

#[test]
fn codegen_function_code_generation_should_work() {
    init();

    let lexer = common::setup_lexer_from_file("tests/codegen/bubblesort.src");

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
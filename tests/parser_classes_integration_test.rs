extern crate comp442_compiler;
use common::init;

mod common;

#[test]
fn parser_classes_should_work() {
    init();

    let lexer = common::setup_lexer_from_file("tests/parser/classes/classes_should_work.src");

    assert!(comp442_compiler::parser::parse::parse(lexer).is_ok());
}

#[test]
fn parser_no_classes_should_work() {
    init();

    let lexer =
        common::setup_lexer_from_file("tests/parser/classes/no_classes_should_work.src.src");

    assert!(comp442_compiler::parser::parse::parse(lexer).is_ok());
}

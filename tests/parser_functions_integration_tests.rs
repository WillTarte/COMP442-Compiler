extern crate comp442_compiler;
use common::init;

mod common;

#[test]
fn parser_functions_should_work() {
    init();

    let lexer = common::setup_lexer_from_file("tests/parser/functions/functions_should_work.src");

    assert!(comp442_compiler::parser::parse::parse(lexer).is_ok());
}

#[test]
fn parser_no_functions_should_work() {
    init();

    let lexer =
        common::setup_lexer_from_file("tests/parser/functions/no_functions_should_work.src");

    assert!(comp442_compiler::parser::parse::parse(lexer).is_ok());
}

#[test]
fn parser_no_main_should_fail() {
    init();

    let lexer = common::setup_lexer_from_string("");

    assert!(comp442_compiler::parser::parse::parse(lexer).is_err());
}

#[test]
fn parser_multiple_main_should_fail() {
    init();

    let lexer = common::setup_lexer_from_string("main {}\nmain{}");

    assert!(comp442_compiler::parser::parse::parse(lexer).is_err());
}

#[test]
fn parser_function_array_return_should_fail() {
    init();

    let lexer = common::setup_lexer_from_file(
        "tests/parser/functions/function_returns_array_should_fail.src",
    );

    assert!(comp442_compiler::parser::parse::parse(lexer).is_err());
}

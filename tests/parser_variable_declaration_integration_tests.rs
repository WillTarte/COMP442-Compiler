extern crate comp442_compiler;
use common::init;

mod common;

#[test]
fn variable_declarations_should_work() {
    init();

    let lexer = common::setup_lexer_from_file(
        "tests/parser/variable_declarations/variable_declarations_should_work.src",
    );

    assert!(comp442_compiler::parser::parse::parse(lexer).is_ok());
}

#[test]
fn variable_declarations_multiple_should_fail() {
    init();

    let lexer = common::setup_lexer_from_string("func f1() : void {var{}var{}} main{}");

    assert!(comp442_compiler::parser::parse::parse(lexer).is_err());
}

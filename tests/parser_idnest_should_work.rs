extern crate comp442_compiler;
use common::init;

mod common;

#[test]
fn parser_idnest_should_work() {
    init();

    let lexer = common::setup_lexer_from_file("tests/parser/idnest/idnest_should_work.src");

    assert!(comp442_compiler::parser::parse::parse(lexer).is_ok());
}

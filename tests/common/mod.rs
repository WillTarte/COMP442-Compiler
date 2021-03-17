use comp442_compiler::lexer::lexer::MyLexerAnalyzer;
use dotenv::dotenv;
use env_logger;
use std::path::Path;

pub fn setup_lexer_from_file<P: AsRef<Path>>(file_name: P) -> MyLexerAnalyzer {
    return MyLexerAnalyzer::from_file(file_name);
}

#[allow(dead_code)]
pub fn setup_lexer_from_string(src: &str) -> MyLexerAnalyzer {
    return MyLexerAnalyzer::from_str(src);
}

pub fn init() {
    dotenv().ok();
    env_logger::builder()
        .is_test(true)
        .parse_filters("trace")
        .init();
}

use crate::token::Token;
use std::path::Path;

trait LexerAnalyzer {
    type TokenOutput;

    /// Returns the next character in the input stream without advancing the lexer
    fn peek(&self) -> char;

    /// Returns the character n positions after the current position in the input stream without advancing the lexer
    fn peek_n(&self, n: usize) -> char;

    /// Returns the next character, advancing the lexer
    fn next_char(&mut self) -> char;

    /// Returns the next token without advancing the lexer
    fn peek_token(&mut self) -> Self::TokenOutput;

    /// Returns the next token, advancing the lexer
    fn next_token(&mut self) -> Self::TokenOutput;
}

struct MyLexerAnalyzer;

impl LexerAnalyzer for MyLexerAnalyzer {
    type TokenOutput = Token;

    fn peek(&self) -> char {
        todo!()
    }

    fn peek_n(&self, n: usize) -> char {
        todo!()
    }

    fn next_char(&mut self) -> char {
        todo!()
    }

    fn peek_token(&mut self) -> Self::TokenOutput {
        todo!()
    }

    fn next_token(&mut self) -> Self::TokenOutput {
        todo!()
    }
}

struct LexerInput(String);

impl LexerInput {
    fn from_str(input: &str) -> Self {
        LexerInput(input.to_string())
    }

    fn from_file<P: AsRef<Path>>(filename: P) -> Self {
        let content: String =
            std::fs::read_to_string(&filename).expect("Failed to read file content");
        LexerInput(content)
    }
}

pub struct MyLexer {
    input: LexerInput,
    analyzer: Box<dyn LexerAnalyzer<TokenOutput = Token>>,
}

impl MyLexer {
    fn from_str(s: &str) -> Self {
        MyLexer {
            input: LexerInput::from_str(s),
            analyzer: Box::new(MyLexerAnalyzer),
        }
    }

    fn from_file<P: AsRef<Path>>(filename: P) -> Self {
        MyLexer {
            input: LexerInput::from_file(filename),
            analyzer: Box::new(MyLexerAnalyzer),
        }
    }
}

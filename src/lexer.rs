use crate::token::{Token, TokenType};
use std::path::Path;

trait LexerAnalyzer {
    type TokenOutput;

    /// Returns the next character in the input stream without advancing the lexer
    fn peek(&self, input: &str) -> Option<char>;

    /// Returns the character n positions after the current position in the input stream without advancing the lexer
    fn peek_n(&self, input: &str, n: usize) -> Option<char>;

    /// Returns the next character, advancing the lexer
    fn next_char(&mut self, input: &str) -> Option<char>;

    /// Returns the next token without advancing the lexer
    fn peek_token(&mut self, input: &str) -> Self::TokenOutput;

    /// Returns the next token, advancing the lexer
    fn next_token(&mut self, input: &str) -> Self::TokenOutput;
}

struct MyLexerAnalyzer {
    idx: usize,
}

impl LexerAnalyzer for MyLexerAnalyzer {
    type TokenOutput = Token;

    fn peek(&self, input: &str) -> Option<char> {
        input.chars().nth(self.idx)
    }

    fn peek_n(&self, input: &str, n: usize) -> Option<char> {
        input.chars().nth(self.idx + n)
    }

    fn next_char(&mut self, input: &str) -> Option<char> {
        let c = input.chars().nth(self.idx);
        self.idx += 1;
        c
    }

    fn peek_token(&mut self, input: &str) -> Self::TokenOutput {
        todo!()
    }

    fn next_token(&mut self, input: &str) -> Self::TokenOutput {
        let char_buffer: Vec<char> = Vec::new();

        // Peek next char: If space, go next, else add it to the buffer
        // Keep going till you reach next space
        // Special cases: comments & strings

        Token::new(TokenType::Error, "ERROR", 0)
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
            analyzer: Box::new(MyLexerAnalyzer { idx: 0 }),
        }
    }

    fn from_file<P: AsRef<Path>>(filename: P) -> Self {
        MyLexer {
            input: LexerInput::from_file(filename),
            analyzer: Box::new(MyLexerAnalyzer { idx: 0 }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::MyLexer;
    use std::borrow::Borrow;
    use std::path::Path;

    #[test]
    fn my_lexer_from_str() {
        let input =
            String::from("Phasellus suscipit mauris purus, quis dictum velit iaculis eget.");

        let my_lexer = MyLexer::from_str(input.borrow());

        assert_eq!(my_lexer.input.0, input);
    }

    #[test]
    fn my_lexer_from_file() {
        let input = std::fs::read_to_string(Path::new("assignment1/lorem_ipsum.txt")).unwrap();

        let my_lexer = MyLexer::from_file(Path::new("assignment1/lorem_ipsum.txt"));

        assert_eq!(my_lexer.input.0, input);
    }
}

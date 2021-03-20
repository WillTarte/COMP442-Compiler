//! Lexer implementation for the compiler

use crate::lexer::token::InvalidTokenType::InvalidCharacter;
use crate::lexer::token::{Token, TokenFragment, TokenType};
use crate::lexer::utils::lexer::{
    is_valid_character, parse_kw_or_id, parse_number, parse_op_or_punct, parse_string,
};
use crate::lexer::utils::LINE_ENDINGS_RE;
use log::{info, trace};
use std::path::Path;

/// Interface for a Lexer Analyzer
pub trait LexerAnalyzer {
    type TokenOutput;

    /// moves the cursor back 1 character
    fn back(&mut self);

    /// moves the cursor back n characters
    fn back_n(&mut self, n: usize);

    /// moves the cursor forwards 1 character
    fn forward(&mut self);

    /// moves the cursor forwards n characters
    fn forward_n(&mut self, n: usize);

    /// Returns the next character in the input stream without advancing the lexer
    fn peek(&self) -> Option<char>;

    /// Returns the character n positions after the current position in the input stream without advancing the lexer
    fn peek_n(&self, n: usize) -> Option<char>;

    /// Returns the next character, advancing the lexer
    fn next_char(&mut self) -> Option<char>;

    /// Returns the next token, advancing the lexer
    fn next_token(&mut self) -> Option<Self::TokenOutput>;

    /// skips any whitespace at the beginning of the input
    fn skip_whitespace(&mut self);
}

/// My Implementation of a Lexer Analyzer
pub struct MyLexerAnalyzer {
    input: LexerInput,
    idx: usize,
    line_num: usize,
}

impl MyLexerAnalyzer {
    #[allow(dead_code)]
    // Builds a LexerInput from a String
    pub fn from_str(s: &str) -> Self {
        info!("Lexing string {}", s);
        Self {
            input: LexerInput::from_str(s),
            idx: 0,
            line_num: 1,
        }
    }

    /// Reads the content of a given file to build the `LexerInput`
    /// # Arguments
    /// * `filename` - the file path to read from
    #[allow(dead_code)]
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Self {
        info!("Lexing file {:?}", filename.as_ref());
        Self {
            input: LexerInput::from_file(filename),
            idx: 0,
            line_num: 1,
        }
    }
}

impl LexerAnalyzer for MyLexerAnalyzer {
    type TokenOutput = Token;

    fn back(&mut self) {
        self.idx -= 1;
    }

    fn back_n(&mut self, n: usize) {
        self.idx -= n;
    }

    fn forward(&mut self) {
        self.idx += 1;
    }

    fn forward_n(&mut self, n: usize) {
        self.idx += n;
    }

    fn peek(&self) -> Option<char> {
        return if self.idx == self.input.0.as_bytes().len() {
            None
        } else {
            Some(self.input.0.as_bytes()[self.idx] as char)
        };
    }

    fn peek_n(&self, n: usize) -> Option<char> {
        return if self.idx + n == self.input.0.as_bytes().len() {
            None
        } else {
            Some(self.input.0.as_bytes()[self.idx + n] as char)
        };
    }

    fn next_char(&mut self) -> Option<char> {
        return if self.idx == self.input.0.as_bytes().len() {
            None
        } else {
            let c = self.input.0.as_bytes()[self.idx];
            self.idx += 1;
            Some(c as char)
        };
    }

    fn next_token(&mut self) -> Option<Self::TokenOutput> {
        self.skip_whitespace();

        if self.idx == self.input.0.len() {
            return None;
        }

        let first_char: char = match self.peek() {
            None => return None,
            Some(c) => c,
        };

        let input_fragment = match self.input.0.get(self.idx..) {
            None => return None,
            Some(s) => s,
        };

        let next_token = if first_char.is_ascii_alphabetic() || first_char == '_' {
            // Probably a keyword or an identifier
            let token_fragment = parse_kw_or_id(input_fragment);
            self.forward_n(token_fragment.lexeme.len());
            Some(Token::new(token_fragment, self.line_num))
        } else if first_char.is_ascii_digit() {
            // Probably a number (int or float)
            let token_fragment = parse_number(input_fragment);
            self.forward_n(token_fragment.lexeme.len());
            Some(Token::new(token_fragment, self.line_num))
        } else if is_valid_character(first_char) {
            // Probably a punctuation token, operator or comment
            let token_fragment = parse_op_or_punct(input_fragment);
            self.forward_n(token_fragment.lexeme.len());
            if token_fragment.token_type == TokenType::MultilineComment {
                let nl_count = LINE_ENDINGS_RE.find_iter(&token_fragment.lexeme).count();
                self.line_num += nl_count;
                return Some(Token::new(token_fragment, self.line_num - nl_count));
            }
            Some(Token::new(token_fragment, self.line_num))
        } else if first_char == '"' {
            // Probably a string literal
            let token_fragment = parse_string(input_fragment);
            self.forward_n(token_fragment.lexeme.len());
            Some(Token::new(token_fragment, self.line_num))
        } else {
            let c = &*self.next_char().unwrap().to_string();
            Some(Token::new(
                TokenFragment::new(TokenType::Error(InvalidCharacter), c),
                self.line_num,
            ))
        };
        trace!("Found: {:?}", next_token);
        return next_token;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                '\r' => match self.peek_n(1) {
                    None => {
                        self.forward();
                        return;
                    }
                    Some(nc) => {
                        if nc == '\n' {
                            self.line_num += 1;
                            self.forward_n(2);
                            continue;
                        } else {
                            self.forward();
                            continue;
                        }
                    }
                },
                '\n' => {
                    self.line_num += 1;
                    self.forward();
                    continue;
                }
                '\t' | ' ' => {
                    self.forward();
                    continue;
                }
                _ => {
                    return;
                }
            }
        }
    }
}

impl IntoIterator for MyLexerAnalyzer {
    type Item = <Self as LexerAnalyzer>::TokenOutput;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        let mut acc = Vec::new();
        while let Some(token) = self.next_token() {
            acc.push(token);
        }
        acc.into_iter()
    }
}

/// Represents the input to be fed to a Lexer
struct LexerInput(String);

impl LexerInput {
    /// Builds a LexerInput from a String
    fn from_str(input: &str) -> Self {
        LexerInput(input.to_string())
    }

    /// Reads the content of a given file to build the `LexerInput`
    /// # Arguments
    /// * `filename` - the file path to read from
    #[allow(dead_code)]
    fn from_file<P: AsRef<Path>>(filename: P) -> Self {
        let content: String =
            std::fs::read_to_string(&filename).expect("Failed to read file content");
        LexerInput(content)
    }
}

#[cfg(test)]
mod tests {
    use super::MyLexerAnalyzer;
    use crate::lexer::lexer::LexerAnalyzer;
    use crate::lexer::token::InvalidTokenType::InvalidCharacter;
    use crate::lexer::token::{Token, TokenFragment, TokenType};
    use std::borrow::Borrow;
    use std::path::Path;

    #[test]
    fn my_lexer_from_str() {
        let input =
            String::from("Phasellus suscipit mauris purus, quis dictum velit iaculis eget.");

        let my_lexer = MyLexerAnalyzer::from_str(input.borrow());

        assert_eq!(my_lexer.input.0, input);
        assert_eq!(my_lexer.idx, 0);
    }

    #[test]
    fn my_lexer_from_file() {
        let input = std::fs::read_to_string(Path::new("assignment1/lorem_ipsum.txt")).unwrap();

        let my_lexer = MyLexerAnalyzer::from_file(Path::new("assignment1/lorem_ipsum.txt"));

        assert_eq!(my_lexer.input.0, input);
        assert_eq!(my_lexer.idx, 0);
    }

    #[test]
    fn my_lexer_next_char() {
        let mut my_lexer =
            MyLexerAnalyzer::from_file(Path::new("assignment1/lexpositivegrading.src"));

        assert_eq!(my_lexer.next_char(), Some('='));
        assert_eq!(my_lexer.next_char(), Some('='));
        assert_eq!(my_lexer.next_char(), Some('\t'));
        assert_eq!(my_lexer.next_char(), Some('+'));
        assert_eq!(my_lexer.next_char(), Some('\t'));
        assert_eq!(my_lexer.next_char(), Some('|'));
        assert_eq!(my_lexer.next_char(), Some('\t'));
        assert_eq!(my_lexer.next_char(), Some('('));
    }

    #[test]
    fn my_lexer_next_token() {
        let mut my_lexer = MyLexerAnalyzer::from_file(Path::new(
            "docs/Assignment1/Assignment1_Handout/lexpositivegrading.src",
        ));

        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(TokenFragment::new(TokenType::EqEq, "=="), 1))
        );
        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(TokenFragment::new(TokenType::Plus, "+"), 1))
        );
        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(TokenFragment::new(TokenType::Or, "|"), 1))
        );
        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(TokenFragment::new(TokenType::OpenParen, "("), 1))
        );
        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(TokenFragment::new(TokenType::SemiColon, ";"), 1))
        );
        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(TokenFragment::new(TokenType::If, "if"), 1))
        );
    }

    #[test]
    fn my_lexer_invalid_characters() {
        let mut my_lexer = MyLexerAnalyzer::from_str(r"@ # $ ' \ ~ ");

        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(
                TokenFragment::new(TokenType::Error(InvalidCharacter), "@"),
                1
            ))
        );
        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(
                TokenFragment::new(TokenType::Error(InvalidCharacter), "#"),
                1
            ))
        );
        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(
                TokenFragment::new(TokenType::Error(InvalidCharacter), "$"),
                1
            ))
        );
        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(
                TokenFragment::new(TokenType::Error(InvalidCharacter), "'"),
                1
            ))
        );
        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(
                TokenFragment::new(TokenType::Error(InvalidCharacter), r"\"),
                1
            ))
        );
        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(
                TokenFragment::new(TokenType::Error(InvalidCharacter), "~"),
                1
            ))
        );
    }

    #[test]
    fn my_lexer_tokens_no_space() {
        let input = String::from("123<=456.34?");

        let mut my_lexer = MyLexerAnalyzer::from_str(input.borrow());

        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(
                TokenFragment::new(TokenType::IntegerLit, "123"),
                1
            ))
        );
        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(
                TokenFragment::new(TokenType::LessEqualThan, "<="),
                1
            ))
        );
        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(
                TokenFragment::new(TokenType::FloatLit, "456.34"),
                1
            ))
        );
        assert_eq!(
            my_lexer.next_token(),
            Some(Token::new(TokenFragment::new(TokenType::Question, "?"), 1))
        );
    }
}

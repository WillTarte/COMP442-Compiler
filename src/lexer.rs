use crate::token::InvalidTokenType::InvalidCharacter;
use crate::token::{Token, TokenFragment, TokenType};
use crate::utils::lexer::{parse_kw_or_id, parse_number, parse_op_or_punct, parse_string, is_valid_character};
use std::path::Path;

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

pub(crate) struct MyLexerAnalyzer {
    input: LexerInput,
    idx: usize,
}

impl MyLexerAnalyzer {
    fn from_str(s: &str) -> Self {
        Self {
            input: LexerInput::from_str(s),
            idx: 0,
        }
    }

    pub(crate) fn from_file<P: AsRef<Path>>(filename: P) -> Self {
        Self {
            input: LexerInput::from_file(filename),
            idx: 0,
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

        let first_char: char = self.peek()?;
        let input_fragment = self.input.0.get(self.idx..)?; //.replace(crate::token::LINE_ENDINGS, "");

        return if first_char.is_ascii_alphabetic() {
            // Probably a keyword or an identifier
            let token_fragment = parse_kw_or_id(input_fragment);
            self.forward_n(token_fragment.lexeme.len());
            Some(Token::new(token_fragment, 1)) //todo
        } else if first_char.is_ascii_digit() {
            // Probably a number (int or float)
            let token_fragment = parse_number(input_fragment);
            self.forward_n(token_fragment.lexeme.len());
            Some(Token::new(token_fragment, 1)) //todo
        } else if is_valid_character(first_char) {
            // Probably a punctuation token, operator or comment
            let token_fragment = parse_op_or_punct(input_fragment);
            self.forward_n(token_fragment.lexeme.len());
            Some(Token::new(token_fragment, 1)) //todo
        } else if first_char == '"' {
            // Probably a string literal
            let token_fragment = parse_string(input_fragment);
            self.forward_n(token_fragment.lexeme.len());
            Some(Token::new(token_fragment, 1)) //todo
        } else {
            let c = &*self.next_char().unwrap().to_string();
            Some(Token::new(
                TokenFragment::new(TokenType::Error(InvalidCharacter), c),
                1, //todo
            ))
        };
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.next_char() {
            if c.is_ascii_whitespace() || c == '\t' {
                continue;
            } else {
                self.back();
                break;
            }
        }
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

#[cfg(test)]
mod tests {
    use super::MyLexerAnalyzer;
    use crate::lexer::LexerAnalyzer;
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
        let input = std::fs::read_to_string(Path::new("assignment1/lexpositivegrading.src"));

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
}

use crate::token::{TokenFragment, TokenType, Token};
use std::path::Path;
use strum::IntoEnumIterator;
use regex::Regex;

trait LexerAnalyzer {
    type TokenOutput;

    fn back(&mut self);

    fn back_n(&mut self, n: usize);

    /// Returns the next character in the input stream without advancing the lexer
    fn peek(&self, input: &str) -> Option<char>;

    /// Returns the character n positions after the current position in the input stream without advancing the lexer
    fn peek_n(&self, input: &str, n: usize) -> Option<char>;

    /// Returns the next character, advancing the lexer
    fn next_char(&mut self, input: &str) -> Option<char>;

    /// Returns the next token without advancing the lexer
    fn peek_token(&mut self, input: &str) -> Option<Self::TokenOutput>;

    /// Returns the next token, advancing the lexer
    fn next_token(&mut self, input: &str) -> Option<Self::TokenOutput>;

    fn skip_whitespace(&mut self, input: &str)
    {
        while let Some(c) = self.next_char(input)
        {
            if c.is_whitespace()
            {
                continue;
            }
            else
            {
                self.back();
                break;
            }
        }
    }
}

struct MyLexerAnalyzer {
    idx: usize,
}

impl LexerAnalyzer for MyLexerAnalyzer {
    type TokenOutput = Token;

    fn back(&mut self)
    {
        assert!(self.idx > 0);
        self.idx -= 1;
    }

    fn back_n(&mut self, n: usize)
    {
        assert!(self.idx - n >= 0);
        self.idx -= n;
    }

    fn peek(&self, input: &str) -> Option<char> {
        Some(input.as_bytes()[self.idx] as char)
    }

    fn peek_n(&self, input: &str, n: usize) -> Option<char> {
        Some(input.as_bytes()[self.idx + n] as char)
    }

    fn next_char(&mut self, input: &str) -> Option<char> {
        let c = input.as_bytes()[self.idx];
        self.idx += 1;
        Some(c as char)
    }

    fn peek_token(&mut self, input: &str) -> Option<Self::TokenOutput> {
        todo!()
    }

    fn next_token(&mut self, input: &str) -> Option<Self::TokenOutput> {

        self.skip_whitespace(input);

        if self.peek(input).unwrap() == '/' || self.peek(input).unwrap() =='\"'
        {
            // process string and comments
            if self.peek_n(input, 1).unwrap() == '*'
            {
                // opening multiline comment
            }
        }
        else if self.peek(input).unwrap() == '*'
        {
            if self.peek_n(input, 1).unwrap() == '/'
            {
                // closing multiline comment
            }
        }

        // We can easily match operators and punctuations
        {
            // = and ==
            // <> and < and > and <= and >=
            // + and - and * and /
            // | and & and ! and ?
            // ( and ) and { and } and [ and ]
            // ; and , and . and : and ::
        }

        // For reserved words, a bit tougher because they are subexpressions of possible identifiers
        // eg. string_variable
        // So if we match a reserved keyword, we have to make sure it's alone (doesn't have extra chars)


        Some(Token::new(TokenFragment::new(TokenType::Error, "ERROR"), 0))
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
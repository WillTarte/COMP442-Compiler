use crate::token::InvalidTokenType::{
    InvalidCharacter, InvalidIdentifier, InvalidNumber, InvalidString,
};
use crate::token::{Token, TokenFragment, TokenType};
use crate::utils;
use std::path::Path;

trait LexerAnalyzer {
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
    fn peek(&self, input: &str) -> Option<char>;

    /// Returns the character n positions after the current position in the input stream without advancing the lexer
    fn peek_n(&self, input: &str, n: usize) -> Option<char>;

    /// Returns the next character, advancing the lexer
    fn next_char(&mut self, input: &str) -> Option<char>;

    /// Returns the next token, advancing the lexer
    fn next_token(&mut self, input: &str) -> Option<Self::TokenOutput>;

    /// skips any whitespace at the beginning of the input
    fn skip_whitespace(&mut self, input: &str) {
        while let Some(c) = self.next_char(input) {
            if c.is_ascii_whitespace() {
                continue;
            } else {
                self.back();
                break;
            }
        }
    }
}

struct MyLexerAnalyzer {
    idx: usize,
}

impl MyLexerAnalyzer {
    /// Parses an input string into a keyword or an identifier.
    /// If the input is neither, returns an Error token fragment.
    /// # Arguments
    /// * `input_fragment` - A string slice to parse. Should always start with a letter
    /// # Outputs
    /// * A `TokenFragment`
    fn parse_kw_or_id(&mut self, input_fragment: &str) -> TokenFragment {
        let word = input_fragment
            .chars()
            .take_while(|c: &char| c.is_ascii_alphanumeric())
            .collect::<String>();

        for kw_type in &*crate::token::KEYWORD_TOKENS {
            if kw_type.str_repr().is_match(&word) {
                return TokenFragment::new(*kw_type, &word);
            }
        }
        return if TokenType::Id.str_repr().is_match(&word) {
            TokenFragment::new(TokenType::Id, &word)
        } else {
            TokenFragment::new(TokenType::Error(InvalidIdentifier), input_fragment)
        };
    }

    /// Parses an input string into a number (float or int)
    /// If the input is not a well formed number, returns an Error token fragment.
    /// # Arguments
    /// * `input_fragment` - A string slice to parse. Should always start with a digit
    /// # Outputs
    /// * A `TokenFragment`
    fn parse_number(&mut self, input_fragment: &str) -> TokenFragment {
        // whole part - nonzero digit* | zero
        let whole_str: String = input_fragment
            .chars()
            .take_while(|c: &char| c.is_ascii_digit())
            .collect();

        // if the number is followed by letters -> invalid ID
        return if input_fragment.as_bytes()[whole_str.len()].is_ascii_alphabetic() {
            TokenFragment::new(TokenType::Error(InvalidIdentifier), &whole_str)
        } else if input_fragment.as_bytes()[whole_str.len()] as char == '.' {
            // fractional part - . digit* nonzero | .0
            let fractional_str: String = input_fragment
                .chars()
                .take_while(|c: &char| *c != '.')
                .skip(1)
                .take_while(|c: &char| c.is_ascii_digit())
                .collect::<String>();
            // exponent part (optional) - e [+|-] nonzero digit* | zero
            let exponent_str: String = input_fragment
                .chars()
                .skip_while(|c: &char| *c != 'e')
                .skip(1)
                .take_while(|c: &char| c.is_ascii_digit() || *c == '+' || *c == '-')
                .collect::<String>();

            if fractional_str.len() == 0 && fractional_str.len() == 0 {
                TokenFragment::new(
                    TokenType::Error(InvalidNumber),
                    &format!("{}.{}e{}", &whole_str, &fractional_str, &exponent_str),
                )
            } else if fractional_str.len() > 0 && fractional_str.len() == 0 {
                let float_str: String = format!("{}.{}", &whole_str, &fractional_str);
                if TokenType::FloatLit.str_repr().is_match(&float_str) {
                    TokenFragment::new(TokenType::FloatLit, &float_str)
                } else {
                    TokenFragment::new(TokenType::Error(InvalidNumber), &float_str)
                }
            } else if fractional_str.len() == 0 && fractional_str.len() > 0 {
                let invalid_float_str = format!("{}.e{}", &whole_str, &exponent_str);
                TokenFragment::new(TokenType::Error(InvalidNumber), &invalid_float_str)
            } else
            // fractional_str.len > 0 && fractional_str.len() > 0
            {
                let float_str = format!("{}.{}e{}", &whole_str, &fractional_str, &exponent_str);
                if TokenType::FloatLit.str_repr().is_match(&float_str) {
                    TokenFragment::new(TokenType::FloatLit, &float_str)
                } else {
                    TokenFragment::new(TokenType::Error(InvalidNumber), &float_str)
                }
            }
        } else {
            if TokenType::IntegerLit.str_repr().is_match(&whole_str) {
                TokenFragment::new(TokenType::IntegerLit, &whole_str)
            } else {
                TokenFragment::new(TokenType::Error(InvalidIdentifier), &whole_str)
            }
        };
    }

    /// Parses an input string into an operator or punctuation based token.
    /// If the input is not a well formed token fragment, returns an Error token fragment.
    /// # Arguments
    /// * `input_fragment` - A string slice to parse. Never starts with a letter or digit.
    /// # Outputs
    /// * A `TokenFragment`
    fn parse_op_or_punct<'a>(&mut self, input_fragment: &str) -> TokenFragment {
        let first_char: char = input_fragment.chars().next().unwrap();
        todo!()
    }

    fn parse_string(&mut self, input_fragment: &str) -> TokenFragment {
        let mut str_literal: String = input_fragment
            .chars()
            .take(1)
            .take_while(|c: &char| *c != '"')
            .collect::<String>();
        return if input_fragment.as_bytes()[str_literal.len()] as char != '"' {
            TokenFragment::new(TokenType::Error(InvalidString), &str_literal)
        } else {
            str_literal.push_str("\"");
            if TokenType::StringLit.str_repr().is_match(&str_literal) {
                TokenFragment::new(TokenType::StringLit, &str_literal)
            } else {
                TokenFragment::new(TokenType::Error(InvalidString), &str_literal)
            }
        };
    }
}

impl LexerAnalyzer for MyLexerAnalyzer {
    type TokenOutput = Token;

    fn back(&mut self) {
        assert!(self.idx > 0);
        self.idx -= 1;
    }

    fn back_n(&mut self, n: usize) {
        assert!(self.idx - n >= 0);
        self.idx -= n;
    }

    fn forward(&mut self) {
        self.idx += 1;
    }

    fn forward_n(&mut self, n: usize) {
        self.idx += n;
    }

    fn peek(&self, input: &str) -> Option<char> {
        Some(input.as_bytes()[self.idx] as char) //todo
    }

    fn peek_n(&self, input: &str, n: usize) -> Option<char> {
        Some(input.as_bytes()[self.idx + n] as char) //todo
    }

    fn next_char(&mut self, input: &str) -> Option<char> {
        let c = input.as_bytes()[self.idx]; //todo
        self.idx += 1;
        Some(c as char)
    }

    fn next_token(&mut self, input: &str) -> Option<Self::TokenOutput> {
        self.skip_whitespace(input);

        let first_char: char = self.peek(input)?;
        let input_fragment = input.get(self.idx..)?; //.replace(crate::token::LINE_ENDINGS, "");

        return if first_char.is_ascii_alphabetic() {
            // Probably a keyword or an identifier
            let token_fragment = self.parse_kw_or_id(input_fragment);
            self.forward_n(token_fragment.lexeme.len());
            Some(Token::new(token_fragment, todo!("line number")))
        } else if first_char.is_ascii_digit() {
            // Probably a number (int or float)
            let token_fragment = self.parse_number(input_fragment);
            self.forward_n(token_fragment.lexeme.len());
            Some(Token::new(token_fragment, todo!("line number")))
        } else if utils::is_valid_character(first_char) {
            // Probably a punctuation token, operator or comment
            let token_fragment = self.parse_op_or_punct(input_fragment);
            self.forward_n(token_fragment.lexeme.len());
            Some(Token::new(token_fragment, todo!("line number")))
        } else if first_char == '"' {
            // Probably a string literal
            let token_fragment = self.parse_string(input_fragment);
            self.forward_n(token_fragment.lexeme.len());
            Some(Token::new(token_fragment, todo!("line number")))
        } else {
            let c = &*self.next_char(input).unwrap().to_string();
            Some(Token::new(
                TokenFragment::new(TokenType::Error(InvalidCharacter), c),
                todo!("line number"),
            ))
        };
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

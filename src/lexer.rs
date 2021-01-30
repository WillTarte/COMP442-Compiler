use crate::token::InvalidTokenType::{InvalidCharacter, InvalidIdentifier, InvalidNumber};
use crate::token::{Token, TokenFragment, TokenType};
use std::path::Path;
use crate::utils;

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

    /// Returns the next token without advancing the lexer
    fn peek_token(&mut self, input: &str) -> Option<Self::TokenOutput>;

    /// Returns the next token, advancing the lexer
    fn next_token(&mut self, input: &str) -> Option<Self::TokenOutput>;

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

    fn parse_number(&mut self, input_fragment: &str) -> TokenFragment {

        let mut num_str: &str = "";

        let whole_str: String = input_fragment.chars().take_while(|c: &char| c.is_ascii_digit()).collect();
        if input_fragment.as_bytes()[whole_str.len()].is_ascii_alphabetic() || !TokenType::IntegerLit.str_repr().is_match(&whole_str)
        {
            return TokenFragment::new(TokenType::Error(InvalidNumber), &whole_str);
        }
        else
        {
            let fractional_str: String = format!(".{}", input_fragment.chars().take_while(|c: &char| *c != '.').skip(1).take_while(|c: &char| c.is_ascii_digit()).collect::<String>()); //.integer
            let exponent_str: String = format!("e{}", input_fragment.chars().skip_while(|c: &char| *c != 'e').skip(1).take_while(|c: &char| c.is_ascii_digit() || *c == '+' || *c == '-').collect::<String>()); //e(+|-)integer

            if fractional_str.len() == 1 && exponent_str.len() == 1
            {
                return TokenFragment::new(TokenType::IntegerLit, &whole_str);
            }
            else if fractional_str.len() > 1 && exponent_str.len() == 1
            {
                return todo!();
            }
            return todo!();
        }
    }

    fn parse_op_or_punct<'a>(&mut self, input_fragment: &str) -> TokenFragment {
        todo!()
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

        let first_char: char = self.peek(input)?;
        let input_fragment = input
            .chars()
            .skip(self.idx)
            .take_while(|c: &char| !c.is_ascii_whitespace())
            .collect::<String>();

        return if first_char.is_ascii_alphabetic() {
            let token_fragment = self.parse_kw_or_id(&input_fragment);
            self.forward_n(token_fragment.lexeme.len());
            Some(Token::new(token_fragment, todo!("line number")))
        } else if first_char.is_ascii_digit() {
            let token_fragment = self.parse_number(&input_fragment);
            todo!()
        } else if utils::is_valid_character(first_char) {
            // todo can be comment, string, keyword or operator
            let token_fragment = self.parse_op_or_punct(&input_fragment);
            Some(Token::new(token_fragment, todo!("line number")))
        } else {
            let c = &*self.next_char(input).unwrap().to_string();
            Some(Token::new(
                TokenFragment::new(TokenType::Error(InvalidCharacter), c),
                todo!("line number"),
            ))
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

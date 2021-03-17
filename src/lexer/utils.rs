//! Utilities for the compiler

use lazy_static::lazy_static;
use regex::Regex;

#[doc(hidden)]
#[cfg(windows)]
pub(crate) const LINE_ENDINGS: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDINGS: &str = "\n";

lazy_static! {
    #[doc(hidden)]
    pub static ref LINE_ENDINGS_RE: Regex = Regex::new("(\r\n|\n)").unwrap();
}

///Contains utility methods used by the lexer implementation [MyLexerAnalyzer](crate::lexer::MyLexerAnalyzer)
pub mod lexer {
    use crate::lexer::token::InvalidTokenType::{
        InvalidCharacter, InvalidIdentifier, InvalidMultilineComment, InvalidNumber, InvalidString,
    };
    use crate::lexer::token::{Token, TokenFragment, TokenType};

    const VALID_CHARS: &str = "=<>+-*/|&!?(){}[];,.:";

    /// Checks if a given character is part of some valid characters defined in the lexical specfification
    pub(crate) fn is_valid_character(c: char) -> bool {
        for vc in VALID_CHARS.chars() {
            if c == vc {
                return true;
            }
        }
        return false;
    }

    /// Parses an input string into a keyword or an identifier.
    /// If the input is neither, returns an Error token fragment.
    /// # Arguments
    /// * `input_fragment` - A string slice to parse. Should always start with a letter
    /// # Outputs
    /// * A `TokenFragment`
    pub(crate) fn parse_kw_or_id(input_fragment: &str) -> TokenFragment {
        //FIXME: leading underscore for ID
        let word = input_fragment
            .chars()
            .take_while(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
            .collect::<String>();

        for kw_type in &*crate::lexer::token::KEYWORD_TOKENS {
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
    pub(crate) fn parse_number(input_fragment: &str) -> TokenFragment {
        if input_fragment.is_empty() {
            return TokenFragment::new(TokenType::Error(InvalidNumber), "");
        }
        if (input_fragment.as_bytes()[0] as char).is_ascii_alphabetic() {
            return parse_kw_or_id(input_fragment);
        }
        // whole part - nonzero digit* | zero
        let whole_str: String = input_fragment
            .chars()
            .take_while(|c: &char| c.is_ascii_digit())
            .collect::<String>();

        if whole_str.len() < input_fragment.len()
            && input_fragment.as_bytes()[whole_str.len()] as char == '.'
        {
            let fractional_str: String = input_fragment
                .chars()
                .skip_while(|c| *c != '.')
                .skip(1)
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();

            let mut float_str = format!("{}.{}", &whole_str, &fractional_str);

            if float_str.len() < input_fragment.len()
                && input_fragment.as_bytes()[float_str.len()] as char == 'e'
            {
                float_str.push_str("e");
                if float_str.len() < input_fragment.len()
                    && input_fragment.as_bytes()[float_str.len()] as char == '+'
                {
                    float_str.push_str("+");
                } else if float_str.len() < input_fragment.len()
                    && input_fragment.as_bytes()[float_str.len()] as char == '-'
                {
                    float_str.push_str("-");
                }
                float_str.push_str(
                    &input_fragment
                        .chars()
                        .skip(float_str.len())
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>(),
                );
            }

            if TokenType::FloatLit.str_repr().is_match(&float_str) {
                return TokenFragment::new(TokenType::FloatLit, &float_str);
            } else {
                return TokenFragment::new(TokenType::Error(InvalidNumber), &float_str);
            }
        } else {
            if TokenType::IntegerLit.str_repr().is_match(&whole_str) {
                return TokenFragment::new(TokenType::IntegerLit, &whole_str);
            } else {
                return TokenFragment::new(TokenType::Error(InvalidNumber), &whole_str);
            }
        }
    }

    /// Parses an input string into an operator or punctuation based token.
    /// If the input is not a well formed token fragment, returns an Error token fragment.
    /// # Arguments
    /// * `input_fragment` - A string slice to parse. Never starts with a letter or digit.
    /// # Outputs
    /// * A `TokenFragment`
    pub(crate) fn parse_op_or_punct(input_fragment: &str) -> TokenFragment {
        if input_fragment.len() < 2 {
            return TokenFragment::from_lexeme(input_fragment);
        }
        let two_chars: [char; 2] = [
            input_fragment.as_bytes()[0] as char,
            input_fragment.as_bytes()[1] as char,
        ];
        return match two_chars[0] {
            '=' => {
                // = or ==
                if two_chars[1] == '=' {
                    TokenFragment::new(TokenType::EqEq, "==")
                } else {
                    TokenFragment::new(TokenType::Assignment, "=")
                }
            }
            '<' => {
                // < or <= or <>
                if two_chars[1] == '=' {
                    TokenFragment::new(TokenType::LessEqualThan, "<=")
                } else if two_chars[1] == '>' {
                    TokenFragment::new(TokenType::NotEq, "<>")
                } else {
                    TokenFragment::new(TokenType::LessThan, "<")
                }
            }
            '>' => {
                // > or >=
                if two_chars[1] == '=' {
                    TokenFragment::new(TokenType::GreaterEqualThan, ">=")
                } else {
                    TokenFragment::new(TokenType::GreaterThan, ">")
                }
            }
            '/' => {
                if two_chars[1] == '/' {
                    let comment: String = input_fragment
                        .chars()
                        .take_while(|c: &char| *c != '\n' && *c != '\r')
                        .collect::<String>();
                    TokenFragment::new(TokenType::LineComment, &comment)
                } else if two_chars[1] == '*' {
                    match TokenType::MultilineComment.str_repr().find(input_fragment) {
                        None => TokenFragment::new(
                            TokenType::Error(InvalidMultilineComment),
                            input_fragment,
                        ),
                        Some(m) => match m.start() {
                            0 => {
                                let comment =
                                    input_fragment.chars().take(m.end()).collect::<String>();
                                return TokenFragment::new(TokenType::MultilineComment, &comment);
                            }
                            _ => {
                                panic!("Matched a multiline comment that is not at the beginning of the input")
                            }
                        },
                    }
                } else {
                    TokenFragment::new(TokenType::Div, "/")
                }
            }
            ':' => {
                // : or ::
                if two_chars[1] == ':' {
                    TokenFragment::new(TokenType::DoubleColon, "::")
                } else {
                    TokenFragment::new(TokenType::Colon, ":")
                }
            }
            '+' | '-' | '*' | '|' | '&' | '!' | '?' | ';' | ',' | '.' | '(' | ')' | '{' | '}'
            | '[' | ']' => TokenFragment::from_lexeme(&two_chars[0].to_string()),
            _ => TokenFragment::new(
                TokenType::Error(InvalidCharacter),
                &two_chars[0].to_string(),
            ),
        };
    }

    /// Parses an input string into a string literal
    /// If the input is not a well formed token fragment, returns an Error token fragment.
    /// # Arguments
    /// * `input_fragment` - A string slice to parse. Should always start with a `"`.
    /// # Outputs
    /// * A `TokenFragment`
    pub(crate) fn parse_string(input_fragment: &str) -> TokenFragment {
        if input_fragment.as_bytes()[0] as char != '"' {
            panic!("Tried to parse string but input didn't start with a quotation mark");
        } else {
            match TokenType::StringLit.str_repr().find(input_fragment) {
                None => {
                    return TokenFragment::new(TokenType::Error(InvalidString), input_fragment);
                }
                Some(m) => TokenFragment::new(TokenType::StringLit, m.as_str()),
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_error_token(token: Token) -> bool {
        match token.token_type() {
            TokenType::Error(_) => true,
            _ => false,
        }
    }
}

/// Utilities to serialize a lexer's output
pub mod lexer_serialize {
    use crate::lexer::lexer::LexerAnalyzer;
    use crate::lexer::token::{Token, TokenType};
    use crate::lexer::utils::LINE_ENDINGS;
    use std::fs::OpenOptions;
    use std::io;
    use std::io::{BufWriter, Write};

    pub fn serialize_lexer_to_file<T>(mut lexer: T, file_name: &str) -> io::Result<()>
    where
        T: LexerAnalyzer<TokenOutput = Token>,
    {
        let lextokens_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("{}.outlextokens", file_name))?;
        let mut buf_token_write = BufWriter::new(lextokens_file);

        let lexerrors_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(format!("{}.outlexerrors", file_name))?;
        let mut buf_err_write = BufWriter::new(lexerrors_file);
        let mut token_errors: Vec<Token> = Vec::new();

        let mut current_line_num = 1;
        let mut line: String = String::new();

        while let Some(token) = lexer.next_token() {
            if current_line_num != token.line_num {
                buf_token_write.write(line.as_bytes())?;
                line.clear();
                buf_token_write.write(LINE_ENDINGS.as_bytes())?;
                current_line_num = token.line_num;
            }

            line.push_str(&format!(
                r"[{:?}, {}, {}] ",
                token.token_type(),
                token.lexeme(),
                token.line_num
            ));

            if token.is_err() {
                token_errors.push(token);
            }
        }

        if line.len() > 0 {
            buf_token_write.write(line.as_bytes())?;
            buf_token_write.flush()?;
            line.clear();
        }

        if token_errors.len() > 0 {
            for token in token_errors {
                match token.token_type() {
                    TokenType::Error(err) => {
                        buf_err_write.write(
                            format!(
                                "Lexical error: {}: {}: line {}.{}",
                                err.to_string(),
                                token.lexeme(),
                                token.line_num,
                                LINE_ENDINGS
                            )
                            .as_bytes(),
                        )?;
                        buf_err_write.flush()?;
                    }
                    _ => panic!("Trying to write valid token to as error"),
                }
            }
        }

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::token::InvalidTokenType::{
        InvalidCharacter, InvalidIdentifier, InvalidMultilineComment, InvalidNumber, InvalidString,
    };
    use crate::lexer::token::{TokenFragment, TokenType};
    use crate::lexer::utils::lexer::{
        is_valid_character, parse_kw_or_id, parse_number, parse_op_or_punct, parse_string,
    };

    #[test]
    fn test_is_valid_character() {
        assert!(is_valid_character('='));
        assert!(is_valid_character('<'));
        assert!(is_valid_character('>'));
        assert!(is_valid_character('+'));
        assert!(is_valid_character('-'));
        assert!(is_valid_character('*'));
        assert!(is_valid_character('/'));
        assert!(is_valid_character('('));
        assert!(is_valid_character(')'));
        assert!(is_valid_character('{'));
        assert!(is_valid_character('}'));
        assert!(is_valid_character('['));
        assert!(is_valid_character(']'));
        assert!(is_valid_character(';'));
        assert!(is_valid_character(','));
        assert!(is_valid_character('.'));
        assert!(is_valid_character(':'));
    }

    #[test]
    fn test_parse_kw_or_id() {
        assert_eq!(
            parse_kw_or_id("abc"),
            TokenFragment::new(TokenType::Id, "abc")
        );
        assert_eq!(
            parse_kw_or_id("abc123"),
            TokenFragment::new(TokenType::Id, "abc123")
        );
        assert_eq!(
            parse_kw_or_id("123abc123"),
            TokenFragment::new(TokenType::Error(InvalidIdentifier), "123abc123")
        );
        assert_eq!(
            parse_kw_or_id("abc_123"),
            TokenFragment::new(TokenType::Id, "abc_123")
        );
        assert_eq!(
            parse_kw_or_id("_abc123"),
            TokenFragment::new(TokenType::Error(InvalidIdentifier), "_abc123")
        );
        assert_eq!(
            parse_kw_or_id("abc+3"),
            TokenFragment::new(TokenType::Id, "abc")
        );
        assert_eq!(
            parse_kw_or_id("abc@"),
            TokenFragment::new(TokenType::Id, "abc")
        );
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(
            parse_number("0"),
            TokenFragment::new(TokenType::IntegerLit, "0")
        );
        assert_eq!(
            parse_number("123"),
            TokenFragment::new(TokenType::IntegerLit, "123")
        );
        assert_eq!(
            parse_number("12300"),
            TokenFragment::new(TokenType::IntegerLit, "12300")
        );
        assert_eq!(
            parse_number("00123"),
            TokenFragment::new(TokenType::Error(InvalidNumber), "00123")
        );
        assert_eq!(
            parse_number("0.0"),
            TokenFragment::new(TokenType::FloatLit, "0.0")
        );
        assert_eq!(
            parse_number("0.0123002"),
            TokenFragment::new(TokenType::FloatLit, "0.0123002")
        );
        assert_eq!(
            parse_number("0.012300200"),
            TokenFragment::new(TokenType::Error(InvalidNumber), "0.012300200")
        );
        assert_eq!(
            parse_number("abc123"),
            TokenFragment::new(TokenType::Id, "abc123")
        );
        assert_eq!(
            parse_number("0.0e0"),
            TokenFragment::new(TokenType::FloatLit, "0.0e0")
        );
        assert_eq!(
            parse_number("0.0e+0"),
            TokenFragment::new(TokenType::FloatLit, "0.0e+0")
        );
        assert_eq!(
            parse_number("0.0e-0"),
            TokenFragment::new(TokenType::FloatLit, "0.0e-0")
        );
        assert_eq!(
            parse_number("0.0e000"),
            TokenFragment::new(TokenType::Error(InvalidNumber), "0.0e000")
        );
        assert_eq!(
            parse_number("0.0e1230"),
            TokenFragment::new(TokenType::FloatLit, "0.0e1230")
        );
        assert_eq!(
            parse_number("0.0e-1230"),
            TokenFragment::new(TokenType::FloatLit, "0.0e-1230")
        );
    }

    #[test]
    fn test_parse_op_or_punct() {
        assert_eq!(
            parse_op_or_punct("@"),
            TokenFragment::new(TokenType::Error(InvalidCharacter), "@")
        );
        assert_eq!(
            parse_op_or_punct("="),
            TokenFragment::new(TokenType::Assignment, "=")
        );
        assert_eq!(
            parse_op_or_punct("=="),
            TokenFragment::new(TokenType::EqEq, "==")
        );
        assert_eq!(
            parse_op_or_punct("<"),
            TokenFragment::new(TokenType::LessThan, "<")
        );
        assert_eq!(
            parse_op_or_punct("<="),
            TokenFragment::new(TokenType::LessEqualThan, "<=")
        );
        assert_eq!(
            parse_op_or_punct(">"),
            TokenFragment::new(TokenType::GreaterThan, ">")
        );
        assert_eq!(
            parse_op_or_punct(">="),
            TokenFragment::new(TokenType::GreaterEqualThan, ">=")
        );
        assert_eq!(
            parse_op_or_punct("<>"),
            TokenFragment::new(TokenType::NotEq, "<>")
        );
        assert_eq!(
            parse_op_or_punct("::"),
            TokenFragment::new(TokenType::DoubleColon, "::")
        );
        assert_eq!(
            parse_op_or_punct("// comment"),
            TokenFragment::new(TokenType::LineComment, "// comment")
        );
        assert_eq!(
            parse_op_or_punct("// comment \r\n more stuff"),
            TokenFragment::new(TokenType::LineComment, "// comment ")
        );
        assert_eq!(
            parse_op_or_punct("/* comment \r\n more stuff */"),
            TokenFragment::new(TokenType::MultilineComment, "/* comment \r\n more stuff */")
        );
        assert_eq!(
            parse_op_or_punct("/* unterminated block comment"),
            TokenFragment::new(
                TokenType::Error(InvalidMultilineComment),
                "/* unterminated block comment"
            )
        );
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(
            parse_string("\"This is a _ _ string literal 111\""),
            TokenFragment::new(TokenType::StringLit, "\"This is a _ _ string literal 111\"")
        );
        assert_eq!(
            parse_string("\"This is a string literal invalid char @@@/\""),
            TokenFragment::new(
                TokenType::Error(InvalidString),
                "\"This is a string literal invalid char @@@/\""
            )
        );
        assert_eq!(
            parse_string("\"This is a string literal unterminated"),
            TokenFragment::new(
                TokenType::Error(InvalidString),
                "\"This is a string literal unterminated"
            )
        );
    }
}

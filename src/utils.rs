pub mod lexer
{
    use crate::token::InvalidTokenType::{
        InvalidCharacter, InvalidIdentifier, InvalidMultilineComment, InvalidNumber, InvalidString,
    };
    use crate::token::{TokenFragment, TokenType};
    use regex::Match;

    const VALID_CHARS: &str = "=<>+-*/|&!?(){}[];,.:";

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
        let word = input_fragment
            .chars()
            .take_while(|c: &char| c.is_ascii_alphanumeric() || *c == '_')
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
    pub(crate) fn parse_number(input_fragment: &str) -> TokenFragment {
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
                .skip(whole_str.len())
                .skip_while(|c: &char| *c != '.' && *c != '\n' && *c != '\r')
                .skip(1)
                .take_while(|c: &char| c.is_ascii_digit())
                .collect::<String>();
            // exponent part (optional) - e [+|-] nonzero digit* | zero
            let exponent_str: String = input_fragment
                .chars()
                .skip(whole_str.len() + fractional_str.len())
                .skip_while(|c: &char| *c != 'e' && *c != '\n' && *c != '\r')
                .skip(1)
                .take_while(|c: &char| c.is_ascii_digit() || *c == '+' || *c == '-')
                .collect::<String>();

            if fractional_str.len() == 0 && exponent_str.len() == 0 {
                TokenFragment::new(TokenType::Error(InvalidNumber), &format!("{}", &whole_str))
            } else if fractional_str.len() > 0 && exponent_str.len() == 0 {
                let float_str: String = format!("{}.{}", &whole_str, &fractional_str);
                if TokenType::FloatLit.str_repr().is_match(&float_str) {
                    TokenFragment::new(TokenType::FloatLit, &float_str)
                } else {
                    TokenFragment::new(TokenType::Error(InvalidNumber), &float_str)
                }
            } else if fractional_str.len() == 0 && exponent_str.len() > 0 {
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
    pub(crate) fn parse_op_or_punct(input_fragment: &str) -> TokenFragment {
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
                //FIXME parses too much of multiline (e.g. 2 in a row)
                // single line comment or division or multiline comment
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
                                let comment = input_fragment.chars().take(m.end()).collect::<String>();
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
            '+' | '-' | '*' | '|' | '&' | '!' | '?' | ';' | ',' | '.' | '(' | ')' | '{' | '}' | '['
            | ']' => TokenFragment::new(
                TokenType::from_lexeme(&two_chars[0].to_string()),
                &two_chars[0].to_string(),
            ),
            _ => TokenFragment::new(
                TokenType::Error(InvalidCharacter),
                &two_chars[0].to_string(),
            ),
        };
    }

    pub(crate) fn parse_string(input_fragment: &str) -> TokenFragment {
        if input_fragment.as_bytes()[0] as char != '"' {
            panic!("Tried to parse string but input didn't start with a quotation mark");
        } else {
            match TokenType::StringLit.str_repr().find(input_fragment) {
                None => {
                    panic!("Tried to parse string from the beginning of the input")
                }
                Some(m) => TokenFragment::new(TokenType::StringLit, m.as_str()),
            }
        }
    }
}

pub mod lexer_serialize
{
    use crate::lexer::LexerAnalyzer;
    use std::io;
    use std::fs::{File, OpenOptions};
    use std::io::{BufWriter, Write};
    use crate::token::{TokenType, Token, LINE_ENDINGS};

    pub fn serialize_lexer_to_file(mut lexer: Box<dyn LexerAnalyzer<TokenOutput=Token>>, file_name: &str) -> io::Result<()>
    {
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(file_name)?;
        let mut buf_write = BufWriter::new(file);

        let mut current_line_num = 1;
        let mut line: String = String::new();

        while let Some(token) = lexer.next_token()
        {
            if current_line_num != token.line_num
            {
                buf_write.write(line.as_bytes());
                line.clear();
                for i in 0..(token.line_num - current_line_num) {
                    buf_write.write(LINE_ENDINGS.as_bytes());
                }
                current_line_num = token.line_num;
            }

            line.push_str(&format!("[{:?}, {}, {}]", token.token_fragment.token_type, token.token_fragment.lexeme, token.line_num));
        }

        if line.len() > 0
        {
            buf_write.write(line.as_bytes());
            buf_write.flush();
            line.clear();
        }
        return Ok(());
    }
}
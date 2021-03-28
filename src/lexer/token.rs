//! Tokens used by the compiler

use lazy_static::lazy_static;
use regex::Regex;

use crate::lexer::token::InvalidTokenType::InvalidCharacter;
use crate::lexer::token_regex::*;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

lazy_static! {
    #[doc(hidden)]
    pub static ref KEYWORD_TOKENS: Vec<TokenType> = vec![
        TokenType::If,
        TokenType::Then,
        TokenType::Else,
        TokenType::IntegerType,
        TokenType::FloatType,
        TokenType::StringType,
        TokenType::Void,
        TokenType::Public,
        TokenType::Private,
        TokenType::Func,
        TokenType::Var,
        TokenType::Class,
        TokenType::While,
        TokenType::Read,
        TokenType::Write,
        TokenType::Return,
        TokenType::Main,
        TokenType::Inherits,
        TokenType::Break,
        TokenType::Continue
    ];
    #[doc(hidden)]
    pub static ref OP_PUNCT_TOKENS: Vec<TokenType> = vec![
        TokenType::EqEq,
        TokenType::NotEq,
        TokenType::GreaterThan,
        TokenType::LessThan,
        TokenType::GreaterEqualThan,
        TokenType::LessEqualThan,
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Mult,
        TokenType::Div,
        TokenType::Assignment,
        TokenType::Or,
        TokenType::And,
        TokenType::Bang,
        TokenType::Question,
        TokenType::OpenParen,
        TokenType::CloseParen,
        TokenType::OpenCurly,
        TokenType::CloseCurly,
        TokenType::OpenSquare,
        TokenType::CloseSquare,
        TokenType::SemiColon,
        TokenType::Comma,
        TokenType::LineComment,
        TokenType::MultilineComment
    ];
    #[doc(hidden)]
    pub static ref ALL_TOKEN_TYPES: Vec<TokenType> = vec![
        TokenType::Id,
        TokenType::IntegerLit,
        TokenType::FloatLit,
        TokenType::StringLit,
        TokenType::If,
        TokenType::Then,
        TokenType::Else,
        TokenType::IntegerType,
        TokenType::FloatType,
        TokenType::StringType,
        TokenType::Void,
        TokenType::Public,
        TokenType::Private,
        TokenType::Func,
        TokenType::Var,
        TokenType::Class,
        TokenType::While,
        TokenType::Read,
        TokenType::Write,
        TokenType::Return,
        TokenType::Main,
        TokenType::Inherits,
        TokenType::Break,
        TokenType::Continue,
        TokenType::EqEq,
        TokenType::NotEq,
        TokenType::GreaterThan,
        TokenType::LessThan,
        TokenType::GreaterEqualThan,
        TokenType::LessEqualThan,
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Mult,
        TokenType::Div,
        TokenType::Assignment,
        TokenType::Or,
        TokenType::Period,
        TokenType::And,
        TokenType::Bang,
        TokenType::Question,
        TokenType::OpenParen,
        TokenType::CloseParen,
        TokenType::OpenCurly,
        TokenType::CloseCurly,
        TokenType::OpenSquare,
        TokenType::CloseSquare,
        TokenType::SemiColon,
        TokenType::Comma
    ];
}

/// Represents the different types of tokens
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TokenType {
    // Basic
    Id,
    IntegerLit,
    FloatLit,
    StringLit,

    // Operators & punctuation
    /// ==
    EqEq,
    /// <>
    NotEq,
    /// >
    GreaterThan,
    /// <
    LessThan,
    /// >=
    GreaterEqualThan,
    /// <=
    LessEqualThan,
    /// \+
    Plus,
    /// \-
    Minus,
    /// \*
    Mult,
    /// /
    Div,
    /// =
    Assignment,
    /// |
    Or,
    /// &
    And,
    /// !
    Bang,
    /// ?
    Question,
    /// (
    OpenParen,
    /// )
    CloseParen,
    /// {
    OpenCurly,
    /// }
    CloseCurly,
    /// [
    OpenSquare,
    /// ]
    CloseSquare,
    /// ;
    SemiColon,
    /// ,
    Comma,
    /// .
    Period,
    /// :
    Colon,
    /// ::
    DoubleColon,

    // reserved keywords
    If,
    Then,
    Else,
    IntegerType,
    FloatType,
    StringType,
    Void,
    Public,
    Private,
    Func,
    Var,
    Class,
    While,
    Read,
    Write,
    Return,
    Main,
    Inherits,
    Break,
    Continue,

    // comments
    /// //
    LineComment,
    /// /* ~ */
    MultilineComment,

    Error(InvalidTokenType),
}

impl TokenType {
    /// Returns a [Regex](regex::Regex) representing this token type
    pub fn str_repr(&self) -> &Regex {
        match self {
            TokenType::Id => &*ID,
            TokenType::IntegerLit => &*INT_LIT,
            TokenType::FloatLit => &*FLOAT_LIT,
            TokenType::StringLit => &*STRING_LIT,
            TokenType::EqEq => &*EQEQ,
            TokenType::NotEq => &*NOTEQ,
            TokenType::GreaterThan => &*GT,
            TokenType::LessThan => &*LT,
            TokenType::GreaterEqualThan => &*GTEQ,
            TokenType::LessEqualThan => &*LTEQ,
            TokenType::Plus => &*PLUS,
            TokenType::Minus => &*MINUS,
            TokenType::Mult => &*MULT,
            TokenType::Div => &*FSLASH,
            TokenType::Assignment => &*ASSIGN,
            TokenType::Or => &*OR,
            TokenType::And => &*AND,
            TokenType::Bang => &*BANG,
            TokenType::Question => &*QUESTION,
            TokenType::OpenParen => &*OPENPAREN,
            TokenType::CloseParen => &*CLOSEPAREN,
            TokenType::OpenCurly => &*OPENCURLY,
            TokenType::CloseCurly => &*CLOSECURLY,
            TokenType::OpenSquare => &*OPENSQUARE,
            TokenType::CloseSquare => &*CLOSESQUARE,
            TokenType::SemiColon => &*SEMICOLON,
            TokenType::Comma => &*COMMA,
            TokenType::Period => &*PERIOD,
            TokenType::Colon => &*COLON,
            TokenType::DoubleColon => &*DBCOLON,
            TokenType::If => &*IF,
            TokenType::Then => &*THEN,
            TokenType::Else => &*ELSE,
            TokenType::IntegerType => &*INT_T,
            TokenType::FloatType => &*FLOAT_T,
            TokenType::StringType => &*STRING_T,
            TokenType::Void => &*VOID,
            TokenType::Public => &*PUBLIC,
            TokenType::Private => &*PRIVATE,
            TokenType::Func => &*FUNC,
            TokenType::Var => &*VAR,
            TokenType::Class => &*CLASS,
            TokenType::While => &*WHILE,
            TokenType::Read => &*READ,
            TokenType::Write => &*WRITE,
            TokenType::Return => &*RETURN,
            TokenType::Main => &*MAIN,
            TokenType::Inherits => &*INHERITS,
            TokenType::Break => &*BREAK,
            TokenType::Continue => &*CONTINUE,
            TokenType::Error(_) => &*ERROR,
            TokenType::LineComment => &*LINE_COMMENT,
            TokenType::MultilineComment => &*MULTILINE_COMMENT,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Token Type: {:?}", self)
    }
}

/// Reprensents the different types of invalid tokens
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum InvalidTokenType {
    InvalidIdentifier,
    InvalidNumber,
    InvalidString,
    InvalidCharacter,
    InvalidMultilineComment,
}

impl ToString for InvalidTokenType {
    fn to_string(&self) -> String {
        match self {
            InvalidTokenType::InvalidIdentifier => String::from("Invalid identifier"),
            InvalidTokenType::InvalidNumber => String::from("Invalid number"),
            InvalidTokenType::InvalidCharacter => String::from("Invalid character"),
            InvalidTokenType::InvalidString => String::from("Invalid string"),
            InvalidTokenType::InvalidMultilineComment => String::from("Invalid multiline comment"),
        }
    }
}

/// A TokenFragment is a [TokenType] - lexeme pair
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenFragment {
    pub token_type: TokenType,
    pub lexeme: String,
}

impl TokenFragment {
    pub(crate) fn new(t_type: TokenType, lexeme: &str) -> Self {
        TokenFragment {
            token_type: t_type,
            lexeme: lexeme.to_owned(),
        }
    }

    /// Tries to match a given lexeme with each [TokenType]'s Regex.
    /// Returns an error if no match was found.
    pub(crate) fn from_lexeme(lexeme: &str) -> Self {
        for token_t in &*ALL_TOKEN_TYPES {
            if token_t.str_repr().is_match(lexeme) {
                return Self::new(*token_t, lexeme);
            }
        }
        return Self::new(TokenType::Error(InvalidCharacter), lexeme);
    }
}

/// Represents a full token, which includes a [TokenFragment] and a line number from the input.
#[derive(Clone, Eq, Debug, PartialEq)]
pub struct Token {
    token_fragment: TokenFragment,
    line_num: usize,
}

impl Token {
    pub(crate) fn new(tkf: TokenFragment, ln: usize) -> Self {
        Token {
            token_fragment: tkf,
            line_num: ln,
        }
    }

    /// Returns true if this token is an error token
    pub(crate) fn is_err(&self) -> bool {
        match self.token_fragment.token_type {
            TokenType::Error(_) => true,
            _ => false,
        }
    }

    /// Returns this token's [TokenType]
    pub(crate) fn token_type(&self) -> TokenType {
        return self.token_fragment.token_type;
    }

    /// Returns this token's lexeme
    pub(crate) fn lexeme(&self) -> &str {
        return self.token_fragment.lexeme.as_ref();
    }

    pub fn line_num(&self) -> usize
    {
        self.line_num
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Token: {}", self.lexeme())
    }
}

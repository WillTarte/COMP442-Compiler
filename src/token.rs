use crate::token_regex::*;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
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
        TokenType::ForwardSlash,
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
        TokenType::Comma
    ];
}

#[cfg(windows)]
const LINE_ENDINGS: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDINGS: &str = "\n";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TokenType {
    // Basic
    Id,
    IntegerLit,
    FloatLit,
    StringLit,

    // Operators & punctuation
    EqEq,
    NotEq,
    GreaterThan,
    LessThan,
    GreaterEqualThan,
    LessEqualThan,
    Plus,
    Minus,
    Mult,
    ForwardSlash,
    Assignment,
    Or,
    And,
    Bang,
    Question,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    SemiColon,
    Comma,
    Period,
    Colon,
    DoubleColon,
    Quote,

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
    LineComment,
    OpenMultiLineComment,
    CloseMultiLineComment,

    Error(InvalidTokenType),
}

impl TokenType {
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
            TokenType::ForwardSlash => &*FSLASH,
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
            TokenType::Quote => &*QUOTE,
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
            TokenType::OpenMultiLineComment => &*OPEN_MULTILINE_COMMENT,
            TokenType::CloseMultiLineComment => &*CLOSE_MULTILINE_COMMENT,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InvalidTokenType {
    InvalidIdentifier,
    InvalidNumber,
    InvalidCharacter,
}

impl ToString for InvalidTokenType {
    fn to_string(&self) -> String {
        match self {
            InvalidTokenType::InvalidIdentifier => String::from("Invalid identifier"),
            InvalidTokenType::InvalidNumber => String::from("Invalid number"),
            InvalidTokenType::InvalidCharacter => String::from("Invalid character"),
        }
    }
}

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
}

pub struct Token {
    pub token: TokenFragment,
    pub line_num: usize,
}

impl Token {
    pub(crate) fn new(tk: TokenFragment, ln: usize) -> Self {
        Token {
            token: tk,
            line_num: ln,
        }
    }
}

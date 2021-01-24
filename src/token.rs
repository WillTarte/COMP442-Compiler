type Span = (usize, usize);

#[cfg(windows)]
const LINE_ENDINGS: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDINGS: &str = "\n";

const EQEQ: &str = "==";
const NOTEQ: &str = "<>";
const LT: &str = "<";
const GT: &str = ">";
const LTEQ: &str = "<=";
const GTEQ: &str = ">=";
const PLUS: &str = "+";
const MINUS: &str = "-";
const MULT: &str = "*";
const FSLASH: &str = "/";
const ASSIGN: &str = "=";
const OR: &str = "|";
const AND: &str = "&";
const BANG: &str = "!";
const QUESTION: &str = "?";
const OPENPAREN: &str = "(";
const CLOSEPAREN: &str = ")";
const OPENCURLY: &str = "{";
const CLOSECURLY: &str = "}";
const OPENSQUARE: &str = "[";
const CLOSESQUARE: &str = "]";
const SEMICOLON: &str = ";";
const COMMA: &str = ",";
const PERIOD: &str = ".";
const COLON: &str = ":";
const DBCOLON: &str = "::";
const QUOTE: &str = "\"";

const IF: &str = "if";
const THEN: &str = "then";
const ELSE: &str = "else";
const INT_T: &str = "integer";
const FLOAT_T: &str = "float";
const STRING_T: &str = "string";
const VOID: &str = "void";
const PUBLIC: &str = "public";
const PRIVATE: &str = "private";
const FUNC: &str = "func";
const VAR: &str = "var";
const CLASS: &str = "class";
const WHILE: &str = "while";
const READ: &str = "read";
const WRITE: &str = "write";
const RETURN: &str = "return";
const MAIN: &str = "main";
const INHERITS: &str = "inherits";
const BREAK: &str = "break";
const CONTINUE: &str = "continue";

const LINE_COMMENT: &str = "//";
const OPEN_MULTILINE_COMMENT: &str = "/*";
const CLOSE_MULTILINE_COMMENT: &str = "*/";

#[allow(dead_code)]
pub enum TokenType {
    // Basic
    Id,
    IntegerLit,
    FloatLit,
    StringLit,

    // Operators, punctuation and reserved keywords
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

    LineComment,
    OpenMultiLineComment,
    CloseMultiLineComment,

    Error,
}

impl TokenType {
    pub fn str_repr(&self) -> &str {
        match self {
            TokenType::Id => todo!(),
            TokenType::IntegerLit => todo!(),
            TokenType::FloatLit => todo!(),
            TokenType::StringLit => todo!(),
            TokenType::EqEq => EQEQ,
            TokenType::NotEq => NOTEQ,
            TokenType::GreaterThan => GT,
            TokenType::LessThan => LT,
            TokenType::GreaterEqualThan => GTEQ,
            TokenType::LessEqualThan => LTEQ,
            TokenType::Plus => PLUS,
            TokenType::Minus => MINUS,
            TokenType::Mult => MULT,
            TokenType::ForwardSlash => FSLASH,
            TokenType::Assignment => ASSIGN,
            TokenType::Or => OR,
            TokenType::And => AND,
            TokenType::Bang => BANG,
            TokenType::Question => QUESTION,
            TokenType::OpenParen => OPENPAREN,
            TokenType::CloseParen => CLOSEPAREN,
            TokenType::OpenCurly => OPENCURLY,
            TokenType::CloseCurly => CLOSECURLY,
            TokenType::OpenSquare => OPENSQUARE,
            TokenType::CloseSquare => CLOSESQUARE,
            TokenType::SemiColon => SEMICOLON,
            TokenType::Comma => COMMA,
            TokenType::Period => PERIOD,
            TokenType::Colon => COLON,
            TokenType::DoubleColon => DBCOLON,
            TokenType::Quote => QUOTE,
            TokenType::If => IF,
            TokenType::Then => THEN,
            TokenType::Else => ELSE,
            TokenType::IntegerType => INT_T,
            TokenType::FloatType => FLOAT_T,
            TokenType::StringType => STRING_T,
            TokenType::Void => VOID,
            TokenType::Public => PUBLIC,
            TokenType::Private => PRIVATE,
            TokenType::Func => FUNC,
            TokenType::Var => VAR,
            TokenType::Class => CLASS,
            TokenType::While => WHILE,
            TokenType::Read => READ,
            TokenType::Write => WRITE,
            TokenType::Return => RETURN,
            TokenType::Main => MAIN,
            TokenType::Inherits => INHERITS,
            TokenType::Break => BREAK,
            TokenType::Continue => CONTINUE,
            TokenType::Error => todo!(),
            TokenType::LineComment => LINE_COMMENT,
            TokenType::OpenMultiLineComment => OPEN_MULTILINE_COMMENT,
            TokenType::CloseMultiLineComment => CLOSE_MULTILINE_COMMENT,
        }
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub location: Span,
}

impl Token {
    fn new(t_type: TokenType, lexeme: &str, span: Span) -> Self {
        Token {
            token_type: t_type,
            lexeme: lexeme.to_string(),
            location: span,
        }
    }
}

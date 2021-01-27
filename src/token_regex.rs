use regex::Regex;

//TODO fix regex
lazy_static!{
    pub static ref ID: Regex = Regex::new("[[:alpha:]]([[:alpha:]]|[0-9]|_)*").unwrap();
    pub static ref INT_LIT: Regex = Regex::new(r"([1-9]\d*)|0").unwrap();
    pub static ref FLOAT_LIT: Regex = Regex::new(r"(([1-9]\d*)|0)(\.(\d*[1-9]|0))(e(\+|\-)?(([1-9]\d*)|0))?").unwrap();
    pub static ref STRING_LIT: Regex = Regex::new("\"([a-zA-Z]|[0-9]|_|[[:space:]])*\"").unwrap();

    pub static ref EQEQ: Regex = Regex::new("==").unwrap();
    pub static ref NOTEQ: Regex = Regex::new("<>").unwrap();
    pub static ref LT: Regex = Regex::new("<").unwrap();
    pub static ref GT: Regex = Regex::new(">").unwrap();
    pub static ref LTEQ: Regex = Regex::new("<=").unwrap();
    pub static ref GTEQ: Regex = Regex::new(">=").unwrap();
    pub static ref PLUS: Regex = Regex::new(r"\+").unwrap();
    pub static ref MINUS: Regex = Regex::new(r"\-").unwrap();
    pub static ref MULT: Regex = Regex::new(r"\*").unwrap();
    pub static ref FSLASH: Regex = Regex::new("/").unwrap();
    pub static ref ASSIGN: Regex = Regex::new("=").unwrap();
    pub static ref OR: Regex = Regex::new(r"\|").unwrap();
    pub static ref AND: Regex = Regex::new("&").unwrap();
    pub static ref BANG: Regex = Regex::new("!").unwrap();
    pub static ref QUESTION: Regex = Regex::new(r"\?").unwrap();
    pub static ref OPENPAREN: Regex = Regex::new(r"\(").unwrap();
    pub static ref CLOSEPAREN: Regex = Regex::new(r"\)").unwrap();
    pub static ref OPENCURLY: Regex = Regex::new(r"\{").unwrap();
    pub static ref CLOSECURLY: Regex = Regex::new(r"\}").unwrap();
    pub static ref OPENSQUARE: Regex = Regex::new(r"\[").unwrap();
    pub static ref CLOSESQUARE: Regex = Regex::new(r"\]").unwrap();
    pub static ref SEMICOLON: Regex = Regex::new(";").unwrap();
    pub static ref COMMA: Regex = Regex::new(",").unwrap();
    pub static ref PERIOD: Regex = Regex::new(r"\.").unwrap();
    pub static ref COLON: Regex = Regex::new(":").unwrap();
    pub static ref DBCOLON: Regex = Regex::new("::").unwrap();
    pub static ref QUOTE: Regex = Regex::new("\"").unwrap();
    pub static ref IF: Regex = Regex::new("if").unwrap();
    pub static ref THEN: Regex = Regex::new("then").unwrap();
    pub static ref ELSE: Regex = Regex::new("else").unwrap();
    pub static ref INT_T: Regex = Regex::new("integer").unwrap();
    pub static ref FLOAT_T: Regex = Regex::new("float").unwrap();
    pub static ref STRING_T: Regex = Regex::new("string").unwrap();
    pub static ref VOID: Regex = Regex::new("void").unwrap();
    pub static ref PUBLIC: Regex = Regex::new("public").unwrap();
    pub static ref PRIVATE: Regex = Regex::new("private").unwrap();
    pub static ref FUNC: Regex = Regex::new("func").unwrap();
    pub static ref VAR: Regex = Regex::new("var").unwrap();
    pub static ref CLASS: Regex = Regex::new("class").unwrap();
    pub static ref WHILE: Regex = Regex::new("while").unwrap();
    pub static ref READ: Regex = Regex::new("read").unwrap();
    pub static ref WRITE: Regex = Regex::new("write").unwrap();
    pub static ref RETURN: Regex = Regex::new("return").unwrap();
    pub static ref MAIN: Regex = Regex::new("main").unwrap();
    pub static ref INHERITS: Regex = Regex::new("inherits").unwrap();
    pub static ref BREAK: Regex = Regex::new("break").unwrap();
    pub static ref CONTINUE: Regex = Regex::new("continue").unwrap();
    pub static ref LINE_COMMENT: Regex = Regex::new("//").unwrap();
    pub static ref OPEN_MULTILINE_COMMENT: Regex = Regex::new(r"/\*").unwrap();
    pub static ref CLOSE_MULTILINE_COMMENT: Regex = Regex::new(r"\*/").unwrap();
    pub static ref ERROR: Regex = Regex::new("ERROR").unwrap();
}

#[cfg(test)]
mod test
{
    use super::*;
    use crate::token::TokenType::FloatLit;

    #[test]
    fn id_match()
    {
        assert!(ID.is_match("abc"));
        assert_eq!(ID.find("abc").unwrap().as_str(), "abc");
        assert!(ID.is_match("abc1"));
        assert_eq!(ID.find("abc1").unwrap().as_str(), "abc1");
        assert!(ID.is_match("a1bc"));
        assert_eq!(ID.find("a1bc").unwrap().as_str(), "a1bc");
        assert!(ID.is_match("abc_1abc"));
        assert_eq!(ID.find("abc_1abc").unwrap().as_str(), "abc_1abc");
        assert!(ID.is_match("abc1_abc"));
        assert_eq!(ID.find("abc1_abc").unwrap().as_str(), "abc1_abc");
    }

    #[test]
    fn int_lit_match()
    {
        assert!(INT_LIT.is_match("0"));
        assert_eq!(INT_LIT.find("0").unwrap().as_str(), "0");
        assert!(INT_LIT.is_match("1"));
        assert_eq!(INT_LIT.find("1").unwrap().as_str(), "1");
        assert!(INT_LIT.is_match("10"));
        assert_eq!(INT_LIT.find("10").unwrap().as_str(), "10");
        assert!(INT_LIT.is_match("12"));
        assert_eq!(INT_LIT.find("12").unwrap().as_str(), "12");
        assert!(INT_LIT.is_match("123"));
        assert_eq!(INT_LIT.find("123").unwrap().as_str(), "123");
        assert!(INT_LIT.is_match("12345"));
        assert_eq!(INT_LIT.find("12345").unwrap().as_str(), "12345");
    }

    #[test]
    fn float_lit_match()
    {
        assert!(FLOAT_LIT.is_match("1.23"));
        assert_eq!(FLOAT_LIT.find("1.23").unwrap().as_str(), "1.23");
        assert!(FLOAT_LIT.is_match("12.34"));
        assert_eq!(FLOAT_LIT.find("12.34").unwrap().as_str(), "12.34");
        assert!(FLOAT_LIT.is_match("120.34e10"));
        assert_eq!(FLOAT_LIT.find("120.34e10").unwrap().as_str(), "120.34e10");
        assert!(FLOAT_LIT.is_match("12345.6789e-123"));
        assert_eq!(FLOAT_LIT.find("12345.6789e-123").unwrap().as_str(), "12345.6789e-123");
    }

    #[test]
    fn string_lit_match()
    {
        assert!(STRING_LIT.is_match("\"this is a string literal\""));
        assert_eq!(STRING_LIT.find("\"this is a string literal\"").unwrap().as_str(), "\"this is a string literal\"");
    }
}
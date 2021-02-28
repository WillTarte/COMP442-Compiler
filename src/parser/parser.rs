//https://courses.cs.vt.edu/cs3304/Fall16/meng/lecture_notes/cs3304-7.pdf

use lazy_static::lazy_static;

use crate::parser::utils::KeyPair;
use crate::lexer::lexer::LexerAnalyzer;
use crate::lexer::token::{Token, TokenType};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;
use crate::parser::grammar::GrammarSymbol::{STOP, NonTerminal, Terminal, START};
use crate::parser::grammar::{GrammarSymbol, GrammarRule};

pub fn parse<T>(mut lexer: T) -> bool
    where T: LexerAnalyzer<TokenOutput = Token>
{
    let mut stack: Vec<GrammarSymbol> = Vec::new();
    let table: HashMap<(GrammarSymbol, GrammarSymbol), GrammarRule> = HashMap::new();

    stack.push(STOP);
    stack.push(START);
    todo!()
}
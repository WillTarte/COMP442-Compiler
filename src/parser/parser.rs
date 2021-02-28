/*
Input: a string w, a parsing table M for grammar G
Output: if w is in L(G), a leftmost derivation of w; otherwise, an error indication
Method:
    set ip to point to the first symbol of w$
    repeat
        let X be the top stack symbol and a the symbol pointed to by ip;
        if X is a terminal or $, then
            if X = a then
                pop X from the stack and advance ip
            else error()
        else /* X is a non-terminal */
            if M[X, a] = X->Y1Y2…Yk, then
                pop X from the stack
                push Yk, …, Y2, Y1 on to the stack
                output the production X->Y1Y2…Yk
            else error()
    until X = $
    https://courses.cs.vt.edu/cs3304/Fall16/meng/lecture_notes/cs3304-7.pdf

*/
use lazy_static::lazy_static;

use crate::parser::utils::KeyPair;
use crate::lexer::lexer::LexerAnalyzer;
use crate::lexer::token::{Token, TokenType};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;




#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum GrammarSymbol
{
    Terminal(TokenType),
    NonTerminal,
    STOP
}
//todo EPSILON

#[derive(Eq, PartialEq, Debug)]
pub struct GrammarRule
{
    pub lhs: GrammarSymbol,
    pub rhs: Vec<GrammarSymbol>
}

pub fn parse<T>(mut lexer: T) -> ()//bool
    where T: LexerAnalyzer<TokenOutput = Token>
{
    let mut stack: Vec<GrammarSymbol> = Vec::new();
    let table: HashMap<(GrammarSymbol, GrammarSymbol), GrammarRule> = HashMap::new();

    let mut ip: usize = 0;

    loop {
        if stack.is_empty()
        {
            break;
        }
        let x = stack.last().unwrap();
        match x
        {
            GrammarSymbol::Terminal(_) => {
                if *x == stack[ip]
                {
                    stack.pop();
                    ip += 1;
                }
                else {
                    todo!("errorHandling()");
                }
            }
            GrammarSymbol::NonTerminal => {
                match table.get(&(*x, stack[ip]))
                {
                    None => { todo!("errorHandling()"); }
                    Some(rule) => {
                        stack.pop();
                        for symbol in rule.rhs.iter().rev()
                        {
                            stack.push(*symbol);
                        }
                        todo!("output rule");
                    }
                }
            },
            GrammarSymbol::STOP => {
                break;
            }
        }
    };
}
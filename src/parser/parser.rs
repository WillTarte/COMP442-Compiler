//https://courses.cs.vt.edu/cs3304/Fall16/meng/lecture_notes/cs3304-7.pdf

use lazy_static::lazy_static;

use crate::parser::utils::KeyPair;
use crate::lexer::lexer::LexerAnalyzer;
use crate::lexer::token::{Token, TokenType};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;
use crate::parser::grammar::GrammarSymbol::{STOP, NonTerminal, Terminal, START, EPSILON};
use crate::parser::grammar::{GrammarSymbol, GrammarRule};
use crate::parser::data::PARSING_TABLE;

pub fn parse<T>(mut lexer: T) -> bool
    where T: LexerAnalyzer<TokenOutput = Token>
{
    let mut stack: Vec<GrammarSymbol> = Vec::new();
    stack.push(STOP);
    stack.push(START);

    let mut next_token = lexer.next_token();

    while *stack.last().unwrap() != STOP //todo
    {
        let top_symbol = stack.last().unwrap();
        match top_symbol {
            Terminal(token_t) => {
                if next_token.is_some() && *token_t == next_token.as_ref().unwrap().token_type()
                {
                    println!("Found Token: {}", next_token.as_ref().unwrap().lexeme());
                    stack.pop();
                    next_token = lexer.next_token();
                }
                else
                {
                    // scan for correct token from input
                    while next_token.is_some() && next_token.as_ref().unwrap().token_type() != *token_t
                    {
                        next_token = lexer.next_token();
                    }
                }
            },
            NonTerminal(named_symbol) => {
                if next_token.is_none()
                {
                    continue;
                }
                else {
                    match PARSING_TABLE.get(&(NonTerminal(*named_symbol), Terminal(next_token.as_ref().unwrap().token_type())))
                    {
                        None => {
                            let grammar_symbol = NonTerminal(*named_symbol);
                            let first = grammar_symbol.first_set();
                            let follow = grammar_symbol.follow_set();
                            // pop
                            if next_token.is_none() || follow.contains(&Terminal(next_token.as_ref().unwrap().token_type()))
                            {
                                stack.pop();
                            }
                            // scan
                            else {
                                while !first.contains(&Terminal(next_token.as_ref().unwrap().token_type())) || (first.contains(&EPSILON) && !follow.contains(&Terminal(next_token.as_ref().unwrap().token_type())))
                                {
                                    next_token = lexer.next_token();
                                }
                            }
                        }
                        Some(rule) => {
                            println!("Applied derivation: {}", rule.to_string());
                            stack.pop();
                            for rhs_symbol in rule.rhs.iter().rev()
                            {
                                stack.push(*rhs_symbol);
                            }
                        }
                    }
                }
            },
            START => {
                if next_token.is_none()
                {
                    continue;
                }
                else {
                    match PARSING_TABLE.get(&(START, Terminal(next_token.as_ref().unwrap().token_type())))
                    {
                        None => {
                            let grammar_symbol = START;
                            let first = grammar_symbol.first_set();
                            let follow = grammar_symbol.follow_set();
                            // pop
                            if next_token.is_none() || follow.contains(&Terminal(next_token.as_ref().unwrap().token_type()))
                            {
                                stack.pop();
                            }
                            // scan
                            else {
                                while !first.contains(&Terminal(next_token.as_ref().unwrap().token_type())) || (first.contains(&EPSILON) && !follow.contains(&Terminal(next_token.as_ref().unwrap().token_type())))
                                {
                                    next_token = lexer.next_token();
                                }
                            }
                        }
                        Some(rule) => {
                            println!("Applied derivation: {}", rule.to_string());
                            stack.pop();
                            for rhs_symbol in rule.rhs.iter().rev()
                            {
                                stack.push(*rhs_symbol);
                            }
                        }
                    }
                }
            },
            EPSILON => { stack.pop(); continue; }
            STOP => { panic!() }
        };
    }
    // if a not None or error == true
    //   return false
    // else
    //   return true
    todo!()
}
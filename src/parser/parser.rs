//https://courses.cs.vt.edu/cs3304/Fall16/meng/lecture_notes/cs3304-7.pdf

use lazy_static::lazy_static;

use crate::lexer::lexer::LexerAnalyzer;
use crate::lexer::token::{Token, TokenType};
use crate::parser::data::PARSING_TABLE;
use crate::parser::grammar::GrammarSymbol::{NonTerminal, Terminal, EPSILON, STOP};
use crate::parser::grammar::{GrammarRule, GrammarSymbol};
use crate::parser::utils::KeyPair;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::lexer::token::TokenType::{Error, LineComment, MultilineComment, Class};
use crate::parser::grammar::NamedSymbol::Start;
use std::ops::Deref;

pub fn parse<T>(mut lexer: T) -> bool
where
    T: LexerAnalyzer<TokenOutput = Token> + IntoIterator<Item = <T as LexerAnalyzer>::TokenOutput>,
{
    lazy_static::initialize(&PARSING_TABLE);

    let mut stack: Vec<GrammarSymbol> = Vec::new();
    stack.push(STOP);
    stack.push(NonTerminal(Start));

    let mut token_stream = lexer.into_iter().filter(|i: &Token| i.token_type() != LineComment && i.token_type() != MultilineComment);

    let mut next_token: Option<Token> = token_stream.next();

    let mut error = false;

    while *stack.last().unwrap() != STOP {

        let top_symbol = stack.last().unwrap();

        match top_symbol {
            Terminal(token_t) => {
                if next_token.is_some() && *token_t == next_token.as_ref().unwrap().token_type() {
                    println!("Found Token: {}", next_token.as_ref().unwrap().lexeme());
                    stack.pop();
                    next_token = token_stream.next();
                    println!("STACK: {:?}", &stack);
                } else {
                    // scan for correct token from input
                    while next_token.is_some()
                        && next_token.as_ref().unwrap().token_type() != *token_t
                    {
                        println!("SCANNING FOR {:?} : {:?}", token_t, &next_token);
                        error = true;
                        next_token = token_stream.next();
                    }
                }
            }
            NonTerminal(named_symbol) => {
                if next_token.is_none() {
                    continue;
                } else {
                    println!("Lookup for ({:?}, {:?})", named_symbol, next_token);
                    match PARSING_TABLE.get(&(
                        NonTerminal(*named_symbol),
                        Terminal(next_token.as_ref().unwrap().token_type()),
                    )) {
                        None => {
                            error = true;
                            let grammar_symbol = NonTerminal(*named_symbol);
                            let first = grammar_symbol.first_set();
                            let follow = grammar_symbol.follow_set();
                            // pop
                            if next_token.is_none()
                                || follow
                                    .contains(&Terminal(next_token.as_ref().unwrap().token_type()))
                            {
                                println!("POP: {:?}", stack.pop());
                            }
                            // scan
                            else {
                                if first.contains(&EPSILON) {
                                    while !follow.contains(&Terminal(
                                        next_token.as_ref().unwrap().token_type(),
                                    )) {
                                        next_token = token_stream.next();
                                        println!("SCAN FOLLOW OF {:?} : {:?}", &grammar_symbol, &next_token);
                                    }
                                } else {
                                    while !first.contains(&Terminal(
                                        next_token.as_ref().unwrap().token_type(),
                                    )) {
                                        next_token = token_stream.next();
                                        println!("SCAN FIRST OF {:?} : {:?}", &grammar_symbol, &next_token);
                                    }
                                }
                            }
                        }
                        Some(rule) => {
                            println!("Applied derivation: {}", rule.to_string());
                            stack.pop();
                            for rhs_symbol in rule.rhs.iter().rev() {
                                stack.push(*rhs_symbol);
                            }
                            println!("STACK: {:?}", &stack);
                        }
                    }
                }
            }
            EPSILON => {
                stack.pop();
                println!("STACK: {:?}", &stack);
                continue;
            }
            STOP => {
                panic!()
            }
        };
    }

    return if next_token.is_some() || stack.len() > 1 || error {
        false
    } else {
        true
    };
}
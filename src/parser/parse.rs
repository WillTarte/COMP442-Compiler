//! Parsing algorithm

//https://courses.cs.vt.edu/cs3304/Fall16/meng/lecture_notes/cs3304-7.pdf
use crate::lexer::lexer::LexerAnalyzer;
use crate::lexer::token::Token;
use crate::lexer::token::TokenType::{LineComment, MultilineComment};
use crate::parser::ast::{SemanticAction, SemanticStack};
use crate::parser::data::PARSING_TABLE;
use crate::parser::grammar::DerivationTable;
use crate::parser::grammar::GrammarSymbol::*;
use crate::parser::grammar::NamedSymbol::Start;
use crate::parser::grammar::{DerivationRecord, GrammarSymbol};
use log::{trace, warn};

/// Parses a token stream and produces either a DerivationTable and an AST or an error
pub fn parse<T>(lexer: T) -> Result<(DerivationTable, SemanticStack), ()>
where
    T: LexerAnalyzer<TokenOutput = Token> + IntoIterator<Item = <T as LexerAnalyzer>::TokenOutput>,
{
    trace!("Initializing parsing table.");
    lazy_static::initialize(&PARSING_TABLE);

    let mut derivation_table = DerivationTable::new();

    let mut parsing_stack: Vec<GrammarSymbol> = Vec::new();
    parsing_stack.push(STOP);
    parsing_stack.push(NonTerminal(Start));

    let mut semantic_stack: SemanticStack = SemanticStack::new();

    let mut token_stream = lexer.into_iter();

    let mut next_token: Option<Token> = token_stream.next();

    let mut error = false;

    derivation_table.add_record(DerivationRecord::new(&parsing_stack, &next_token, None));

    while *parsing_stack.last().unwrap() != STOP {
        let top_symbol = parsing_stack.last().unwrap().clone();
        if next_token.is_some()
            && (next_token.as_ref().unwrap().token_type() == LineComment
                || next_token.as_ref().unwrap().token_type() == MultilineComment)
        {
            next_token = token_stream.next();
        }
        trace!("Top Symbol: {:?}", top_symbol);
        trace!("Lookahead: {:?}", next_token);
        match top_symbol {
            Terminal(token_t) => {
                if next_token.is_some() && token_t == next_token.as_ref().unwrap().token_type() {
                    parsing_stack.pop();
                    next_token = token_stream.next();
                    derivation_table.add_record(DerivationRecord::new(
                        &parsing_stack,
                        &next_token,
                        None,
                    ))
                } else {
                    warn!(
                        "~ Mistmatch! Expected token of type {:?}, but got {:?} instead.",
                        token_t, next_token
                    );
                    error = true;
                    while next_token.is_some()
                        && next_token.as_ref().unwrap().token_type() != token_t
                    {
                        next_token = token_stream.next();
                    }
                }
            }
            NonTerminal(named_symbol) => {
                if next_token.is_none() {
                    break;
                } else {
                    match PARSING_TABLE.get(&(
                        NonTerminal(named_symbol),
                        Terminal(next_token.as_ref().unwrap().token_type()),
                    )) {
                        None => {
                            warn!("~ No rule found for ({:?}, {:?})", named_symbol, next_token);
                            error = true;
                            let first = named_symbol.first_set();
                            let follow = named_symbol.follow_set();
                            // pop
                            if next_token.is_none()
                                || follow
                                    .contains(&Terminal(next_token.as_ref().unwrap().token_type()))
                            {
                                warn!("~ Popped: {:?}", parsing_stack.pop());
                            }
                            // scan
                            else {
                                if first.contains(&EPSILON) {
                                    warn!("~ Scanning Follow set");
                                    while !follow.contains(&Terminal(
                                        next_token.as_ref().unwrap().token_type(),
                                    )) {
                                        next_token = token_stream.next();
                                        if next_token.is_none() {
                                            break;
                                        }
                                    }
                                } else {
                                    warn!("~ Scanning First set");
                                    while !first.contains(&Terminal(
                                        next_token.as_ref().unwrap().token_type(),
                                    )) {
                                        next_token = token_stream.next();
                                        if next_token.is_none() {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        Some(rule) => {
                            trace!("Found rule! Applying derivation: {}", rule.to_string());
                            parsing_stack.pop();
                            for rhs_symbol in rule.rhs.iter().rev() {
                                parsing_stack.push(*rhs_symbol);
                            }
                            derivation_table.add_record(DerivationRecord::new(
                                &parsing_stack,
                                &next_token,
                                Some(rule),
                            ));
                        }
                    }
                }
            }
            EPSILON => {
                trace!("Applying Epsilon");
                parsing_stack.pop();

                derivation_table.add_record(DerivationRecord::new(
                    &parsing_stack,
                    &next_token,
                    None,
                ));
                continue;
            }
            STOP => {
                panic!()
            }
            SemanticActionType(sa) => {
                parsing_stack.pop();
                match sa {
                    SemanticAction::MakeFamilyRootNode(ty) => {
                        semantic_stack.make_family_root(ty);
                    }
                    SemanticAction::MakeTerminalNode => {
                        if next_token.is_none() {
                            panic!("Tried to make terminal node with None next_token");
                        } else {
                            semantic_stack.make_terminal_node(next_token.as_ref().unwrap());
                        }
                    }
                    SemanticAction::MakeRelativeOperation => {
                        semantic_stack.make_relative_operation();
                    }
                    SemanticAction::MakeEmptyNode => {
                        semantic_stack.make_empty_node();
                    }
                    SemanticAction::AddChild => {
                        semantic_stack.add_child();
                    }
                }
            }
        };
    }

    return if next_token.is_some() || parsing_stack.len() > 1 || error {
        Err(())
    } else {
        Ok((derivation_table, semantic_stack))
    };
}

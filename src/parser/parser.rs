//https://courses.cs.vt.edu/cs3304/Fall16/meng/lecture_notes/cs3304-7.pdf

use crate::lexer::lexer::LexerAnalyzer;
use crate::lexer::token::TokenType::{LineComment, MultilineComment};
use crate::lexer::token::{Token};
use crate::parser::ast::NodeVal::{Internal, Leaf, NonParsed};
use crate::parser::ast::{Node, NodeVal, SemanticStack};
use crate::parser::data::PARSING_TABLE;
use crate::parser::grammar::DerivationTable;
use crate::parser::grammar::GrammarSymbol::{NonTerminal, Terminal, EPSILON, STOP, MakeFamilyAttribute};
use crate::parser::grammar::NamedSymbol::Start;
use crate::parser::grammar::{DerivationRecord, GrammarSymbol};

//http://pages.cs.wisc.edu/~fischer/cs536.f12/lectures/Lecture23.pdf
pub fn parse<T>(lexer: T) -> Result<(DerivationTable, SemanticStack), ()>
where
    T: LexerAnalyzer<TokenOutput = Token> + IntoIterator<Item = <T as LexerAnalyzer>::TokenOutput>,
{
    lazy_static::initialize(&PARSING_TABLE);

    let mut derivation_table = DerivationTable::new();

    let mut parsing_stack: Vec<GrammarSymbol> = Vec::new();
    parsing_stack.push(STOP);
    parsing_stack.push(NonTerminal(Start));

    let mut semantic_stack: SemanticStack = SemanticStack::new();
    semantic_stack.add_node(Node::new_with_val(Internal(Start)));

    let mut token_stream = lexer
        .into_iter()
        .filter(|i: &Token| i.token_type() != LineComment && i.token_type() != MultilineComment);

    let mut next_token: Option<Token> = token_stream.next();

    let mut error = false;

    derivation_table.add_record(DerivationRecord::new(&parsing_stack, &next_token, None));

    while *parsing_stack.last().unwrap() != STOP {
        let top_symbol = parsing_stack.last().unwrap().clone();

        match top_symbol {
            Terminal(token_t) => {
                if next_token.is_some() && token_t == next_token.as_ref().unwrap().token_type() {
                    //println!("Found Token: {}", next_token.as_ref().unwrap().lexeme());
                    parsing_stack.pop();
                    match semantic_stack.0.last() {
                        None => {}
                        Some(node) => match &node.val {
                            None => {}
                            Some(node_val) => match node_val {
                                NonParsed(t_type) => {
                                    if *t_type == next_token.as_ref().unwrap().token_type() {
                                        semantic_stack.replace_last(Node::new_with_val(Leaf(
                                            next_token.as_ref().unwrap().clone(),
                                        )));
                                    }
                                }
                                Leaf(_) => {}
                                Internal(_) => {}
                            },
                        },
                    }
                    next_token = token_stream.next();
                    derivation_table.add_record(DerivationRecord::new(
                        &parsing_stack,
                        &next_token,
                        None,
                    ))
                } else {
                    // scan for correct token from input
                    while next_token.is_some()
                        && next_token.as_ref().unwrap().token_type() != token_t
                    {
                        //println!("SCANNING FOR {:?} : {:?}", token_t, &next_token);
                        error = true;
                        next_token = token_stream.next();
                    }
                }
            }
            NonTerminal(named_symbol) => {
                if next_token.is_none() {
                    continue;
                } else {
                    //println!("Lookup for ({:?}, {:?})", named_symbol, next_token);
                    match PARSING_TABLE.get(&(
                        NonTerminal(named_symbol),
                        Terminal(next_token.as_ref().unwrap().token_type()),
                    )) {
                        None => {
                            error = true;
                            let grammar_symbol = NonTerminal(named_symbol);
                            let first = grammar_symbol.first_set();
                            let follow = grammar_symbol.follow_set();
                            // pop
                            if next_token.is_none()
                                || follow
                                    .contains(&Terminal(next_token.as_ref().unwrap().token_type()))
                            {
                                println!("POP: {:?}", parsing_stack.pop());
                            }
                            // scan
                            else {
                                if first.contains(&EPSILON) {
                                    while !follow.contains(&Terminal(
                                        next_token.as_ref().unwrap().token_type(),
                                    )) {
                                        next_token = token_stream.next();
                                        println!(
                                            "SCAN FOLLOW OF {:?} : {:?}",
                                            &grammar_symbol, &next_token
                                        );
                                    }
                                } else {
                                    while !first.contains(&Terminal(
                                        next_token.as_ref().unwrap().token_type(),
                                    )) {
                                        next_token = token_stream.next();
                                        println!(
                                            "SCAN FIRST OF {:?} : {:?}",
                                            &grammar_symbol, &next_token
                                        );
                                    }
                                }
                            }
                        }
                        Some(rule) => {
                            println!("Applying derivation: {}", rule.to_string());
                            parsing_stack.pop();
                            let mut temp_parsing: Vec<GrammarSymbol> = Vec::new();
                            let mut family_size = 0usize;
                            for rhs_symbol in rule.rhs.iter().rev() {
                                //parsing_stack.push(*rhs_symbol);
                                temp_parsing.push(*rhs_symbol);
                                match rhs_symbol
                                {
                                    Terminal(ty) => { semantic_stack.add_node(Node::new_with_val(NonParsed(*ty))); }
                                    NonTerminal(sy) => { semantic_stack.add_node(Node::new_with_val(NodeVal::Internal(*sy))); }
                                    EPSILON => { break; }
                                    STOP => { panic!() }
                                    _ => { todo!("Semantic Attributes") }
                                }
                                family_size += 1;
                            }
                            parsing_stack.push(MakeFamilyAttribute(family_size));
                            parsing_stack.append(&mut temp_parsing);
                            //println!("-> Parsing Stack {:?}", &parsing_stack);
                            //println!("-> Semantic Stack {:?}", &semantic_stack);
                            derivation_table.add_record(DerivationRecord::new(
                                &parsing_stack,
                                &next_token,
                                Some(rule),
                            ));
                        }
                    }
                }
            },
            MakeFamilyAttribute(size) => {
                parsing_stack.pop();
                semantic_stack.make_family(size);
            },
            EPSILON => {
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
        };
    }

    return if next_token.is_some() || parsing_stack.len() > 1 || error {
        Err(())
    } else {
        Ok((derivation_table, semantic_stack))
    };
}

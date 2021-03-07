use crate::lexer::token::Token;
use crate::parser::grammar::GrammarSymbol;
use std::rc::Rc;

//https://cs.nyu.edu/courses/spring01/G22.2130-001/parsing-slides/ppframe.htm
#[derive(Clone, Debug)]
pub struct Node {
    parent: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
    leftmost: Option<Rc<Node>>,
    val: NodeVal,
}

#[derive(Clone, Debug)]
pub enum NodeVal {
    Leaf(Token),
    Internal(GrammarSymbol),
}

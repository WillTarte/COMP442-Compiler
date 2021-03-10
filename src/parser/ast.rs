use crate::lexer::token::Token;
use crate::parser::grammar::GrammarSymbol;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::borrow::Borrow;
use crate::lexer::token::TokenType::Read;

type Link = Option<Rc<RefCell<Node>>>;

//https://cs.nyu.edu/courses/spring01/G22.2130-001/parsing-slides/ppframe.htm
#[derive(Clone, Debug)]
pub struct Node {
    parent: Link,
    right_sibling: Link,
    leftmost_sibling: Link,
    children: Vec<Node>,
    val: Option<NodeVal>,
}

impl Node
{
    pub fn new(val: NodeVal) -> Self
    {
        Self
        {
            parent: None,
            right_sibling: None,
            leftmost_sibling: None,
            children: Vec::new(),
            val: Some(val)
        }
    }

    pub fn make_empty() -> Self
    {
        Self
        {
            parent: None,
            right_sibling: None,
            leftmost_sibling: None,
            children: Vec::new(),
            val: None
        }
    }

    pub fn add_sibling_tree(&mut self, mut new_sibling: Rc<RefCell<Node>>)
    {
        let mut xsib: Link = None;
        while self.right_sibling.is_some()
        {
            xsib = self.right_sibling.clone();
        }

        let mut ysib: Link = Some(new_sibling.clone());

        match (xsib, ysib)
        {
            (None, None) => { self.right_sibling = Some(new_sibling.clone()) },
            (None, Some(y)) => { self.right_sibling = Some(y.clone()) },
            (Some(x), None) => { x.borrow_mut().right_sibling = Some(new_sibling.clone()) },
            (Some(x), Some(y)) => { x.borrow_mut().right_sibling = Some(y.clone()) }
        };

        new_sibling.borrow_mut().leftmost_sibling = self.leftmost_sibling.clone();
        new_sibling.borrow_mut().parent = self.parent.clone();

        let mut new_sib = new_sibling.clone();
        while new_sib.borrow_mut().right_sibling.is_some()
        {
            new_sib.borrow_mut().leftmost_sibling = self.leftmost_sibling.clone();
            new_sib.borrow_mut().parent = self.parent.clone();
            new_sib = Rc::clone(new_sib.borrow().into_inner())
        }
    }

    pub fn add_sibling_node(&mut self, mut new_sibling: Node)
    {
        let mut xsib: Link = None;
        while self.right_sibling.is_some()
        {
            xsib = self.right_sibling.clone();
        }

        let ysib: Link = new_sibling.leftmost_sibling.clone();

        match (xsib, ysib) {
            (None, None) => { self.right_sibling = Some(Rc::new(RefCell::new(new_sibling))) },
            (None, Some(y)) => { self.right_sibling = Some(y.clone()) },
            (Some(x), None) => { x.borrow_mut().right_sibling = Some(Rc::new(RefCell::new(new_sibling))) },
            (Some(x), Some(y)) => { x.borrow_mut().right_sibling = Some(y.clone()) }
        }

        new_sibling.leftmost_sibling = self.leftmost_sibling.clone();
        new_sibling.parent = self.parent.clone();

    }
}

#[derive(Clone, Debug)]
pub enum NodeVal {
    Leaf(Token),
    Internal(GrammarSymbol),
}

pub struct AST
{
    root: Link
}

impl AST
{
    pub fn new() -> Self
    {
        AST
        {
            root: None
        }
    }

    pub fn from_node(root: Node) -> Self
    {
        AST
        {
            root: Some(Rc::new(RefCell::new(root)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::ast::NodeVal::Internal;
    use crate::parser::grammar::GrammarSymbol::NonTerminal;
    use crate::parser::grammar::NamedSymbol::{Start, Expr};

    #[test]
    fn add_sibling()
    {
        let ast: AST = AST::from_node(Node::new(Internal(NonTerminal(Start))));

        let other_node = None::new(Internal(NonTerminal(Expr)));

        //ast.
    }
}
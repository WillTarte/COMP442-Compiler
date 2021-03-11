use crate::lexer::token::{Token, TokenType};
use crate::parser::grammar::{NamedSymbol};
//https://cs.nyu.edu/courses/spring01/G22.2130-001/parsing-slides/ppframe.htm

#[derive(Clone, Debug)]
pub struct Node {
    pub(crate) val: Option<NodeVal>,
    pub(crate) children: Vec<Node>,
}

impl Node {
    pub fn new_empty() -> Self {
        Self {
            children: Vec::new(),
            val: None,
        }
    }

    pub fn new_with_val(val: NodeVal) -> Self {
        Self {
            children: Vec::new(),
            val: Some(val),
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn add_children(&mut self, children: &mut Vec<Node>)
    {
        self.children.append(children);
    }
}

impl ToString for Node
{
    fn to_string(&self) -> String {
        match &self.val
        {
            None => { String::from("None") }
            Some(node_val) => {
                match node_val
                {
                    NodeVal::NonParsed(t_type) => { format!("{:?}", t_type) }
                    NodeVal::Leaf(t) => { format!("\"{}\"", t.lexeme()) }
                    NodeVal::Internal(s) => { format!("{:?}", s) }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeVal {
    NonParsed(TokenType),
    Leaf(Token),
    Internal(NamedSymbol),
}

#[derive(Debug)]
pub struct SemanticStack(pub(crate) Vec<Node>);

impl SemanticStack {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_node(&mut self, node: Node) {
        self.0.push(node);
    }

    pub fn replace_last(&mut self, node: Node) {
        self.0.pop();
        self.0.push(node);
    }

    pub fn make_family(&mut self, n: usize) {
        if n >= self.0.len() {
            panic!(
                "Tried to make family with {} + 1 nodes, but only had {} nodes",
                n,
                self.0.len()
            );
        }
        else if n == 0
        {
            return;
        }

        let mut temp = self.0.drain(self.0.len() - n .. self.0.len()).rev().collect();
        self.0.last_mut().unwrap().add_children(&mut temp);
    }
}

#[cfg(test)]
mod test {}

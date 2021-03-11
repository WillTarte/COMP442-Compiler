use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use crate::parser::grammar::{DerivationTable, GrammarSymbol};
use std::io;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use crate::parser::ast::{SemanticStack, Node, NodeVal};

//https://stackoverflow.com/questions/45786717/how-to-implement-hashmap-with-two-keys
pub trait KeyPair<A, B> {
    fn first(&self) -> &A;
    fn second(&self) -> &B;
}

impl<'a, A, B> Borrow<dyn KeyPair<A, B> + 'a> for (A, B)
where
    A: Eq + Hash + 'a,
    B: Eq + Hash + 'a,
{
    fn borrow(&self) -> &(dyn KeyPair<A, B> + 'a) {
        self
    }
}

impl<A: Hash, B: Hash> Hash for (dyn KeyPair<A, B> + '_) {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first().hash(state);
        self.second().hash(state);
    }
}

impl<A: Eq, B: Eq> PartialEq for (dyn KeyPair<A, B> + '_) {
    fn eq(&self, other: &Self) -> bool {
        self.first() == other.first() && self.second() == other.second()
    }
}

impl<A: Eq, B: Eq> Eq for (dyn KeyPair<A, B> + '_) {}

impl<A, B> KeyPair<A, B> for (A, B) {
    fn first(&self) -> &A {
        &self.0
    }
    fn second(&self) -> &B {
        &self.1
    }
}
impl<A, B> KeyPair<A, B> for (&A, &B) {
    fn first(&self) -> &A {
        self.0
    }
    fn second(&self) -> &B {
        self.1
    }
}

pub fn serialize_parsing_table_to_file(table: DerivationTable, file_name: &str) -> io::Result<()>
{
    let table_file = OpenOptions::new().write(true).create(true).truncate(true).open(format!("{}.derivation.md", file_name))?;
    let mut buf_writer = BufWriter::new(table_file);

    buf_writer.write("| Stack | Lookahead | Rule |\n| --- | --- | --- |\n".as_bytes());

    for record in table.0
    {
        buf_writer.write("|".as_bytes());
        buf_writer.write(format!("{:?}", record.stack_state.iter().rev().take(5).rev().collect::<Vec<&GrammarSymbol>>()).as_bytes());
        buf_writer.write("|".as_bytes());
        buf_writer.write(format!("{:?}", record.lookahead_token).as_bytes());
        buf_writer.write("|".as_bytes());
        buf_writer.write(format!("{:?}", record.derived_rule).as_bytes());
        buf_writer.write("|\n".as_bytes());
    }

    buf_writer.flush()?;
    Ok(())
}

pub fn serialize_tree_to_file(mut tree: SemanticStack, file_name: &str) -> io::Result<()>
{
    let tree_file = OpenOptions::new().write(true).create(true).truncate(true).open(format!("{}.ast.gv", file_name))?;
    let mut buf_writer = BufWriter::new(tree_file);

    buf_writer.write("digraph AST {\n".as_bytes());

    let mut root_stack: Vec<Node> = Vec::new();

    root_stack.append(&mut tree.0);

    while !root_stack.is_empty() {
        let mut current_root = root_stack.remove(0);
        let root_name: String = current_root.to_string();
        buf_writer.write(format!("{};\n", root_name).as_bytes());

        for child in current_root.children.drain(..)
        {
            buf_writer.write(format!("{} -> {};\n", root_name, child.to_string()).as_bytes());
            root_stack.push(child)
        }
    }
    buf_writer.write("}".as_bytes());
    buf_writer.flush()?;
    Ok(())
}

//! Utilities for the parsing

use crate::parser::ast::{Node, SemanticStack};
use crate::parser::grammar::{DerivationTable, GrammarSymbol};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::{BufWriter, Write};

/// https://stackoverflow.com/questions/45786717/how-to-implement-hashmap-with-two-keys
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

/// Serializes a [DerivationTable] to a file
pub fn serialize_derivation_table_to_file(
    table: DerivationTable,
    file_name: &str,
) -> io::Result<()> {
    let table_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}.derivation.md", file_name))?;
    let mut buf_writer = BufWriter::new(table_file);

    buf_writer.write("| Stack | Lookahead | Rule |\n| --- | --- | --- |\n".as_bytes())?;

    for record in table.0 {
        buf_writer.write("|".as_bytes())?;
        buf_writer.write(
            format!(
                "{:?}",
                record
                    .stack_state
                    .iter()
                    .rev()
                    .take(5)
                    .rev()
                    .collect::<Vec<&GrammarSymbol>>()
            )
            .as_bytes(),
        )?;
        buf_writer.write("|".as_bytes())?;
        buf_writer.write(format!("{:?}", record.lookahead_token).as_bytes())?;
        buf_writer.write("|".as_bytes())?;
        buf_writer.write(format!("{:?}", record.derived_rule).as_bytes())?;
        buf_writer.write("|\n".as_bytes())?;
    }

    buf_writer.flush()?;
    Ok(())
}

struct LabeledNode {
    node: Node,
    label: String,
}

impl LabeledNode {
    pub fn new(label: String, node: Node) -> Self {
        Self { node, label }
    }
}

/// Serializes a [SemanticStack] into GraphViz dot file as a graph
pub fn serialize_tree_to_file(mut tree: SemanticStack, file_name: &str) -> io::Result<()> {
    let tree_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}.ast.gv", file_name))?;
    let mut buf_writer = BufWriter::new(tree_file);
    buf_writer.write("digraph AST {\n".as_bytes())?;

    let mut node_label_count: HashMap<String, usize> = HashMap::new();

    let mut root_stack: Vec<LabeledNode> = Vec::new();
    let root = tree.0.remove(0);
    let root_suffix = node_label_count.entry(root.to_string()).or_insert(0);
    let root_label = format!("{}_{}", root.to_string(), *root_suffix);
    *root_suffix += 1;
    let root_labeled = LabeledNode::new(root_label, root);
    root_stack.push(root_labeled);

    while !root_stack.is_empty() {
        let mut current_root = root_stack.remove(0);
        buf_writer.write(
            format!(
                "\"{}\" [label=\"{}\"];",
                current_root.label,
                current_root.node.to_string()
            )
            .as_bytes(),
        )?;

        for unlabelled_child in current_root.node.children_mut().drain(..) {
            let child_suffix = node_label_count
                .entry(unlabelled_child.to_string())
                .or_insert(0);
            let labelled_child = LabeledNode::new(
                format!("{}_{}", unlabelled_child.to_string(), child_suffix),
                unlabelled_child,
            );
            *child_suffix += 1;

            buf_writer.write(
                format!(
                    "\"{}\" -> \"{}\";\n",
                    current_root.label, labelled_child.label
                )
                .as_bytes(),
            )?;
            root_stack.push(labelled_child);
        }
    }

    buf_writer.write("}".as_bytes())?;
    buf_writer.flush()?;
    Ok(())
}

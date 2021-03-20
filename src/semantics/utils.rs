use crate::parser::ast::{Node, NodeVal};
use crate::semantics::symbol_table;
use crate::lexer::token::{TokenType};
use crate::semantics::symbol_table::Type::CustomArray;

pub(crate) fn map_to_type(node: &Node) -> symbol_table::Type
{
    let base_t = match &node.val
    {
        None => { panic!() }
        Some(node_val) => {
            match node_val
            {
                NodeVal::Leaf(t) => {
                    match t.token_type()
                    {
                        TokenType::IntegerType => { symbol_table::Type::Integer },
                        TokenType::FloatType => { symbol_table::Type::Float },
                        TokenType::StringType => { symbol_table::Type::String },
                        TokenType::Id => { symbol_table::Type::Custom(t.lexeme().to_string()) },
                        _ => { todo!() }
                    }
                }
                NodeVal::Internal(_) => { panic!() }
            }
        }
    };
    let array_dim: Vec<usize> = node.children.iter().map(map_to_usize).filter_map(|u| u).collect();
    if array_dim.len() > 0
    {
        base_t
    }
    else {
        base_t.to_array_type(array_dim)
    }
}

pub(crate) fn map_to_usize(node: &Node) -> Option<usize> // e.g. ArrayDim children are integer tokens
{
    match &node.val
    {
        None => { None }
        Some(v) => {
            match v
            {
                NodeVal::Leaf(t) => {
                    Some(t.lexeme().parse::<usize>().unwrap()) // todo this is yikes
                }
                NodeVal::Internal(_) => { panic!() }
            }
        }
    }
}
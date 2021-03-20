use std::collections::HashMap;
use crate::parser::ast::{Node, NodeVal, InternalNodeType};
use crate::semantics::utils::map_to_type;
use crate::semantics::symbol_table::Type::{IntegerArray, FloatArray, StringArray, CustomArray};

pub enum Scope
{
    Class(ClassEntry, SymbolTable),
    Function(FunctionEntry, SymbolTable),
    Variable(VariableEntry)
}

pub struct SymbolTable(Vec<Scope>);

impl SymbolTable
{
    pub fn new() -> Self
    {
        Self(Vec::new())
    }
}

pub enum Type
{
    Integer,
    IntegerArray(Vec<usize>),
    Float,
    FloatArray(Vec<usize>),
    String,
    StringArray(Vec<usize>),
    Custom(String),
    CustomArray(String, Vec<usize>),
}

impl Type
{
    pub fn to_array_type(&self, array_dim: Vec<usize>) -> Self
    {
        match self {
            Type::Integer => { IntegerArray(array_dim) },
            Type::Float => { FloatArray(array_dim) },
            Type::String => { StringArray(array_dim) },
            Type::Custom(id) => { CustomArray(id.clone(), array_dim) },
            _ => { panic!() }
        }
    }
}

pub struct FunctionEntry {
    identifier: String,
    type_signature: (Vec<Type>, Option<Type>), // param types -> return type
    table: SymbolTable,
}

impl FunctionEntry
{
    pub fn new_from_node(node: &Node) -> Self
    {
        todo!()
    }
}

pub struct ClassEntry {
    identifier: String,
    inherits: Vec<Type>,
    table: SymbolTable,
}

impl ClassEntry {
    pub fn new_from_node(node: &Node) -> Self
    {
        match &node.val {
            None => {
                panic!()
            }
            Some(v) => {
                match v
                {
                    NodeVal::Leaf(_) => { panic!() }
                    NodeVal::Internal(ty) => {
                        match ty {
                            InternalNodeType::ClassDeclaration => {
                                assert_eq!(node.children.len(), 3); // class name, inherit list, member list
                                let ident = match node.children[0].val.as_ref().unwrap()
                                {
                                    NodeVal::Leaf(t) => { t.token_fragment.lexeme.clone() }
                                    NodeVal::Internal(_) => { panic!() }
                                };
                                let inherits: Vec<Type> = node.children[1].children.iter().map(map_to_type).collect();
                                todo!("parse members")
                            },
                            _ => { panic!(); }
                        }
                    }
                }
            }
        }
    }
}

pub struct VariableEntry {
    identifier: String,
    variable_type: Type,
}

impl VariableEntry {
    pub fn new_from_node(node: &Node) -> Self
    {
        todo!()
    }
}

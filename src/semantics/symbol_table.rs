use crate::parser::ast::InternalNodeType::ClassDeclaration;
use crate::parser::ast::{InternalNodeType, Node, NodeVal};
use crate::semantics::symbol_table::Type::{CustomArray, FloatArray, IntegerArray, StringArray};
use std::collections::HashMap;

pub enum Scope {
    Class(ClassEntry),
    Function(FunctionEntry),
    Variable(VariableEntry),
    FunctionParameter(ParameterEntry),
}

pub struct SymbolTable(Vec<Scope>);

impl SymbolTable {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn new_from_scopes(scopes: Vec<Scope>) -> Self {
        Self(scopes)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Type {
    Integer,
    IntegerArray(Vec<usize>),
    Float,
    FloatArray(Vec<usize>),
    String,
    StringArray(Vec<usize>),
    Custom(String),
    CustomArray(String, Vec<usize>),
    Void,
}

impl Type {
    pub fn as_array_type(self, array_dim: Vec<usize>) -> Self {
        match self {
            Type::Integer => IntegerArray(array_dim),
            Type::Float => FloatArray(array_dim),
            Type::String => StringArray(array_dim),
            Type::Custom(id) => CustomArray(id.clone(), array_dim),
            _ => {
                panic!()
            }
        }
    }
}

pub struct FunctionEntry {
    identifier: String,
    type_signature: (Vec<Type>, Type), // param types -> return type
    table: SymbolTable,
}

impl FunctionEntry {
    pub fn new(ident: &str, ty_sig: (Vec<Type>, Type), table: SymbolTable) -> Self {
        Self {
            identifier: ident.to_string(),
            type_signature: ty_sig,
            table,
        }
    }
}

pub struct ClassEntry {
    identifier: String,
    inherits: Vec<Type>,
    table: SymbolTable,
}

impl ClassEntry {
    pub fn new(ident: &str, inherits: Vec<Type>, table: SymbolTable) -> Self {
        Self {
            identifier: ident.to_string(),
            inherits,
            table,
        }
    }
}

pub struct VariableEntry {
    identifier: String,
    variable_type: Type,
}

impl VariableEntry {
    pub fn new(ident: &str, ty: Type) -> Self {
        Self {
            identifier: ident.to_string(),
            variable_type: ty,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ParameterEntry {
    identifier: String,
    pub(crate) parameter_type: Type,
}

impl ParameterEntry {
    pub fn new(ident: &str, ty: Type) -> Self {
        Self {
            identifier: ident.to_string(),
            parameter_type: ty,
        }
    }
}

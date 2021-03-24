use crate::parser::ast::InternalNodeType::ClassDeclaration;
use crate::parser::ast::{InternalNodeType, Node, NodeVal};
use crate::semantics::symbol_table::Type::{CustomArray, FloatArray, IntegerArray, StringArray};
use crate::semantics::utils::{
    map_class_decl_to_entry, map_func_decl_to_entry, map_func_def_to_entry, map_main_to_scope,
};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Scope {
    Class(ClassEntry),
    Function(FunctionEntry),
    Variable(VariableEntry),
    FunctionParameter(ParameterEntry),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SymbolTable(Vec<Scope>);

impl SymbolTable {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn new_from_scopes(scopes: Vec<Scope>) -> Self {
        Self(scopes)
    }

    pub fn add_scope(&mut self, scope: Scope) {
        self.0.push(scope);
    }

    pub fn add_scopes(&mut self, mut scopes: Vec<Scope>) {
        self.0.append(&mut scopes);
    }

    pub fn merge(&mut self, mut other: SymbolTable) {
        self.0.append(&mut other.0);
    }

    pub fn find_scope_by_ident(&self, ident: &str) -> Option<&Scope> {
        for scope in self.0.iter() {
            match scope {
                Scope::Class(entry) => {
                    return if entry.identifier == ident {
                        Some(scope)
                    } else {
                        entry.table.find_scope_by_ident(ident)
                    }
                }
                Scope::Function(entry) => {
                    return if entry.identifier == ident {
                        Some(scope)
                    } else {
                        entry.table.find_scope_by_ident(ident)
                    }
                }
                Scope::Variable(entry) => {
                    if entry.identifier == ident {
                        return Some(scope);
                    }
                }
                Scope::FunctionParameter(entry) => {
                    if entry.identifier == ident {
                        return Some(scope);
                    }
                }
            }
        }
        None
    }

    pub fn find_scope_by_ident_mut(&mut self, ident: &str) -> Option<&mut Scope> {
        for scope in self.0.iter_mut() {
            //todo
        }
        None
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
//todo visibility
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionEntry {
    identifier: String,
    member_of: Option<String>,
    type_signature: (Vec<Type>, Type), // param types -> return type
    table: SymbolTable,
}

impl FunctionEntry {
    pub fn new(ident: &str, ty_sig: (Vec<Type>, Type), table: SymbolTable) -> Self {
        Self {
            identifier: ident.to_string(),
            member_of: None,
            type_signature: ty_sig,
            table,
        }
    }

    pub fn new_as_member(
        ident: &str,
        class_ident: &str,
        ty_sig: (Vec<Type>, Type),
        table: SymbolTable,
    ) -> Self {
        Self {
            identifier: ident.to_string(),
            member_of: Some(class_ident.to_string()),
            type_signature: ty_sig,
            table,
        }
    }

    pub fn as_member_of(&mut self, member: &str) {
        self.member_of = Some(member.to_string());
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
//todo visibility
#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
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

pub fn generate_symbol_table(root: &Node) -> SymbolTable {
    assert_eq!(root.val, Some(NodeVal::Internal(InternalNodeType::Root)));
    assert_eq!(root.children.len(), 3); // class declarations, func definitions, main

    let mut global = SymbolTable::new();

    let class_scopes: Vec<Scope> = root.children[0]
        .children
        .iter()
        .map(|n| Scope::Class(map_class_decl_to_entry(n)))
        .collect();
    let (free_function_scopes, member_function_scopes): (Vec<FunctionEntry>, Vec<FunctionEntry>) = root.children[1]
        .children
        .iter()
        .map(map_func_def_to_entry)
        .partition(|entry| entry.member_of.is_none());

    let main_scope: Scope = Scope::Function(map_main_to_scope(&root.children[2]));

    global.add_scopes(class_scopes);
    global.add_scopes(free_function_scopes.into_iter().map(Scope::Function).collect()); //todo member function defs should go in the appropriate class symbol table
    for member_func in member_function_scopes
    {
        match global.find_scope_by_ident_mut(&member_func.member_of.unwrap())
        {
            None => { todo!("Semantic Error")}
            Some(scope) => {
                match scope
                {
                    Scope::Class(entry) => {
                        match entry.table.find_scope_by_ident_mut(&member_func.identifier)
                        {
                            None => { todo!("Semantic Error") }
                            Some(fscope) => {
                                match fscope
                                {
                                    Scope::Function(fentry) => { fentry.table.merge(member_func.table); }
                                    _ => { panic!() }
                                }
                            }
                        }
                    }
                    _ => { panic!() }
                }
            }
        }
    }
    global.add_scope(main_scope);
    global
}

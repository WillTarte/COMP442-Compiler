use crate::parser::ast::InternalNodeType::{ClassDeclaration, ClassDeclarations, FunctionDefinitions};
use crate::parser::ast::{InternalNodeType, Node, NodeVal};
use crate::semantics::symbol_table::Type::{CustomArray, FloatArray, IntegerArray, StringArray};
use crate::semantics::utils::{map_class_decl_to_entry, map_func_decl_to_entry, map_func_def_to_entry, map_main_to_func_entry, merge_member_function_tables};
use std::collections::HashMap;
use std::ops::Deref;
use crate::semantics::checking::{check_class_symbol_errors, SemanticError, report_symbol_errors, report_semantic_errors, check_member_func_defined};
use log::{warn};
use std::{io, fmt};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Scope {
    Class(ClassEntry),
    Function(FunctionEntry),
    Variable(VariableEntry),
    FunctionParameter(ParameterEntry),
}

impl Scope {
    pub fn ident(&self) -> &str
    {
        match self
        {
            Scope::Class(e) => { &e.identifier }
            Scope::Function(e) => { &e.identifier }
            Scope::Variable(e) => { &e.identifier }
            Scope::FunctionParameter(e) => { &e.identifier }
        }
    }

    pub fn line_num(&self) -> usize
    {
        match self
        {
            Scope::Class(e) => { e.line_num() }
            Scope::Function(e) => { e.line_num() }
            Scope::Variable(e) => { e.line_num() }
            Scope::FunctionParameter(e) => { e.line_num() }
        }
    }
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
        self.0.append(other.scopes_mut());
        /*let mut errors: Vec<SemanticError> = Vec::new();
        for other_scope in other.scopes_mut().drain(..)
        {
            for scope in self.scopes().iter()
            {
                if scope.ident() == other_scope.ident()
                {
                    errors.push(SemanticError::MultipleDeclIdent(format!("Multiply declared ident {}", other_scope.ident())));
                }
            }
            self.scopes_mut().push(other_scope);
        }
        errors*/
    }

    pub fn find_scope_by_ident(&self, ident: &str) -> Option<&Scope> {
        for scope in self.scopes().iter() {
            if scope.ident() == ident
            {
                return Some(scope);
            }
        }
        None
    }

    pub fn find_scope_by_ident_mut(&mut self, ident: &str) -> Option<&mut Scope> {
        for scope in self.scopes_mut().iter_mut() {
            if scope.ident() == ident
            {
                return Some(scope);
            }
        }
        None
    }

    pub fn find_scope_by_scope(&self, other_scope: &Scope) -> Option<&Scope>
    {
        for scope in self.scopes()
        {
            if scope == other_scope
            {
                return Some(scope);
            }
        }
        None
    }
    pub fn find_scope_by_scope_mut(&mut self, other_scope: &Scope) -> Option<&mut Scope>
    {
        for scope in self.scopes_mut()
        {
            if scope == other_scope
            {
                return Some(scope);
            }
        }
        None
    }

    pub fn scopes(&self) -> &Vec<Scope>
    {
        &self.0
    }

    pub fn scopes_mut(&mut self) -> &mut Vec<Scope>
    {
        &mut self.0
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
    pub fn into_array_type(self, array_dim: Vec<usize>) -> Self {
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

impl Debug for Type
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self
        {
            Type::Integer => { write!(f, "integer") }
            Type::Float => { write!(f, "float") }
            Type::String => { write!(f, "string") }
            Type::Void => { write!(f, "void") }
            Type::IntegerArray(dim) => {
                write!(f, "integer")?;
                for u in dim
                {
                    write!(f, "[{}]", u)?;
                }
                Ok(())
            }
            Type::FloatArray(dim) => {
                write!(f, "float")?;
                for u in dim
                {
                    write!(f, "[{}]", u)?;
                }
                Ok(())
            }
            Type::StringArray(dim) =>
            {
                write!(f, "string")?;
                for u in dim
                {
                    write!(f, "[{}]", u)?;
                }
                Ok(())
            }
            Type::Custom(id) => { write!(f, "{}", id) }
            CustomArray(id, dim) => {
                write!(f, "{}", id)?;
                for u in dim
                {
                    write!(f, "[{}]", u)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Visibility
{
    Private,
    Public,
    Default,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Clone)]
pub struct FunctionEntry {
    identifier: String,
    member_of: Option<String>,
    type_signature: (Vec<Type>, Type), // param types -> return type
    visibility: Visibility,
    table: SymbolTable,
    line_num: usize,
    defined: bool
}

impl FunctionEntry {
    pub fn new(ident: &str, ty_sig: (Vec<Type>, Type), table: SymbolTable, line_num: usize, defined: bool) -> Self {
        Self {
            identifier: ident.to_string(),
            member_of: None,
            type_signature: ty_sig,
            visibility: Visibility::default(),
            table,
            line_num,
            defined
        }
    }

    pub fn new_as_member(
        ident: &str,
        class_ident: &str,
        ty_sig: (Vec<Type>, Type),
        table: SymbolTable,
        line_num: usize,
        defined: bool
    ) -> Self {
        Self {
            identifier: ident.to_string(),
            member_of: Some(class_ident.to_string()),
            type_signature: ty_sig,
            visibility: Visibility::default(),
            table,
            line_num,
            defined
        }
    }

    pub fn as_member_of(&mut self, member: &str) {
        self.member_of = Some(member.to_string());
    }

    pub fn ident(&self) -> &str
    {
        &self.identifier
    }

    pub fn member_of(&self) -> Option<&str>
    {
        match &self.member_of
        {
            None => { None }
            Some(s) => { Some(s.as_str()) }
        }
    }

    pub fn type_sig(&self) -> &(Vec<Type>, Type)
    {
        &self.type_signature
    }

    pub fn table(&self) -> &SymbolTable
    {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut SymbolTable
    {
        &mut self.table
    }

    pub fn visibility(&self) -> Visibility
    {
        self.visibility
    }

    pub fn set_visibility(&mut self, vis: Visibility)
    {
        self.visibility = vis;
    }

    pub fn line_num(&self) -> usize
    {
        self.line_num
    }

    pub fn is_defined(&self) -> bool
    {
        self.defined
    }

    pub fn define(&mut self)
    {
        self.defined = true;
    }

    pub fn merge(&mut self, other: FunctionEntry)
    {
        assert_eq!(self.member_of, other.member_of);
        assert_eq!(self.identifier, other.identifier);
        assert_eq!(self.type_signature, other.type_signature);
        self.define();
        self.table_mut().merge(other.table);
    }
}

impl PartialEq for FunctionEntry
{
    fn eq(&self, other: &Self) -> bool {
        return self.ident() == other.ident() && self.type_sig() == other.type_sig() && self.member_of() == other.member_of();
    }
}

impl Eq for FunctionEntry {}

#[derive(Debug, Clone)]
pub struct ClassEntry {
    identifier: String,
    inherits: Vec<Type>,
    table: SymbolTable,
    line_num: usize,
}

impl ClassEntry {
    pub fn new(ident: &str, inherits: Vec<Type>, table: SymbolTable, line_num: usize) -> Self {
        Self {
            identifier: ident.to_string(),
            inherits,
            table,
            line_num
        }
    }

    pub fn ident(&self) -> &str
    {
        &self.identifier
    }

    pub fn inherits(&self) -> &Vec<Type>
    {
        &self.inherits
    }

    pub fn table(&self) -> &SymbolTable
    {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut SymbolTable
    {
        &mut self.table
    }

    pub fn line_num(&self) -> usize
    {
        self.line_num
    }
}

impl PartialEq for ClassEntry
{
    fn eq(&self, other: &Self) -> bool {
        return self.ident() == other.ident();
    }
}

impl Eq for ClassEntry {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VariableEntry {
    identifier: String,
    variable_type: Type,
    visibility: Visibility,
    line_num: usize,
}

impl VariableEntry {
    pub fn new(ident: &str, ty: Type, line_num: usize) -> Self {
        Self {
            identifier: ident.to_string(),
            variable_type: ty,
            visibility: Visibility::default(),
            line_num
        }
    }

    pub fn ident(&self) -> &str
    {
        &self.identifier
    }

    pub fn var_type(&self) -> &Type
    {
        &self.variable_type
    }

    pub fn visibility(&self) -> Visibility
    {
        self.visibility
    }

    pub fn set_visibility(&mut self, vis: Visibility)
    {
        self.visibility = vis;
    }

    pub fn line_num(&self) -> usize
    {
        self.line_num
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParameterEntry {
    identifier: String,
    parameter_type: Type,
    line_num: usize,
}

impl ParameterEntry {
    pub fn new(ident: &str, ty: Type, line_num: usize) -> Self {
        Self {
            identifier: ident.to_string(),
            parameter_type: ty,
            line_num
        }
    }

    pub fn ident(&self) -> &str
    {
        &self.identifier
    }

    pub fn param_type(&self) -> &Type
    {
        &self.parameter_type
    }

    pub fn line_num(&self) -> usize
    {
        self.line_num
    }
}

pub fn generate_class_entries(node: &Node) -> Vec<ClassEntry>
{
    assert_eq!(node.val, Some(NodeVal::Internal(ClassDeclarations)));

    let entries: Vec<ClassEntry> = node.children.iter().map(map_class_decl_to_entry).collect();

    entries
}

pub fn generate_function_entries(node: &Node) -> (Vec<FunctionEntry>, Vec<FunctionEntry>)
{
    assert_eq!(node.val, Some(NodeVal::Internal(FunctionDefinitions)));

    let entries: (Vec<FunctionEntry>, Vec<FunctionEntry>) = node.children.iter().map(map_func_def_to_entry).partition(|entry| entry.member_of.is_none());

    entries
}

pub fn generate_symbol_table(root: &Node) -> (SymbolTable, Vec<SemanticError>) {
    assert_eq!(root.val, Some(NodeVal::Internal(InternalNodeType::Root)));
    assert_eq!(root.children.len(), 3); // class declarations, func definitions, main

    let mut global_table = SymbolTable::new();

    let class_entries: Vec<ClassEntry> = generate_class_entries(&root.children[0]);

    let (free_function_entries, mut member_function_entries) = generate_function_entries(&root.children[1]);

    let main_entry: FunctionEntry = map_main_to_func_entry(&root.children[2]);

    global_table.add_scopes(class_entries.into_iter().map(Scope::Class).collect());
    global_table.add_scopes(free_function_entries.into_iter().map(Scope::Function).collect());
    global_table.add_scope(Scope::Function(main_entry));

    let mut errors: Vec<SemanticError> = merge_member_function_tables(&mut global_table, &mut member_function_entries);
    errors.append(&mut check_member_func_defined(&global_table));

    (global_table, errors)
}

pub fn check_semantics(root: &Node, global: &SymbolTable) -> Vec<SemanticError>
{
    let mut errors: Vec<SemanticError> = Vec::new();

    errors.append(&mut report_symbol_errors(global));

    errors.append(&mut report_semantic_errors(root, global));

    errors
}
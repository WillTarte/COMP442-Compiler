use crate::semantics::symbol_table::{ClassEntry, SymbolTable, Type, FunctionEntry, Scope};
use crate::semantics::symbol_table::Scope::{Class, Function, FunctionParameter, Variable};
use crate::parser::ast::Node;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SemanticError {
    Warning(WarningType),
    NoMemberFuncDefinition(String),
    NoMemberFuncDeclaration(String),
    MultipleDeclIdent(String),
    MultiplyDeclVariable(String),
    MultiplyDeclMember(String),
    MultiplyDeclClass(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WarningType
{
    OverloadWarning(String),
    ShadowedMemberWarning(String)
}

pub(crate) fn report_symbol_errors(global: &SymbolTable) -> Vec<SemanticError>
{
    let mut errors: Vec<SemanticError> = Vec::new();

    // Check duplicate ident
    errors.append(&mut check_multiply_decl_id(global));

    for scope in global.scopes()
    {
        match scope
        {
            Class(e) => {
                errors.append(&mut check_class_symbol_errors(e, global));
            }
            Function(e) => {
                errors.append(&mut check_function_symbol_errors(e, global));
            }
            _ => {}
        }
    }

    errors
}

pub fn report_semantic_errors(root: &Node, global: &SymbolTable) -> Vec<SemanticError>
{
    let errors: Vec<SemanticError> = Vec::new();

    // Type check expressions (member access, arith expr, rel expr, assign, statements)
    // Check visibility when accessing class members
    // Check func calls, array indexing
    // Check referenced ID for existence


    errors
}

pub fn check_class_symbol_errors(class: &ClassEntry, global: &SymbolTable) -> Vec<SemanticError>
{
    let mut errors: Vec<SemanticError> = Vec::new();

    // Check duplicated members & overloaded members
    errors.append(&mut check_multiply_decl_id(class.table()));
    // Check circular inheritance
    errors.append(&mut check_circular_inheritance(class.inherits(), global));
    // Check shadowed members
    errors.append(&mut check_shadowed_members(class, global));

    // check errors in member functions
    for scope in class.table().scopes()
    {
        match scope {
            Function(e) => {
                errors.append(&mut check_function_symbol_errors(e, global));
            },
            _ => {}
        }
    }

    errors
}

pub fn check_function_symbol_errors(function: &FunctionEntry, global: &SymbolTable) -> Vec<SemanticError>
{
    // check duplicated ident
    return check_multiply_decl_id(function.table())
}


pub(crate) fn check_multiply_decl_id(table: &SymbolTable) -> Vec<SemanticError>
{
    let mut errors: Vec<SemanticError> = Vec::new();

    for idx in 0..(table.scopes().len() - 1)
    {
        for idy in (idx + 1)..table.scopes().len()
        {
            match (&table.scopes()[idx], &table.scopes()[idy])
            {
                (Function(e1), Function(e2)) => {
                    if e1 == e2 //same ID, same type sig, same member_of -> Error
                    {
                        errors.push(SemanticError::MultipleDeclIdent(format!("Multiply declared func ident {}: lines {} and {}", e1.ident(), e1.line_num(), e2.line_num())));
                        continue;
                    }
                    else if e1.ident() == e2.ident() && e1.member_of() == e2.member_of() //overload
                    {
                        errors.push(SemanticError::Warning(WarningType::OverloadWarning(format!("Found func overload for {}: lines {} and {}", e1.ident(), e1.line_num(), e2.line_num()))));
                        continue;
                    }
                },
                (e1, e2) if e1.ident() == e2.ident() => {
                    errors.push(SemanticError::MultipleDeclIdent(format!("Multiply declared ident {}: lines {} and {}", e1.ident(), e1.line_num(), e2.line_num())));
                }
                _ => {}
            }
        }
    }

    errors
}

pub fn check_circular_inheritance(inherits: &Vec<Type>, global: &SymbolTable) -> Vec<SemanticError>
{
    let errors: Vec<SemanticError> = Vec::new();
    todo!();
    errors
}

pub(crate) fn check_shadowed_members(class: &ClassEntry, global: &SymbolTable) -> Vec<SemanticError>
{
    let mut warnings: Vec<SemanticError> = Vec::new();
    todo!();
    warnings
}
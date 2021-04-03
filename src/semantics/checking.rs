use crate::parser::ast::Node;
use crate::semantics::symbol_table::Scope::{Class, Function, FunctionParameter, Variable};
use crate::semantics::symbol_table::{ClassEntry, FunctionEntry, Scope, SymbolTable, Type};
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SemanticError {
    Warning(WarningType),
    NoMemberFuncDefinition(String),
    NoMemberFuncDeclaration(String),
    MultipleDeclIdent(String),
    InheritanceCycle(String),
    //MultiplyDeclVariable(String),
    //MultiplyDeclMember(String),
    //MultiplyDeclClass(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WarningType {
    OverloadWarning(String),
    ShadowedMemberWarning(String),
}

pub(crate) fn report_symbol_errors(global: &SymbolTable) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    // Check duplicate ident
    errors.append(&mut check_multiply_decl_id(global));

    for scope in global.scopes() {
        match scope {
            Class(e) => {
                errors.append(&mut check_class_symbol_errors(e, global));
            }
            Function(e) => {
                errors.append(&mut check_function_symbol_errors(e, global));
            }
            _ => {}
        }
    }
    //todo check if any member functions don't have a definition
    errors
}

pub fn report_semantic_errors(root: &Node, global: &SymbolTable) -> Vec<SemanticError> {
    let errors: Vec<SemanticError> = Vec::new();

    // Type check expressions (member access, arith expr, rel expr, assign, statements)
    // Check visibility when accessing class members
    // Check func calls, array indexing
    // Check referenced ID for existence

    errors
}

pub fn check_class_symbol_errors(class: &ClassEntry, global: &SymbolTable) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    // Check duplicated members & overloaded members
    errors.append(&mut check_multiply_decl_id(class.table()));
    // Check circular inheritance
    errors.append(&mut check_circular_inheritance(class.inherits(), global));
    // Check shadowed members
    errors.append(&mut check_shadowed_members(class, global));

    // check errors in member functions
    for scope in class.table().scopes() {
        match scope {
            Function(e) => {
                errors.append(&mut check_function_symbol_errors(e, global));
            }
            _ => {}
        }
    }

    errors
}

pub fn check_function_symbol_errors(
    function: &FunctionEntry,
    global: &SymbolTable,
) -> Vec<SemanticError> {
    // check duplicated ident
    return check_multiply_decl_id(function.table());
}

pub(crate) fn check_multiply_decl_id(table: &SymbolTable) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    for idx in 0..(table.scopes().len() - 1) {
        for idy in (idx + 1)..table.scopes().len() {
            match (&table.scopes()[idx], &table.scopes()[idy]) {
                (Function(e1), Function(e2)) => {
                    if e1 == e2
                    //same ID, same type sig, same member_of -> Error
                    {
                        errors.push(SemanticError::MultipleDeclIdent(format!(
                            "Multiply declared func ident {}: lines {} and {}",
                            e1.ident(),
                            e1.line_num(),
                            e2.line_num()
                        )));
                        continue;
                    } else if e1.ident() == e2.ident() && e1.member_of() == e2.member_of()
                    //overload
                    {
                        errors.push(SemanticError::Warning(WarningType::OverloadWarning(
                            format!(
                                "Found func overload for {}: lines {} and {}",
                                e1.ident(),
                                e1.line_num(),
                                e2.line_num()
                            ),
                        )));
                        continue;
                    }
                }
                (e1, e2) if e1.ident() == e2.ident() => {
                    errors.push(SemanticError::MultipleDeclIdent(format!(
                        "Multiply declared ident {}: lines {} and {}",
                        e1.ident(),
                        e1.line_num(),
                        e2.line_num()
                    )));
                }
                _ => {}
            }
        }
    }

    errors
}

pub fn check_circular_inheritance(
    inherits: &Vec<Type>,
    global: &SymbolTable,
) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    for scope in global.scopes() {
        match scope {
            Class(e) => {
                let mut inheritance_list: HashSet<&str> = HashSet::new();
                inheritance_list.insert(e.ident());
                for parent in e.inherits() {
                    match parent {
                        Type::Custom(ident) => {
                            if inheritance_list.contains(ident.as_str()) {
                                errors.push(SemanticError::InheritanceCycle(format!(
                                    "Inheritance cycle detected for {} starting from {}.",
                                    ident,
                                    e.ident()
                                )));
                            }
                        }
                        _ => panic!(),
                    }
                }
            }
            _ => {}
        }
    }

    todo!("through data members?");

    errors
}

pub(crate) fn check_member_func_defined(global: &SymbolTable) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();
    for scope in global.scopes() {
        match scope {
            Class(e) => {
                for member in e.table().scopes() {
                    match member {
                        Function(mfunc) => {
                            if !mfunc.is_defined() {
                                errors.push(SemanticError::NoMemberFuncDefinition(format!(
                                    "Member function {}::{} has no valid definition.",
                                    e.ident(),
                                    mfunc.ident()
                                )));
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    errors
}

pub(crate) fn check_shadowed_members(
    class: &ClassEntry,
    global: &SymbolTable,
) -> Vec<SemanticError> {
    let mut warnings: Vec<SemanticError> = Vec::new();

    let mut parents: Vec<&ClassEntry> = Vec::new();

    let mut parent_idents: Vec<&str> = Vec::new();
    for parent_ty in class.inherits() {
        match parent_ty {
            Type::Custom(ident) => {
                parent_idents.push(ident);
            }
            _ => {
                panic!()
            }
        }
    }
    while !parent_idents.is_empty() {
        match global.find_scope_by_ident(parent_idents.pop().unwrap()) {
            None => {
                panic!("oh fuck")
            }
            Some(scope) => match scope {
                Class(e) => {
                    if !parents.contains(&e) {
                        parents.push(e);
                        for parent_id in e.inherits() {
                            match parent_id {
                                Type::Custom(id) => {
                                    parent_idents.push(id);
                                }
                                _ => panic!(),
                            }
                        }
                    }
                }
                _ => {}
            },
        }
    }

    for parent in parents {
        for member in class.table().scopes() {
            match member {
                Function(e) => {
                    for parent_member in parent.table().scopes() {
                        match parent_member {
                            Function(pe) => {
                                if e.ident() == pe.ident()
                                    && e.type_sig() == pe.type_sig()
                                    && e.visibility() == pe.visibility()
                                {
                                    warnings.push(SemanticError::Warning(WarningType::ShadowedMemberWarning(format!("Member function {} of {} is shadowing member function {} of {}", e.ident(), class.ident(), pe.ident(), parent.ident()))))
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Variable(e) => {
                    for parent_member in parent.table().scopes() {
                        match parent_member {
                            Variable(pe) => {
                                if e.ident() == pe.ident() && e.var_type() == pe.var_type() {
                                    warnings.push(SemanticError::Warning(WarningType::ShadowedMemberWarning(format!("Member variable {} of {} is shadowing member variable {} of {}", e.ident(), class.ident(), pe.ident(), parent.ident()))))
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    warnings
}

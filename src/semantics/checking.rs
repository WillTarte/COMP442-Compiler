use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::{InternalNodeType, Node, NodeVal};
use crate::semantics::checking::WarningType::{OverloadWarning, ShadowedMemberWarning};
use crate::semantics::symbol_table::Scope::{Class, Function, FunctionParameter, Variable};
use crate::semantics::symbol_table::Type::{Float, Integer, IntegerArray};
use crate::semantics::symbol_table::{ClassEntry, FunctionEntry, Scope, SymbolTable, Type};
use crate::semantics::utils::{get_ancestors_for_class, map_token_to_type};
use crate::semantics::validation::validate_statement;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SemanticError {
    Warning(WarningType),
    NoMemberFuncDefinition(String),
    NoMemberFuncDeclaration(String),
    MultipleDeclIdent(String),
    InheritanceCycle(String),
    UndeclaredClass(String),
    UndeclaredVariable(String),
    NotIndexable(String),
    TooManyIndices(String),
    FunctionNotFound(String),
    InvalidParameters(String),
    TypeMistmatch(String),
    NotCallable(String),
    RecursionNotSupported(String),
    NotClassType(String),
    //MultiplyDeclVariable(String),
    //MultiplyDeclMember(String),
    //MultiplyDeclClass(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WarningType {
    OverloadWarning(String),
    ShadowedMemberWarning(String),
}

#[allow(dead_code)]
pub(crate) fn report_symbol_errors(global: &SymbolTable) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    log::info!("Checking symbols");

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

    errors
}

#[allow(dead_code)]
pub fn report_semantic_errors(root: &Node, global: &SymbolTable) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    // Type check expressions (member access, arith expr, rel expr, assign, statements)
    // Check visibility when accessing class members
    // Check func calls, array indexing
    // Check referenced ID for existence

    log::info!("Checking semantic errors in function bodies");
    log::warn!("WARNING: IF ANY FUNCTIONS ARE OVERLOADED IT WILL CAUSE ISSUES");

    // function definitions
    for function_definition in root.children()[1].children() {
        match (
            function_definition.children()[0].val(),
            function_definition.children()[1].val(),
        ) {
            (Some(NodeVal::Leaf(token1)), Some(NodeVal::Leaf(token2))) => {
                // member function
                if let Some(Scope::Class(ce)) = global.find_scope_by_ident(token1.lexeme()) {
                    if let Some(Scope::Function(fe)) =
                        ce.table().find_scope_by_ident(token2.lexeme())
                    {
                        for statement in function_definition.children()[4].children()[1].children()
                        {
                            let statement_res = validate_statement(statement, fe, global);
                        }
                    } else {
                        errors.push(SemanticError::FunctionNotFound(format!(
                            "No member function {} found in {}: line {}",
                            token2.lexeme(),
                            ce.ident(),
                            token2.line_num()
                        )));
                    }
                } else {
                    errors.push(SemanticError::UndeclaredClass(format!(
                        "Undeclared class {}: line {}",
                        token1.lexeme(),
                        token1.line_num()
                    )));
                }
            }
            (Some(NodeVal::Leaf(token1)), None) => {
                // free
                if let Some(Scope::Function(fe)) = global.find_scope_by_ident(token1.lexeme()) {
                    for statement in function_definition.children()[4].children()[1].children() {
                        let statement_res = validate_statement(statement, fe, global);
                        if statement_res.is_err() {
                            errors.push(statement_res.unwrap_err());
                        }
                    }
                } else {
                    errors.push(SemanticError::FunctionNotFound(format!(
                        "No function {} found: line {}",
                        token1.lexeme(),
                        token1.line_num()
                    )));
                }
            }
            _ => panic!(),
        }
    }

    if let Some(Scope::Function(main)) = global.find_scope_by_ident("main") {
        for statement in root.children()[2].children()[0].children()[1].children() {
            let statement_res = validate_statement(statement, main, global);
            if statement_res.is_err() {
                errors.push(statement_res.unwrap_err());
            }
        }
    } else {
        panic!();
    }

    errors
}

#[allow(dead_code)]
pub fn check_class_symbol_errors(class: &ClassEntry, global: &SymbolTable) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    log::info!("Checking symbols in class {}", class.ident());

    // Check duplicated members & overloaded members
    errors.append(&mut check_multiply_decl_id(class.table()));
    // Check if member functions are defined
    errors.append(&mut check_member_func_defined(class));
    // Check undeclared types usage
    errors.append(&mut check_undeclared_types_usage_class(class, global));
    // Check circular inheritance
    errors.append(&mut check_circular_inheritance(class, global));
    // Check circular data member dependencies
    errors.append(&mut check_circular_data_member_dependencies(class, global));
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

#[allow(dead_code)]
pub fn check_function_symbol_errors(
    function: &FunctionEntry,
    global: &SymbolTable,
) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    log::info!("Checking symbols in function {}", function.ident());

    // check duplicated ident
    errors.append(&mut check_multiply_decl_id(function.table()));
    // check undeclared type usage
    errors.append(&mut check_undeclared_types_usage_fn(function, global));

    errors
}

#[allow(dead_code)]
pub(crate) fn check_multiply_decl_id(table: &SymbolTable) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    log::info!("Checking multiply declared identifiers");

    if table.scopes().len() == 0 {
        return errors;
    }
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

#[allow(dead_code)]
pub(crate) fn check_undeclared_types_usage_fn(
    entry: &FunctionEntry,
    global: &SymbolTable,
) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    log::info!(
        "Checking undeclared type usages in function {}",
        entry.ident()
    );

    for scope in entry.table().scopes() {
        match scope {
            Variable(e) => {
                match e.var_type() {
                    Type::Custom(ident) | Type::CustomArray(ident, _) => {
                        if let Some(found) = global.find_scope_by_ident(ident) {
                            match found {
                                Class(_) => { /* good */ }
                                _ => {
                                    panic!()
                                }
                            }
                        } else {
                            errors.push(SemanticError::UndeclaredClass(format!(
                                "Variable {} in fn {} has undeclared type {}",
                                e.ident(),
                                entry.ident(),
                                ident
                            )))
                        }
                    }
                    _ => {}
                }
            }
            FunctionParameter(e) => {
                match e.param_type() {
                    Type::Custom(ident) | Type::CustomArray(ident, _) => {
                        if let Some(found) = global.find_scope_by_ident(ident) {
                            match found {
                                Class(_) => { /* good */ }
                                _ => {
                                    panic!()
                                }
                            }
                        } else {
                            errors.push(SemanticError::UndeclaredClass(format!(
                                "Func param {} in fn {} has undeclared type {}",
                                e.ident(),
                                entry.ident(),
                                ident
                            )))
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    errors
}

#[allow(dead_code)]
pub(crate) fn check_undeclared_types_usage_class(
    entry: &ClassEntry,
    global: &SymbolTable,
) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    log::info!("Checking undeclared type usages in class {}", entry.ident());

    for scope in entry.table().scopes() {
        match scope {
            Variable(e) => {
                match e.var_type() {
                    Type::Custom(ident) | Type::CustomArray(ident, _) => {
                        if let Some(found) = global.find_scope_by_ident(ident) {
                            match found {
                                Class(_) => { /* good */ }
                                _ => {
                                    panic!()
                                }
                            }
                        } else {
                            errors.push(SemanticError::UndeclaredClass(format!(
                                "Variable {} in fn {} has undeclared type {}",
                                e.ident(),
                                entry.ident(),
                                ident
                            )))
                        }
                    }
                    _ => {}
                }
            }
            Function(e) => {
                errors.append(&mut check_undeclared_types_usage_fn(e, global));
            }
            _ => {}
        }
    }

    errors
}

#[allow(dead_code)]
pub fn check_circular_inheritance(class: &ClassEntry, global: &SymbolTable) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    let (ancestors, mut errors1) = get_ancestors_for_class(class, global);
    errors.append(&mut errors1);

    if ancestors.iter().any(|a| a.ident() == class.ident()) {
        errors.push(SemanticError::InheritanceCycle(format!(
            "Circular inheritance detected starting from {}",
            class.ident()
        )));
    }

    errors
}

#[allow(dead_code)]
pub(crate) fn check_circular_data_member_dependencies(
    class: &ClassEntry,
    global: &SymbolTable,
) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    log::error!("check_circular_data_member_dependencies : NOT IMPLEMENTED"); //todo

    errors
}

#[allow(dead_code)]
pub(crate) fn check_member_func_defined(class: &ClassEntry) -> Vec<SemanticError> {
    let mut errors: Vec<SemanticError> = Vec::new();

    log::info!("Checking if member functions are defined");

    for member in class.table().scopes() {
        match member {
            Function(mfunc) => {
                if !mfunc.is_defined() {
                    errors.push(SemanticError::NoMemberFuncDefinition(format!(
                        "Member function {}::{} has no valid definition.",
                        class.ident(),
                        mfunc.ident()
                    )));
                }
            }
            _ => {}
        }
    }

    errors
}

#[allow(dead_code)]
pub(crate) fn check_shadowed_members(
    class: &ClassEntry,
    global: &SymbolTable,
) -> Vec<SemanticError> {
    let mut warnings: Vec<SemanticError> = Vec::new();

    log::info!("Checking if any members are shadowed");

    let (parents, mut errors) = get_ancestors_for_class(class, global);
    warnings.append(&mut errors);

    for parent in parents {
        if class.ident() != parent.ident()
        // In case of circular inheritance
        {
            warnings.append(&mut check_shadowed_members_priv(class, parent));
        }
    }

    warnings
}

#[allow(dead_code)]
fn check_shadowed_members_priv(class: &ClassEntry, ancestor: &ClassEntry) -> Vec<SemanticError> {
    let mut warnings: Vec<SemanticError> = Vec::new();

    for member in class.table().scopes() {
        match member {
            Variable(e) => {
                for parent_member in ancestor.table().scopes() {
                    match parent_member {
                        Variable(pe) => {
                            if e.ident() == pe.ident() {
                                warnings.push(SemanticError::Warning(ShadowedMemberWarning(
                                    format!(
                                    "{}'s member variable {} is shadowing {}'s member variable {}",
                                    class.ident(),
                                    e.ident(),
                                    ancestor.ident(),
                                    pe.ident()
                                ),
                                )));
                            }
                        }
                        _ => {}
                    }
                }
            }
            Function(e) => {
                for parent_member in ancestor.table().scopes() {
                    match parent_member {
                        Function(pe) => {
                            if e.ident() == pe.ident() {
                                warnings.push(SemanticError::Warning(ShadowedMemberWarning(
                                    format!(
                                        "{}'s member func {} is shadowing {}'s member func {}",
                                        class.ident(),
                                        e.ident(),
                                        ancestor.ident(),
                                        pe.ident()
                                    ),
                                )));
                                if e.type_sig() != pe.type_sig() {
                                    warnings.push(SemanticError::Warning(OverloadWarning(
                                        format!(
                                        "{}'s member func {} is overloading {}'s member func {}",
                                        class.ident(),
                                        e.ident(),
                                        ancestor.ident(),
                                        pe.ident()
                                    ),
                                    )));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    warnings
}

#[allow(dead_code)]
pub(crate) fn check_func_def_semantics(def: &Node, global: &SymbolTable) -> Vec<SemanticError> {
    assert_eq!(
        def.val(),
        Some(&NodeVal::Internal(InternalNodeType::FuncDef))
    );
    let mut errors: Vec<SemanticError> = Vec::new();

    let (func_decl, opt_class) = match (def.children()[0].val(), def.children()[1].val()) {
        (Some(NodeVal::Leaf(t1)), Some(NodeVal::Leaf(t2))) => {
            let class = match global.find_scope_by_ident(t1.lexeme()) {
                Some(Class(e)) => e,
                _ => {
                    return errors;
                }
            };
            match class.table().find_scope_by_ident(t2.lexeme()) {
                Some(Function(e)) => (e, Some(class)),
                _ => {
                    return errors;
                }
            }
        }
        (Some(NodeVal::Leaf(t1)), None) => match global.find_scope_by_ident(t1.lexeme()) {
            Some(Function(e)) => (e, None),
            _ => {
                return errors;
            }
        },
        _ => {
            panic!()
        }
    };

    errors
}

use crate::lexer::token::TokenType;
use crate::lexer::token::TokenType::Func;
use crate::parser::ast::{InternalNodeType, Node, NodeVal};
use crate::semantics::symbol_table;
use crate::semantics::symbol_table::Type::{CustomArray, IntegerArray};
use crate::semantics::symbol_table::{
    ClassEntry, FunctionEntry, ParameterEntry, Scope, SymbolTable, Type, VariableEntry,
};

/// ClassDeclaration node
pub fn map_class_decl_to_entry(node: &Node) -> ClassEntry {
    assert_eq!(
        node.val,
        Some(NodeVal::Internal(InternalNodeType::ClassDeclaration))
    );
    match &node.val {
        None => {
            panic!()
        }
        Some(v) => {
            match v {
                NodeVal::Leaf(_) => {
                    panic!()
                }
                NodeVal::Internal(ty) => {
                    match ty {
                        InternalNodeType::ClassDeclaration => {
                            assert_eq!(node.children.len(), 3); // class name, inherit list, member list
                            let ident = match node.children[0].val.as_ref().unwrap() {
                                NodeVal::Leaf(t) => t.lexeme(),
                                NodeVal::Internal(_) => {
                                    panic!()
                                }
                            };
                            let inherits: Vec<Type> =
                                node.children[1].children.iter().map(map_to_type).collect();

                            let mut members: Vec<Scope> = node.children[2]
                                .children
                                .iter()
                                .map(map_member_to_scope)
                                .collect();
                            for scope in members.iter_mut()
                            //todo a bit janky
                            {
                                match scope {
                                    Scope::Function(entry) => {
                                        entry.as_member_of(ident);
                                    }
                                    _ => {}
                                }
                            }
                            ClassEntry::new(ident, inherits, SymbolTable::new_from_scopes(members))
                        }
                        _ => {
                            panic!();
                        }
                    }
                }
            }
        }
    }
}

/// FuncDeclaration node
pub(crate) fn map_func_decl_to_entry(node: &Node) -> FunctionEntry {
    assert_eq!(
        node.val,
        Some(NodeVal::Internal(InternalNodeType::FuncDeclaration))
    );
    assert_eq!(node.children.len(), 3);
    match &node.val {
        None => {
            panic!()
        }
        Some(node_val) => match node_val {
            NodeVal::Leaf(_) => {
                panic!()
            }
            NodeVal::Internal(InternalNodeType::FuncDeclaration) => {
                let ident = match &node.children[0].val {
                    None => {
                        panic!()
                    }
                    Some(v) => match v {
                        NodeVal::Leaf(t) => t.lexeme(),
                        NodeVal::Internal(_) => {
                            panic!()
                        }
                    },
                };
                let params: Vec<ParameterEntry> = node.children[1]
                    .children
                    .iter()
                    .map(map_func_param_to_entry)
                    .collect();
                let return_ty = map_to_type(&node.children[2]);
                let ty_signature: (Vec<Type>, Type) = (
                    params.iter().map(|p| p.parameter_type.clone()).collect(),
                    return_ty,
                );
                let symbol_table = SymbolTable::new_from_scopes(
                    params
                        .into_iter()
                        .map(|p| Scope::FunctionParameter(p))
                        .collect(),
                );
                FunctionEntry::new(ident, ty_signature, symbol_table)
            }
            _ => {
                panic!()
            }
        },
    }
}

/// FuncDef node
pub(crate) fn map_func_def_to_entry(node: &Node) -> FunctionEntry {
    assert_eq!(node.val, Some(NodeVal::Internal(InternalNodeType::FuncDef)));
    assert_eq!(node.children.len(), 5);

    let (ident1, ident2) = match (&node.children[0].val, &node.children[1].val) {
        (Some(val1), Some(val2)) => match (val1, val2) {
            (NodeVal::Leaf(t1), NodeVal::Leaf(t2)) => (t1.lexeme(), Some(t2.lexeme())),
            (_, _) => {
                panic!()
            }
        },
        (Some(val1), None) => match val1 {
            NodeVal::Leaf(t) => (t.lexeme(), None),
            NodeVal::Internal(_) => {
                panic!()
            }
        },
        (_, _) => {
            panic!()
        }
    };
    let params: Vec<ParameterEntry> = node.children[2]
        .children
        .iter()
        .map(map_func_param_to_entry)
        .collect();
    let return_ty = map_to_type(&node.children[3]);
    let ty_signature: (Vec<Type>, Type) = (
        params.iter().map(|p| p.parameter_type.clone()).collect(),
        return_ty,
    );
    let mut symbol_table = SymbolTable::new_from_scopes(
        params
            .into_iter()
            .map(|p| Scope::FunctionParameter(p))
            .collect(),
    );
    let body_vars: Vec<VariableEntry> = node.children[4]
        .children
        .iter()
        .map(map_var_decl_to_entry)
        .collect();
    symbol_table.add_scopes(body_vars.into_iter().map(Scope::Variable).collect());
    match ident2 {
        None => FunctionEntry::new(ident1, ty_signature, symbol_table),
        Some(ident) => FunctionEntry::new_as_member(ident, ident1, ty_signature, symbol_table),
    }
}

/// Main node
pub(crate) fn map_main_to_scope(node: &Node) -> FunctionEntry {
    assert_eq!(node.val, Some(NodeVal::Internal(InternalNodeType::Main)));
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0].children.len(), 1);
    let var_scopes: Vec<Scope> = node.children[0].children[0]
        .children
        .iter()
        .map(|n| Scope::Variable(map_var_decl_to_entry(n)))
        .collect();
    FunctionEntry::new(
        "main",
        (Vec::new(), Type::Void),
        SymbolTable::new_from_scopes(var_scopes),
    )
}

/// VarDeclaration node
pub(crate) fn map_var_decl_to_entry(node: &Node) -> VariableEntry {
    assert_eq!(
        node.val,
        Some(NodeVal::Internal(InternalNodeType::VarDeclaration))
    );
    assert_eq!(node.children.len(), 3);
    let ty = map_to_type(&node);
    let ident = match &node.children[1].val {
        None => {
            panic!()
        }
        Some(node_val) => match node_val {
            NodeVal::Leaf(t) => t.lexeme(),
            NodeVal::Internal(_) => {
                panic!()
            }
        },
    };
    VariableEntry::new(ident, ty)
}

/// FuncParam node
pub(crate) fn map_func_param_to_entry(node: &Node) -> ParameterEntry {
    assert_eq!(
        node.val,
        Some(NodeVal::Internal(InternalNodeType::FuncParam))
    );
    assert_eq!(node.children.len(), 3);
    let ty = map_to_type(&node);
    let ident = match &node.children[1].val {
        None => {
            panic!()
        }
        Some(node_val) => match node_val {
            NodeVal::Leaf(t) => t.lexeme(),
            NodeVal::Internal(_) => {
                panic!()
            }
        },
    };
    ParameterEntry::new(ident, ty)
}

/// Extracts type information from a token, or from VarDeclaration/FuncParam
pub(crate) fn map_to_type(node: &Node) -> symbol_table::Type {
    match &node.val {
        None => {
            panic!()
        }
        Some(node_val) => match node_val {
            NodeVal::Leaf(t) => match t.token_type() {
                TokenType::IntegerType => symbol_table::Type::Integer,
                TokenType::FloatType => symbol_table::Type::Float,
                TokenType::StringType => symbol_table::Type::String,
                TokenType::Id => symbol_table::Type::Custom(t.lexeme().to_string()),
                TokenType::Void => return symbol_table::Type::Void,
                _ => {
                    panic!()
                }
            },
            NodeVal::Internal(InternalNodeType::FuncParam)
            | NodeVal::Internal(InternalNodeType::VarDeclaration) => {
                assert_eq!(node.children.len(), 3);
                assert_eq!(
                    node.children[2].val,
                    Some(NodeVal::Internal(InternalNodeType::ArrayDim))
                );
                let ty: symbol_table::Type = match &node.children[0].val {
                    None => {
                        panic!()
                    }
                    Some(node_val) => match node_val {
                        NodeVal::Leaf(t) => match t.token_type() {
                            TokenType::IntegerType => symbol_table::Type::Integer,
                            TokenType::FloatType => symbol_table::Type::Float,
                            TokenType::StringType => symbol_table::Type::String,
                            TokenType::Id => symbol_table::Type::Custom(t.lexeme().to_string()),
                            _ => {
                                panic!()
                            }
                        },
                        NodeVal::Internal(_) => {
                            panic!()
                        }
                    },
                };
                return if node.children[2].children.is_empty() {
                    ty
                } else {
                    let array_dim: Vec<usize> = node.children[2]
                        .children
                        .iter()
                        .map(map_to_usize)
                        .filter_map(|o| o)
                        .collect();
                    ty.as_array_type(array_dim)
                };
            }
            _ => {
                panic!()
            }
        },
    }
}

/// Extracts usize from tokens
pub(crate) fn map_to_usize(node: &Node) -> Option<usize> // e.g. ArrayDim children are integer tokens
{
    match &node.val {
        None => None,
        Some(v) => {
            match v {
                NodeVal::Leaf(t) => {
                    Some(t.lexeme().parse::<usize>().unwrap()) // todo this is yikes
                }
                NodeVal::Internal(_) => {
                    panic!()
                }
            }
        }
    }
}

pub(crate) fn map_member_to_scope(node: &Node) -> Scope {
    assert_eq!(
        node.val,
        Some(NodeVal::Internal(InternalNodeType::MemberDeclaration))
    );
    match &node.val {
        None => {
            panic!()
        }
        Some(node_val) => match node_val {
            NodeVal::Leaf(_) => {
                panic!()
            }
            NodeVal::Internal(InternalNodeType::MemberDeclaration) => match &node.children[0].val {
                None => {
                    panic!()
                }
                Some(v) => match v {
                    NodeVal::Leaf(_) => {
                        panic!()
                    }
                    NodeVal::Internal(InternalNodeType::MemberVarDeclaration) => {
                        Scope::Variable(map_var_decl_to_entry(&node.children[0].children[0]))
                    }
                    NodeVal::Internal(InternalNodeType::MemberFuncDeclaration) => {
                        Scope::Function(map_func_decl_to_entry(&node.children[0].children[0]))
                    }
                    _ => {
                        panic!()
                    }
                },
            },
            _ => {
                panic!()
            }
        },
    }
}

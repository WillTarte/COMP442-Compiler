use crate::lexer::token::TokenType;
use crate::parser::ast::{InternalNodeType, Node, NodeVal};
use crate::semantics::symbol_table;
use crate::semantics::symbol_table::Scope::FunctionParameter;
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

                            let members: Vec<Scope> = node.children[2]
                                .children
                                .iter()
                                .map(map_member_to_scope)
                                .collect();
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
                    params.into_iter().map(|p| FunctionParameter(p)).collect(),
                );
                FunctionEntry::new(ident, ty_signature, symbol_table)
            }
            _ => {
                panic!()
            }
        },
    }
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

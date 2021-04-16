use crate::parser::ast::{InternalNodeType, Node, NodeVal};
use crate::semantics::symbol_table::{
    ClassEntry, FunctionEntry, ParameterEntry, Scope, SymbolTable, Type, VariableEntry,
};

/// Returns the size of the given type in bytes
pub fn sizeof(t: &Type, symbols: &SymbolTable) -> u32 {
    let size: u32 = match t {
        Type::Integer => 4,
        Type::IntegerArray(dim) => 4u32 * dim.iter().product::<u32>(),
        Type::Float => 4,
        Type::FloatArray(dim) => 8u32 * dim.iter().product::<u32>(),
        Type::String => {
            todo!()
        }
        Type::StringArray(dim) => 0u32 * dim.iter().product::<u32>(),
        Type::Custom(ident) => {
            let mut temp_size: u32 = 0;
            if let Some(Scope::Class(ce)) = symbols.find_scope_by_ident(ident) {
                for scope in ce.table().scopes() {
                    if let Scope::Variable(ve) = scope {
                        temp_size += sizeof(ve.var_type(), symbols);
                    } else if let Scope::Function(fe) = scope {
                        //todo!()
                    }
                }
            } else {
                panic!()
            }
            temp_size
        }
        Type::CustomArray(ident, dim) => {
            let mut temp_size: u32 = 0;
            if let Some(Scope::Class(ce)) = symbols.find_scope_by_ident(ident) {
                for scope in ce.table().scopes() {
                    if let Scope::Variable(ve) = scope {
                        temp_size += sizeof(ve.var_type(), symbols);
                    } else if let Scope::Function(fe) = scope {
                        todo!()
                    }
                }
            } else {
                panic!()
            }
            temp_size * dim.iter().product::<u32>()
        }
        Type::Void => 0,
    };
    size
}

pub fn generate_arith_expr_postfix<'a>(arith_expr: &'a Node, acc: &mut Vec<&'a Node>) {
    assert_eq!(
        arith_expr.val(),
        Some(&NodeVal::Internal(InternalNodeType::ArithExpr))
    );
    return post_order_traversal(arith_expr, acc);
}

fn post_order_traversal<'a>(root: &'a Node, acc: &mut Vec<&'a Node>) {
    // recurse on left subtree
    if root.children().len() > 0 {
        post_order_traversal(&root.children()[0], acc);
    }
    // recurse on right subtree
    if root.children().len() > 1 {
        post_order_traversal(&root.children()[1], acc);
    }
    // add to accumulator
    if is_arith_operator(root) || is_arith_operand(root) {
        acc.push(root)
    }
}

fn is_arith_expr_token(v: Option<&NodeVal>) -> bool {
    match v {
        None => false,
        Some(val) => {
            match val {
                NodeVal::Leaf(_) => true,
                NodeVal::Internal(internal) => {
                    match internal
                    {
                        InternalNodeType::Add
                        | InternalNodeType::Sub
                        | InternalNodeType::Or
                        //InternalNodeType::Negation => {}
                        | InternalNodeType::SignedFactor
                        | InternalNodeType::Mult
                        | InternalNodeType::Div
                        | InternalNodeType::And
                        //InternalNodeType::Equal => {}
                        //InternalNodeType::NotEqual => {}
                        //InternalNodeType::GreaterThan => {}
                        //InternalNodeType::LessThan => {}
                        //InternalNodeType::GreaterEqualThan => {}
                        //InternalNodeType::LessEqualThan => {}
                        | InternalNodeType::DotOp => { true }
                        _ => false

                    }
                }
            }
        }
    }
}

pub fn is_arith_operator(node: &Node) -> bool {
    match node.val() {
        Some(NodeVal::Internal(internal)) => match internal {
            InternalNodeType::Add
            | InternalNodeType::Sub
            | InternalNodeType::Or
            | InternalNodeType::Mult
            | InternalNodeType::Div
            | InternalNodeType::And => true,
            _ => false,
        },
        _ => false,
    }
}

pub fn is_arith_operand(node: &Node) -> bool {
    match node.val() {
        Some(NodeVal::Internal(internal)) => match internal {
            InternalNodeType::SignedFactor | InternalNodeType::DotOp => true,
            _ => false,
        },
        Some(NodeVal::Leaf(_)) => true,
        _ => false,
    }
}

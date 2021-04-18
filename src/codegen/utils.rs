use crate::codegen::generator::CodeGenOutput;
use crate::parser::ast::{InternalNodeType, Node, NodeVal};
use crate::semantics::symbol_table::{Scope, SymbolTable, Type};
use std::fs::OpenOptions;
use std::io;
use std::io::{BufWriter, Write};
use crate::semantics::utils::get_class_hierarchy_data_members;

/// Returns the size of the given type in bytes
pub fn sizeof(t: &Type, symbols: &SymbolTable) -> u32 {
    let size: u32 = match t {
        Type::Integer => 4,
        Type::IntegerArray(dim) => {
            let size= 4u32 * dim.iter().product::<u32>();
            if size == 0 { 4 } else { size }
        },
        Type::Float => 4,
        Type::FloatArray(dim) => 8u32 * dim.iter().product::<u32>(),
        Type::String => {
            unimplemented!("Sizeof for string is not implemented")
        }
        Type::StringArray(dim) => 0u32 * dim.iter().product::<u32>(),
        Type::Custom(ident) => {
            let mut temp_size: u32 = 0;
            if let Some(Scope::Class(ce)) = symbols.find_scope_by_ident(ident) {
                let (all_members, _) = get_class_hierarchy_data_members(ce, symbols);
                for member in all_members
                {
                    temp_size += sizeof(member.var_type(), symbols);
                }
            } else {
                panic!()
            }
            temp_size
        }
        Type::CustomArray(ident, dim) => {
            let mut temp_size: u32 = 0;
            if let Some(Scope::Class(ce)) = symbols.find_scope_by_ident(ident) {
                let (all_members, _) = get_class_hierarchy_data_members(ce, symbols);
                for member in all_members
                {
                    temp_size += sizeof(member.var_type(), symbols);
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
    match root.val() {
        Some(NodeVal::Internal(InternalNodeType::DotOp))
        | Some(NodeVal::Internal(InternalNodeType::SignedFactor))
        | Some(NodeVal::Internal(InternalNodeType::TernaryOperation))
        | Some(NodeVal::Leaf(_)) => {
            acc.push(root);
            return;
        }
        _ => {}
    }

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
            InternalNodeType::SignedFactor
            | InternalNodeType::DotOp
            | InternalNodeType::TernaryOperation => true,
            _ => false,
        },
        Some(NodeVal::Leaf(_)) => true,
        _ => false,
    }
}

pub fn write_moon_code_to_file(code: CodeGenOutput, file_name: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}.m", file_name))?;
    let mut buf_writer = BufWriter::new(file);

    for line in code.into_iter() {
        buf_writer.write(format!("{}\n", line.to_string()).as_bytes())?;
    }
    buf_writer.flush()?;

    Ok(())
}

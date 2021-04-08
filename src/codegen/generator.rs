use crate::parser::ast::{Node};
use crate::semantics::symbol_table::{SymbolTable, Type};

#[derive(Default)]
struct CodeGenerator;
pub struct CodeGenOutput;

pub struct MoonGenerator
{
    ast: Node,
    symbols: SymbolTable,
    allocator: Allocator,
    generator: CodeGenerator,
}

impl MoonGenerator
{
    pub fn new(root: Node, symbols: SymbolTable) -> Self
    {
        Self
        {
            ast: root,
            symbols,
            allocator: Allocator::default(),
            generator: CodeGenerator::default()
        }
    }

    pub fn generate(&mut self) -> CodeGenOutput
    {
        todo!()
    }
}

#[derive(Default)]
struct Allocator
{

}

impl Allocator
{

}

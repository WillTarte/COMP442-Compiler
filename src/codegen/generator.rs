use crate::parser::ast::{Node, NodeVal, InternalNodeType};
use crate::semantics::symbol_table::{SymbolTable, Type, Scope, FunctionEntry, ClassEntry, ParameterEntry, VariableEntry};
use crate::codegen::instruction_set::{Instruction, TaggedInstruction};
use crate::codegen::utils::{sizeof};
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::codegen::instruction_set::Register::*;
use crate::codegen::instruction_set::Instruction::JumpLabel;


#[derive(Default)]
pub struct CodeGenOutput(Vec<TaggedInstruction>);

impl CodeGenOutput
{
    pub fn append(&mut self, mut other: CodeGenOutput)
    {
        self.0.append(&mut other.0)
    }
}

impl Display for CodeGenOutput
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for inst in self.0.iter()
        {
            write!(f, "{}\n", inst.to_string())?;
        }
        Ok(())
    }
}

#[derive(Default)]
struct CodeGenerator {
    output: CodeGenOutput
}

impl CodeGenerator {
    pub fn new() -> Self
    {
        Self
        {
            output: CodeGenOutput::default()
        }
    }

    pub fn add_tagged_resource(&mut self, tag: &str, bytes: u32)
    {
        self.output.0.push(TaggedInstruction(Some(tag.to_string()), Instruction::Res(bytes)));
    }

    pub fn add_instruction(&mut self, inst: Instruction)
    {
        self.output.0.push(TaggedInstruction(None, inst));
    }

    pub fn add_tagged_instruction(&mut self, t_inst: TaggedInstruction)
    {
        self.output.0.push(t_inst);
    }

    pub fn buffer_label(&mut self, label: &str)
    {
        todo!()
    }
}



pub struct MoonGenerator
{
    // ast: Node,
    //symbols: SymbolTable,
    allocator: Allocator,
    generator: CodeGenerator,
}

impl MoonGenerator
{
    pub fn new() -> Self
    {
        Self
        {
            allocator: Allocator::default(),
            generator: CodeGenerator::default()
        }
    }

    pub fn generate(&mut self, ast: &Node, symbols: &SymbolTable)
    {
        //visit root children
        self.visit_class_declarations(&ast.children()[0], symbols);
        self.visit_function_definitions(&ast.children()[1], symbols);
        self.visit_main_function(&ast.children()[2], symbols);

        // do root code generation
    }

    pub fn finish(self) -> CodeGenOutput
    {
        self.generator.output
    }


    fn visit_class_declarations(&mut self, class_declaration: &Node, symbols: &SymbolTable) {
        log::error!("CODE GENERATION FOR CLASSES: NOT IMPLEMENTED");
    }

    fn visit_function_definitions(&mut self, function_definitions: &Node, symbols: &SymbolTable) {
        log::info!("CODE GENERATION FOR FUNCTIONS: WIP");
        //visit each function
        for function_def in function_definitions.children()
        {
            if let Some(NodeVal::Leaf(token)) = function_def.children()[0].val()
            {
                if let Some(Scope::Function(fe)) = symbols.find_scope_by_ident(token.lexeme())
                {
                    self.generate_function_code(fe, function_def, symbols);
                }
            }
        }
        // do global function definitions setup?
    }

    fn visit_main_function(&mut self, main_func: &Node, symbols: &SymbolTable) {
        log::error!("CODE GENERATION FOR MAIN: WIP");

        if let Some(Scope::Function(main)) = symbols.find_scope_by_ident("main")
        {
            for scope in main.table().scopes()
            {
                if let Scope::Variable(ve) = scope
                {
                    let tag: String = format!("var_{}_{}", main.ident(), ve.ident());
                    self.generator.add_tagged_resource(&tag, sizeof(ve.var_type(), symbols));
                }
            }
        }

        // Generate code for main function body
        self.generator.add_tagged_instruction(TaggedInstruction(Some("fn_main".to_string()), Instruction::Entry));
        todo!("func body");

        return;
    }

    fn generate_function_code(&mut self, fe: &FunctionEntry, function_definition: &Node, symbols: &SymbolTable)
    {
        // reserve memory for function return value
        self.generator.add_tagged_resource(&format!("fnres_{}", fe.ident()), sizeof(&fe.type_sig().1, symbols));

        // Allocate resources for the function parameters and local variables
        for scope in fe.table().scopes().iter()
        {
            //todo we need to store the labels somewhere so that they are easily accessible
            if let Scope::FunctionParameter(fpe) = scope
            {
                let tag: String = format!("param_{}_{}", fe.ident(), fpe.ident());
                self.generator.add_tagged_resource(&tag, sizeof(fpe.param_type(), symbols));
            }
            else if let Scope::Variable(ve) = scope
            {
                let tag: String = format!("var_{}_{}", fe.ident(), ve.ident());
                self.generator.add_tagged_resource(&tag, sizeof(ve.var_type(), symbols));
            }
        }

        // Add function body
        //self.generator.add_tagged_instruction(TaggedInstruction(Some(format!("fn_{}",fe.ident())), Instruction::NoOp));
        self.generate_statement_block_code(&function_definition.children()[4].children()[1], fe, symbols); //Statement list

        // reset local variables
        for scope in fe.table().scopes().iter()
        {
            if let Scope::Variable(ve) = scope
            {
                self.generator.add_instruction(Instruction::StoreWordLabel(R0, R0, format!("var_{}_{}", fe.ident(), ve.ident())));
            }
        }

        // jump back to callee, assuming address will be stored in R15
        self.generator.add_instruction(Instruction::JumpRegister(R15));
        return;
    }

    fn generate_statement_block_code(&mut self, statement_block: &Node, fe: &FunctionEntry, symbols: &SymbolTable)
    {
        for statement in statement_block.children()
        {
            match statement.val()
            {
                None => {}
                Some(statement_type) => {
                    match statement_type
                    {
                        NodeVal::Leaf(_) => { panic!() }
                        NodeVal::Internal(ty) => {
                            match ty
                            {
                                InternalNodeType::WhileStatement => {
                                    self.generate_while_statement_code(statement, fe, symbols);
                                },
                                InternalNodeType::IfStatement => {
                                    self.generate_if_statement_code(statement, fe, symbols);
                                },
                                InternalNodeType::ReadStatement => {
                                    self.generate_read_statement_code(statement, fe, symbols);
                                },
                                InternalNodeType::WriteStatement => {
                                    self.generate_write_statement_code(statement, fe, symbols);
                                },
                                InternalNodeType::ReturnStatement => {
                                    self.generate_return_statement_code(statement, fe, symbols);
                                },
                                InternalNodeType:: BreakStatement => { /*panic!()*/ },
                                InternalNodeType::ContinueStatement => { /*panic!()*/ },
                                InternalNodeType::GenericStatement => {
                                    todo!("generate_generic_statement_code()");
                                },
                                _ => panic!()
                            }
                        }
                    }
                }
            }
        }
    }

    fn generate_while_statement_code(&mut self, while_statement: &Node, fe: &FunctionEntry, symbols: &SymbolTable)
    {
        /*
        gowhile1	{code for a<b yields tn as a result}
                    lw r1,tn(r0)
             	    bz r1,endwhile1
             	    {code for statblock}
             	    j gowhile1
        endwhile1   {code continuation}

         */
        self.generator.buffer_label(todo!("while label"));
        todo!("parse rel expr"); // while_statement.children()[0]; store result in tn

        self.generator.add_instruction(Instruction::LoadWordLabel(R1, R0, todo!("tn")));
        self.generator.add_instruction(Instruction::BranchIfZeroLabel(R1, todo!("end while label")));

        // code for stat block
        self.generate_statement_block_code(&while_statement.children()[1], fe, symbols);

        self.generator.add_instruction(Instruction::JumpLabel(todo!("while label")));
        self.generator.buffer_label(todo!("end while label"));

        return;
    }

    fn generate_if_statement_code(&mut self, if_statement: &Node, fe: &FunctionEntry, symbols: &SymbolTable)
    {
        /*
                {code for expr yields tn as a result}
                lw r1,tn(r0)
                bz r1,else1
                {code for statblock1}
                j endif1
        else1  	{code for statblock2}
        endif1 	{code continuation}
         */
        todo!("parse rel expr"); // if_statement.children()[0]; store result in tn
        self.generator.add_instruction(Instruction::LoadWordLabel(R1, R0, todo!("tn")));
        self.generator.add_instruction(Instruction::BranchIfZeroLabel(R1, todo!("else label")));

        // code for then stat block
        self.generate_statement_block_code(&if_statement.children()[1], fe, symbols);
        self.generator.add_instruction(Instruction::JumpLabel(todo!("statement after if/then/else")));

        // code for else stat block
        self.generate_statement_block_code(&if_statement.children()[2], fe, symbols);

        self.generator.buffer_label(todo!("endif label"));

        return;
    }

    fn generate_read_statement_code(&mut self, read_statement: &Node, fe: &FunctionEntry, symbols: &SymbolTable)
    {
        log::error!("READ STATEMENT CODE GENERATION: NOT IMPLEMENTED");
    }

    fn generate_write_statement_code(&mut self, write_statement: &Node, fe: &FunctionEntry, symbols: &SymbolTable)
    {
        log::error!("WRITE STATEMENT CODE GENERATION: NOT IMPLEMENTED");
    }

    fn generate_return_statement_code(&mut self, return_statement: &Node, fe: &FunctionEntry, symbols: &SymbolTable)
    {
        todo!("parse expr"); // return_statement.children()[0]; store result in fn res

        // jump back to callee, assuming address will be stored in R15
        self.generator.add_instruction(Instruction::JumpRegister(R15));
        return;
    }

    fn generate_generic_statement_code(&mut self, generic_statement: &Node, fe: &FunctionEntry, symbols: &SymbolTable)
    {
        // either assignment or function call
        match generic_statement.children()[0].val()
        {
            None => { panic!() }
            Some(val) => {
                match val
                {
                    NodeVal::Leaf(func_ident) => {
                        self.generate_function_call_code(&generic_statement.children()[0], fe, symbols); //todo what about member function call
                    }
                    NodeVal::Internal(InternalNodeType::Assignment) => {
                        self.generate_assignment_code(&generic_statement.children()[0], fe, symbols);
                    },
                    NodeVal::Internal(InternalNodeType::DotOp) => {
                        todo!("probably member function call");
                    },
                    _ => { panic!() }
                }
            }
        }
    }

    fn generate_function_call_code(&mut self, function_call: &Node, function_entry: &FunctionEntry, symbols: &SymbolTable)
    {
        // parse parameters
        // store parameters for function call
        // jump link
        todo!();
    }

    fn generate_assignment_code(&mut self, assignment_statement: &Node, function_entry: &FunctionEntry, symbols: &SymbolTable)
    {
        // parse right expression
        // maybe parse left expression (e.g. array indexing, data member (dot op) )
        // store rhs result register in lhs

        // usually  lw rn, rhs
        //          sw lhs, rn
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
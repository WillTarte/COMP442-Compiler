use crate::codegen::allocator::{LabelAllocator, RegisterAllocator};
use crate::codegen::generator::ExprParseStorage::*;
use crate::codegen::instruction_set::Instruction::{JumpLabel, JumpRegister};
use crate::codegen::instruction_set::Register::*;
use crate::codegen::instruction_set::{Instruction, Register, TaggedInstruction};
use crate::codegen::utils::{
    generate_arith_expr_postfix, is_arith_operand, is_arith_operator, sizeof,
};
use crate::lexer::token::TokenType;
use crate::parser::ast::{InternalNodeType, Node, NodeVal};
use crate::semantics::symbol_table::{
    ClassEntry, FunctionEntry, ParameterEntry, Scope, SymbolTable, Type, VariableEntry,
};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::thread::current;

#[derive(Default)]
pub struct CodeGenOutput(Vec<TaggedInstruction>);

impl CodeGenOutput {
    pub fn append(&mut self, mut other: CodeGenOutput) {
        self.0.append(&mut other.0)
    }
    pub fn push(&mut self, inst: TaggedInstruction) {
        self.0.push(inst);
    }
}

impl Display for CodeGenOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for inst in self.0.iter() {
            write!(f, "{}\n", inst.to_string())?;
        }
        Ok(())
    }
}

#[derive(Default)]
struct CodeGenerator {
    output: CodeGenOutput,
    current_label: Option<String>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            output: CodeGenOutput::default(),
            current_label: None,
        }
    }

    pub fn add_tagged_resource(&mut self, tag: &str, bytes: u32) {
        self.output.0.push(TaggedInstruction(
            Some(tag.to_string()),
            Instruction::Res(bytes),
        ));
    }

    pub fn add_instruction(&mut self, inst: Instruction) {
        if self.current_label.is_some() {
            let label = std::mem::replace(&mut self.current_label, None);
            self.output.0.push(TaggedInstruction(label, inst));
        } else {
            self.output.0.push(TaggedInstruction(None, inst));
        }
    }

    pub fn add_tagged_instruction(&mut self, t_inst: TaggedInstruction) {
        if self.current_label.is_some() {
            log::warn!(
                "CURRENT LABEL {:?}; OTHER {:?}",
                self.current_label,
                t_inst.0
            );
            let old_label = std::mem::replace(&mut self.current_label, None);
            self.output
                .push(TaggedInstruction(old_label, Instruction::NoOp));
        }
        self.output.0.push(t_inst);
    }

    pub fn buffer_label(&mut self, label: &str) {
        if self.current_label.is_some() {
            log::warn!("CURRENT LABEL {:?}; OTHER {}", self.current_label, label);
            let old_label = std::mem::replace(&mut self.current_label, None);
            self.output
                .push(TaggedInstruction(old_label, Instruction::NoOp));
        }
        self.current_label = Some(label.to_string());
    }
}

pub struct MoonGenerator {
    label_allocator: LabelAllocator,
    register_allocator: RegisterAllocator,
    generator: CodeGenerator,
    fn_pointer_offset: i16,
}

impl MoonGenerator {
    pub fn new() -> Self {
        Self {
            label_allocator: Default::default(),
            register_allocator: RegisterAllocator::new(),
            generator: Default::default(),
            fn_pointer_offset: 0,
        }
    }

    pub fn generate(&mut self, ast: &Node, symbols: &SymbolTable) {
        //todo!("setup code");

        //visit root children
        self.visit_class_declarations(&ast.children()[0], symbols);
        self.visit_function_definitions(&ast.children()[1], symbols);
        self.visit_main_function(&ast.children()[2], symbols);

        //todo!("fn ptr stack setup")
    }

    pub fn finish(self) -> CodeGenOutput {
        self.generator.output
    }

    fn visit_class_declarations(&mut self, class_declaration: &Node, symbols: &SymbolTable) {
        log::error!("CODE GENERATION FOR CLASSES: NOT IMPLEMENTED");
    }

    fn visit_function_definitions(&mut self, function_definitions: &Node, symbols: &SymbolTable) {
        log::info!("CODE GENERATION FOR FUNCTIONS: WIP");
        //visit each function
        for function_def in function_definitions.children() {
            if let Some(NodeVal::Leaf(token)) = function_def.children()[0].val() {
                if let Some(Scope::Function(fe)) = symbols.find_scope_by_ident(token.lexeme()) {
                    self.generate_function_code(fe, function_def, symbols);
                }
            }
        }

        return;
    }

    fn visit_main_function(&mut self, main_func: &Node, symbols: &SymbolTable) {
        log::error!("CODE GENERATION FOR MAIN: WIP");

        if let Some(Scope::Function(main)) = symbols.find_scope_by_ident("main") {
            for scope in main.table().scopes() {
                if let Scope::Variable(ve) = scope {
                    let tag: String = format!("var_{}_{}", main.ident(), ve.ident());
                    self.generator
                        .add_tagged_resource(&tag, sizeof(ve.var_type(), symbols));
                }
            }

            // Generate code for main function body
            self.generator.add_tagged_instruction(TaggedInstruction(
                Some("fn_main".to_string()),
                Instruction::Entry,
            ));
            self.generate_statement_block_code(
                &main_func.children()[0].children()[1],
                main,
                symbols,
            );
        }

        return;
    }

    fn generate_function_code(
        &mut self,
        fe: &FunctionEntry,
        function_definition: &Node,
        symbols: &SymbolTable,
    ) {
        // reserve memory for function return value
        self.generator.add_tagged_resource(
            &format!("fnres_{}", fe.ident()),
            sizeof(&fe.type_sig().1, symbols),
        );

        // Allocate resources for the function parameters and local variables
        for scope in fe.table().scopes().iter() {
            if let Scope::FunctionParameter(fpe) = scope {
                let tag: String = format!("param_{}_{}", fe.ident(), fpe.ident());
                self.generator
                    .add_tagged_resource(&tag, sizeof(fpe.param_type(), symbols));
            } else if let Scope::Variable(ve) = scope {
                let tag: String = format!("var_{}_{}", fe.ident(), ve.ident());
                self.generator
                    .add_tagged_resource(&tag, sizeof(ve.var_type(), symbols));
            }
        }

        // Add function body
        self.generator.add_tagged_instruction(TaggedInstruction(
            Some(format!("fn_{}", fe.ident())),
            Instruction::NoOp,
        ));
        self.store_and_inc_fn_ptr(); // store the callee's next instruction's address on the fn_ptr stack
        self.generate_statement_block_code(
            &function_definition.children()[4].children()[1],
            fe,
            symbols,
        ); //Statement list

        // reset local variables
        for scope in fe.table().scopes().iter() {
            if let Scope::Variable(ve) = scope {
                self.generator.add_instruction(Instruction::StoreWordLabel(
                    R0,
                    R0,
                    format!("var_{}_{}", fe.ident(), ve.ident()),
                ));
            }
        }

        // jump back to callee, assuming address will be stored in R15
        self.load_and_dec_fn_ptr();
        self.generator
            .add_instruction(Instruction::JumpRegister(R15));

        return;
    }

    fn generate_statement_block_code(
        &mut self,
        statement_block: &Node,
        fe: &FunctionEntry,
        symbols: &SymbolTable,
    ) {
        for statement in statement_block.children() {
            match statement.val() {
                None => {
                    self.generator.add_instruction(Instruction::NoOp);
                    break;
                }
                Some(statement_type) => {
                    match statement_type {
                        NodeVal::Leaf(_) => {
                            panic!()
                        }
                        NodeVal::Internal(ty) => {
                            match ty {
                                InternalNodeType::WhileStatement => {
                                    self.generate_while_statement_code(statement, fe, symbols);
                                }
                                InternalNodeType::IfStatement => {
                                    self.generate_if_statement_code(statement, fe, symbols);
                                }
                                InternalNodeType::ReadStatement => {
                                    self.generate_read_statement_code(statement, fe, symbols);
                                }
                                InternalNodeType::WriteStatement => {
                                    self.generate_write_statement_code(statement, fe, symbols);
                                }
                                InternalNodeType::ReturnStatement => {
                                    self.generate_return_statement_code(statement, fe, symbols);
                                }
                                InternalNodeType::BreakStatement => { /*panic!()*/ }
                                InternalNodeType::ContinueStatement => { /*panic!()*/ }
                                InternalNodeType::GenericStatement => {
                                    self.generate_generic_statement_code(statement, fe, symbols);
                                }
                                _ => panic!(),
                            }
                        }
                    }
                }
            }
        }

        return;
    }

    fn generate_while_statement_code(
        &mut self,
        while_statement: &Node,
        fe: &FunctionEntry,
        symbols: &SymbolTable,
    ) {
        /*
        gowhile1	{code for a<b yields tn as a result}
                    lw r1,tn(r0)
                     bz r1,endwhile1
                     {code for statblock}
                     j gowhile1
        endwhile1   {code continuation}

         */
        let (while_label, endwhile_label) = self.label_allocator.next_while_labels();

        self.generator.buffer_label(&while_label);
        let rel_expr_res =
            self.generate_relative_expression_code(&while_statement.children()[0], fe, symbols);
        let rel_expr_reg = self.register_allocator.next_free_register();
        match rel_expr_res {
            Immediate(val) => {
                self.generator.add_instruction(Instruction::Substract(
                    rel_expr_reg,
                    rel_expr_reg,
                    rel_expr_reg,
                ));
                self.generator
                    .add_instruction(Instruction::AddImmediate(rel_expr_reg, R0, val));
            }
            Labelled(label, offset) => {
                self.generator.add_instruction(Instruction::Substract(
                    rel_expr_reg,
                    rel_expr_reg,
                    rel_expr_reg,
                ));
                let offset_reg = self.register_allocator.next_free_register();
                self.generator
                    .add_instruction(Instruction::Substract(offset_reg, offset_reg, offset_reg));
                self.generator.add_instruction(Instruction::AddImmediate(
                    offset_reg,
                    R0,
                    offset.to_string(),
                ));
                self.generator.add_instruction(Instruction::LoadWordLabel(
                    rel_expr_reg,
                    offset_reg,
                    label.to_string(),
                ));
                self.register_allocator.release_register(offset_reg);
            }
            Register(r) => {
                self.generator.add_instruction(Instruction::Substract(
                    rel_expr_reg,
                    rel_expr_reg,
                    rel_expr_reg,
                ));
                self.generator
                    .add_instruction(Instruction::Add(rel_expr_reg, R0, r));
            }
        }
        self.generator
            .add_instruction(Instruction::BranchIfZeroLabel(
                rel_expr_reg,
                endwhile_label.clone(),
            ));
        self.register_allocator.release_register(rel_expr_reg);

        // code for stat block
        self.generate_statement_block_code(&while_statement.children()[1], fe, symbols);

        self.generator
            .add_instruction(Instruction::JumpLabel(while_label.clone()));
        self.generator.buffer_label(&endwhile_label);

        return;
    }

    fn generate_if_statement_code(
        &mut self,
        if_statement: &Node,
        fe: &FunctionEntry,
        symbols: &SymbolTable,
    ) {
        /*
                {code for expr yields tn as a result}
                lw r1,tn(r0)
                bz r1,else1
                {code for statblock1}
                j endif1
        else1  	{code for statblock2}
        endif1 	{code continuation}
         */
        let (if_label, else_label, endif_label) = self.label_allocator.next_if_labels();

        let rel_expr_res =
            self.generate_relative_expression_code(&if_statement.children()[0], fe, symbols);
        let rel_expr_reg = self.register_allocator.next_free_register();
        match rel_expr_res {
            Immediate(val) => {
                self.generator.add_instruction(Instruction::Substract(
                    rel_expr_reg,
                    rel_expr_reg,
                    rel_expr_reg,
                ));
                self.generator
                    .add_instruction(Instruction::AddImmediate(rel_expr_reg, R0, val));
            }
            Labelled(label, offset) => {
                self.generator.add_instruction(Instruction::Substract(
                    rel_expr_reg,
                    rel_expr_reg,
                    rel_expr_reg,
                ));
                let offset_reg = self.register_allocator.next_free_register();
                self.generator
                    .add_instruction(Instruction::Substract(offset_reg, offset_reg, offset_reg));
                self.generator.add_instruction(Instruction::AddImmediate(
                    offset_reg,
                    R0,
                    offset.to_string(),
                ));
                self.generator.add_instruction(Instruction::LoadWordLabel(
                    rel_expr_reg,
                    offset_reg,
                    label.to_string(),
                ));
                self.register_allocator.release_register(offset_reg);
            }
            Register(r) => {
                self.generator.add_instruction(Instruction::Substract(
                    rel_expr_reg,
                    rel_expr_reg,
                    rel_expr_reg,
                ));
                self.generator
                    .add_instruction(Instruction::Add(rel_expr_reg, R0, r));
            }
        }
        self.generator.add_tagged_instruction(TaggedInstruction(
            Some(if_label),
            Instruction::BranchIfZeroLabel(rel_expr_reg, else_label.clone()),
        ));
        self.register_allocator.release_register(rel_expr_reg);

        // code for then stat block
        self.generate_statement_block_code(&if_statement.children()[1], fe, symbols);
        self.generator
            .add_instruction(Instruction::JumpLabel(endif_label.clone()));

        // code for else stat block
        self.generator.buffer_label(&else_label.clone());
        self.generate_statement_block_code(&if_statement.children()[2], fe, symbols);

        self.generator.buffer_label(&endif_label);

        return;
    }

    fn generate_read_statement_code(
        &mut self,
        read_statement: &Node,
        fe: &FunctionEntry,
        symbols: &SymbolTable,
    ) {
        log::error!("READ STATEMENT CODE GENERATION: NOT IMPLEMENTED");
    }

    fn generate_write_statement_code(
        &mut self,
        write_statement: &Node,
        fe: &FunctionEntry,
        symbols: &SymbolTable,
    ) {
        log::error!("WRITE STATEMENT CODE GENERATION: NOT IMPLEMENTED");
    }

    fn generate_return_statement_code(
        &mut self,
        return_statement: &Node,
        fe: &FunctionEntry,
        symbols: &SymbolTable,
    ) {
        let expr_result =
            self.generate_expression_code(&return_statement.children()[0], fe, symbols);
        let expr_result_reg = self.register_allocator.next_free_register();
        match expr_result {
            Immediate(val) => {
                self.generator.add_instruction(Instruction::Substract(
                    expr_result_reg,
                    expr_result_reg,
                    expr_result_reg,
                ));
                self.generator
                    .add_instruction(Instruction::AddImmediate(expr_result_reg, R0, val));
            }
            Labelled(label, offset) => {
                self.generator.add_instruction(Instruction::Substract(
                    expr_result_reg,
                    expr_result_reg,
                    expr_result_reg,
                ));
                let offset_reg = self.register_allocator.next_free_register();
                self.generator
                    .add_instruction(Instruction::Substract(offset_reg, offset_reg, offset_reg));
                self.generator.add_instruction(Instruction::AddImmediate(
                    offset_reg,
                    R0,
                    offset.to_string(),
                ));
                self.generator.add_instruction(Instruction::LoadWordLabel(
                    expr_result_reg,
                    offset_reg,
                    label.to_string(),
                ));
                self.register_allocator.release_register(offset_reg);
            }
            Register(r) => {
                self.generator.add_instruction(Instruction::Substract(
                    expr_result_reg,
                    expr_result_reg,
                    expr_result_reg,
                ));
                self.generator
                    .add_instruction(Instruction::Add(expr_result_reg, R0, r));
            }
        }
        self.generator.add_instruction(Instruction::StoreWordLabel(
            expr_result_reg,
            R0,
            format!("fnres_{}", fe.ident()),
        ));
        self.register_allocator.release_register(expr_result_reg);

        // jump back to callee, assuming address will be stored in R15
        self.load_and_dec_fn_ptr();
        self.generator
            .add_instruction(Instruction::JumpRegister(R15));

        return;
    }

    fn generate_generic_statement_code(
        &mut self,
        generic_statement: &Node,
        fe: &FunctionEntry,
        symbols: &SymbolTable,
    ) {
        // either assignment or function call
        match generic_statement.children()[0].val() {
            None => {
                panic!()
            }
            Some(val) => {
                match val {
                    NodeVal::Leaf(func_ident) => {
                        self.generate_function_call_code(
                            &generic_statement.children()[0],
                            fe,
                            symbols,
                        );
                    }
                    NodeVal::Internal(InternalNodeType::Assignment) => {
                        self.generate_assignment_code(
                            &generic_statement.children()[0],
                            fe,
                            symbols,
                        );
                    }
                    NodeVal::Internal(InternalNodeType::DotOp) => {
                        let _ = self.generate_dot_operator_code(
                            &generic_statement.children()[0],
                            fe,
                            symbols,
                        ); // todo do we care about the return value here?
                    }
                    _ => {
                        panic!()
                    }
                }
            }
        }

        return;
    }

    fn generate_function_call_code(
        &mut self,
        function_call: &Node,
        function_entry: &FunctionEntry,
        symbols: &SymbolTable,
    ) {
        // parse parameters
        // store parameters for function call
        // jump link
        let function_ident = match function_call.val() //todo
        {
            Some(NodeVal::Leaf(token)) => { token.lexeme() }
            _ => { panic!() }
        };

        let mut expr_results: Vec<ExprParseStorage> = Vec::new();
        for expr_node in function_call.children()[0].children() {
            expr_results.push(self.generate_expression_code(expr_node, function_entry, symbols));
        }

        todo!("store expr results in parameters");
        self.generator.add_instruction(Instruction::JumpLinkLabel(
            R15,
            format!("fn_{}", function_ident),
        )); // jump to function

        return;
    }

    fn generate_assignment_code(
        &mut self,
        assignment_statement: &Node,
        function_entry: &FunctionEntry,
        symbols: &SymbolTable,
    ) {
        // parse right expression
        // maybe parse left expression (e.g. array indexing, data member (dot op) )
        // store rhs result register in lhs

        // usually  lw rn, rhs
        //          sw lhs, rn
        //

        let lhs: ExprParseStorage = match assignment_statement.children()[0].val() {
            Some(NodeVal::Internal(InternalNodeType::DotOp)) => self.generate_dot_operator_code(
                &assignment_statement.children()[0],
                function_entry,
                symbols,
            ),
            Some(NodeVal::Leaf(token)) => {
                match function_entry.table().find_scope_by_ident(token.lexeme()) {
                    Some(Scope::Variable(ve)) => match ve.var_type() {
                        Type::Integer | Type::Float => ExprParseStorage::Labelled(
                            format!("var_{}_{}", function_entry.ident(), ve.ident()),
                            0,
                        ),
                        Type::String => {
                            todo!("strings not implemented")
                        }
                        Type::Custom(_) => {
                            todo!("follow ptr")
                        }
                        Type::CustomArray(_, _)
                        | Type::IntegerArray(_)
                        | Type::FloatArray(_)
                        | Type::StringArray(_) => {
                            todo!("indices, follow ptr")
                        }
                        _ => panic!(),
                    },
                    Some(Scope::FunctionParameter(pe)) => match pe.param_type() {
                        Type::Integer | Type::Float => ExprParseStorage::Labelled(
                            format!("var_{}_{}", function_entry.ident(), pe.ident()),
                            0,
                        ),
                        Type::String => {
                            todo!("strings not implemented")
                        }
                        Type::Custom(_) => {
                            todo!("follow ptr")
                        }
                        Type::CustomArray(_, _)
                        | Type::IntegerArray(_)
                        | Type::FloatArray(_)
                        | Type::StringArray(_) => {
                            todo!("indices, follow ptr")
                        }
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            }
            _ => {
                panic!()
            }
        };

        let rhs: ExprParseStorage = self.generate_expression_code(
            &assignment_statement.children()[1],
            function_entry,
            symbols,
        );

        match (lhs, rhs) {
            (Labelled(llabel, loffs), Labelled(rlabel, roffs)) => {
                if loffs != 0 {
                    let lreg = self.register_allocator.next_free_register();
                    self.generator
                        .add_instruction(Instruction::Substract(lreg, lreg, lreg)); // zero out
                    let rreg = self.register_allocator.next_free_register();
                    self.generator
                        .add_instruction(Instruction::Substract(rreg, rreg, rreg)); // zero out

                    self.generator.add_instruction(Instruction::AddImmediate(
                        rreg,
                        R0,
                        roffs.to_string(),
                    )); // offset
                    self.generator.add_instruction(Instruction::LoadWordLabel(
                        rreg,
                        rreg,
                        rlabel.to_string(),
                    )); // actual word

                    self.generator.add_instruction(Instruction::AddImmediate(
                        lreg,
                        R0,
                        loffs.to_string(),
                    )); // offset
                    self.generator.add_instruction(Instruction::StoreWordLabel(
                        rreg,
                        lreg,
                        llabel.to_string(),
                    )); // actual word

                    self.register_allocator.release_register(lreg);
                    self.register_allocator.release_register(rreg);
                } else {
                    let rreg = self.register_allocator.next_free_register();
                    self.generator
                        .add_instruction(Instruction::Substract(rreg, rreg, rreg)); // zero out

                    self.generator.add_instruction(Instruction::AddImmediate(
                        rreg,
                        R0,
                        roffs.to_string(),
                    )); // offset
                    self.generator.add_instruction(Instruction::LoadWordLabel(
                        rreg,
                        rreg,
                        rlabel.to_string(),
                    )); // actual word

                    self.generator.add_instruction(Instruction::StoreWordLabel(
                        rreg,
                        R0,
                        llabel.to_string(),
                    )); // actual word

                    self.register_allocator.release_register(rreg);
                }
            }
            (Labelled(lhs_label, loffs), Immediate(imm)) => {
                if loffs != 0 {
                    let lhs_reg = self.register_allocator.next_free_register();
                    self.generator
                        .add_instruction(Instruction::Substract(lhs_reg, lhs_reg, lhs_reg)); // zero out
                    let rhs_reg = self.register_allocator.next_free_register();
                    self.generator
                        .add_instruction(Instruction::Substract(rhs_reg, rhs_reg, rhs_reg)); // zero out

                    self.generator
                        .add_instruction(Instruction::AddImmediate(rhs_reg, R0, imm));
                    self.generator.add_instruction(Instruction::AddImmediate(
                        lhs_reg,
                        R0,
                        loffs.to_string(),
                    ));
                    self.generator.add_instruction(Instruction::StoreWordLabel(
                        rhs_reg,
                        lhs_reg,
                        lhs_label.to_string(),
                    ));
                    self.register_allocator.release_register(lhs_reg);
                    self.register_allocator.release_register(rhs_reg);
                } else {
                    let rhs_reg = self.register_allocator.next_free_register();
                    self.generator
                        .add_instruction(Instruction::Substract(rhs_reg, rhs_reg, rhs_reg)); // zero out

                    self.generator
                        .add_instruction(Instruction::AddImmediate(rhs_reg, R0, imm));
                    self.generator.add_instruction(Instruction::StoreWordLabel(
                        rhs_reg,
                        R0,
                        lhs_label.to_string(),
                    ));
                    self.register_allocator.release_register(rhs_reg);
                }
            }
            (Labelled(lhs_label, lhs_offs), Register(r)) => {
                if lhs_offs != 0 {
                    let lhs_reg = self.register_allocator.next_free_register();
                    self.generator
                        .add_instruction(Instruction::Substract(lhs_reg, lhs_reg, lhs_reg)); // zero out
                    self.generator.add_instruction(Instruction::AddImmediate(
                        lhs_reg,
                        R0,
                        lhs_offs.to_string(),
                    ));
                    self.generator.add_instruction(Instruction::StoreWordLabel(
                        r,
                        lhs_reg,
                        lhs_label.to_string(),
                    ));
                    self.register_allocator.release_register(lhs_reg);
                } else {
                    self.generator.add_instruction(Instruction::StoreWordLabel(
                        r,
                        R0,
                        lhs_label.to_string(),
                    ));
                }
            }
            _ => panic!(),
        }

        return;
    }

    /// Given an expression nodes, generates code for the it.
    /// Return value is where/what of the return value of the expression.
    fn generate_expression_code(
        &mut self,
        expr: &Node,
        function_entry: &FunctionEntry,
        symbols: &SymbolTable,
    ) -> ExprParseStorage {
        match expr.children()[0].val() {
            Some(NodeVal::Internal(InternalNodeType::ArithExpr)) => {
                return self.generate_arith_expr_code(&expr.children()[0], function_entry, symbols);
            }
            Some(NodeVal::Internal(InternalNodeType::Equal))
            | Some(NodeVal::Internal(InternalNodeType::NotEqual))
            | Some(NodeVal::Internal(InternalNodeType::LessThan))
            | Some(NodeVal::Internal(InternalNodeType::LessEqualThan))
            | Some(NodeVal::Internal(InternalNodeType::GreaterThan))
            | Some(NodeVal::Internal(InternalNodeType::GreaterEqualThan)) => {
                return self.generate_relative_expression_code(
                    &expr.children()[0],
                    function_entry,
                    symbols,
                );
            }
            _ => panic!(),
        }
    }

    /// Given a relative expression node, generates code for it.
    /// Return value is where/what of the return value of the expression.
    fn generate_relative_expression_code(
        &mut self,
        rel_expr: &Node,
        function_entry: &FunctionEntry,
        symbols: &SymbolTable,
    ) -> ExprParseStorage {
        let left = self.generate_arith_expr_code(
            &rel_expr.children()[0].children()[0],
            function_entry,
            symbols,
        );
        let right = self.generate_arith_expr_code(
            &rel_expr.children()[0].children()[1],
            function_entry,
            symbols,
        );
        let temp_storage_label = self.label_allocator.next_temp_label();

        // reserve for temporary
        self.generator.add_tagged_instruction(TaggedInstruction(
            Some(temp_storage_label.clone()),
            Instruction::Res(sizeof(&Type::Integer, symbols)),
        ));

        let mut lr = self.register_allocator.next_free_register();
        self.generator
            .add_instruction(Instruction::Substract(lr, lr, lr));
        let mut rr = self.register_allocator.next_free_register();
        self.generator
            .add_instruction(Instruction::Substract(rr, rr, rr));

        match (left, right) {
            (Immediate(lval), Immediate(rval)) => {
                self.generator
                    .add_instruction(Instruction::AddImmediate(lr, R0, lval));
                self.generator
                    .add_instruction(Instruction::AddImmediate(rr, R0, rval));
            }
            (Immediate(lval), Labelled(rlabel, roffs)) => {
                self.generator
                    .add_instruction(Instruction::AddImmediate(lr, R0, lval));

                let roffs_reg = self.register_allocator.next_free_register();
                self.generator
                    .add_instruction(Instruction::Substract(roffs_reg, roffs_reg, roffs_reg));
                self.generator.add_instruction(Instruction::AddImmediate(
                    roffs_reg,
                    R0,
                    roffs.to_string(),
                ));
                self.generator
                    .add_instruction(Instruction::LoadWordLabel(rr, roffs_reg, rlabel));
                self.register_allocator.release_register(roffs_reg);
            }
            (Immediate(lval), Register(_rr)) => {
                self.generator
                    .add_instruction(Instruction::AddImmediate(lr, R0, lval));
                self.register_allocator.release_register(rr);
                rr = _rr;
            }
            (Labelled(llabel, loffs), Immediate(rval)) => {
                let offs_reg = self.register_allocator.next_free_register();
                self.generator
                    .add_instruction(Instruction::Substract(offs_reg, offs_reg, offs_reg));
                self.generator.add_instruction(Instruction::AddImmediate(
                    offs_reg,
                    R0,
                    loffs.to_string(),
                ));
                self.generator
                    .add_instruction(Instruction::LoadWordLabel(lr, offs_reg, llabel));
                self.generator
                    .add_instruction(Instruction::AddImmediate(rr, R0, rval));
                self.register_allocator.release_register(offs_reg);
            }
            (Labelled(llabel, loffs), Labelled(rlabel, roffs)) => {
                let loffs_reg = self.register_allocator.next_free_register();
                self.generator
                    .add_instruction(Instruction::Substract(loffs_reg, loffs_reg, loffs_reg));
                self.generator.add_instruction(Instruction::AddImmediate(
                    loffs_reg,
                    R0,
                    loffs.to_string(),
                ));
                self.generator
                    .add_instruction(Instruction::LoadWordLabel(lr, loffs_reg, llabel));

                let roffs_reg = self.register_allocator.next_free_register();
                self.generator
                    .add_instruction(Instruction::Substract(roffs_reg, roffs_reg, roffs_reg));
                self.generator.add_instruction(Instruction::AddImmediate(
                    roffs_reg,
                    R0,
                    roffs.to_string(),
                ));
                self.generator
                    .add_instruction(Instruction::LoadWordLabel(rr, roffs_reg, rlabel));

                self.register_allocator.release_register(loffs_reg);
                self.register_allocator.release_register(roffs_reg);
            }
            (Labelled(llabel, loffs), Register(_rr)) => {
                let loffs_reg = self.register_allocator.next_free_register();
                self.generator
                    .add_instruction(Instruction::Substract(loffs_reg, loffs_reg, loffs_reg));
                self.generator.add_instruction(Instruction::AddImmediate(
                    loffs_reg,
                    R0,
                    loffs.to_string(),
                ));
                self.generator
                    .add_instruction(Instruction::LoadWordLabel(lr, loffs_reg, llabel));
                self.register_allocator.release_register(loffs_reg);
                self.register_allocator.release_register(rr);
                rr = _rr;
            }
            (Register(_lr), Immediate(rval)) => {
                self.register_allocator.release_register(lr);
                lr = _lr;

                self.generator
                    .add_instruction(Instruction::AddImmediate(rr, R0, rval));
            }
            (Register(_lr), Labelled(rlabel, roffs)) => {
                self.register_allocator.release_register(lr);
                lr = _lr;

                let roffs_reg = self.register_allocator.next_free_register();
                self.generator
                    .add_instruction(Instruction::Substract(roffs_reg, roffs_reg, roffs_reg));
                self.generator.add_instruction(Instruction::AddImmediate(
                    roffs_reg,
                    R0,
                    roffs.to_string(),
                ));
                self.generator
                    .add_instruction(Instruction::LoadWordLabel(rr, roffs_reg, rlabel));
                self.register_allocator.release_register(roffs_reg);
            }
            (Register(_lr), Register(_rr)) => {
                self.register_allocator.release_register(lr);
                lr = _lr;

                self.register_allocator.release_register(rr);
                rr = _rr;
            }
        }

        match rel_expr.children()[0].val() {
            Some(NodeVal::Internal(InternalNodeType::Equal)) => {
                self.generator
                    .add_instruction(Instruction::Equal(lr, lr, rr));
                self.register_allocator.release_register(rr);
                return ExprParseStorage::Register(lr);
            }
            Some(NodeVal::Internal(InternalNodeType::NotEqual)) => {
                self.generator
                    .add_instruction(Instruction::NotEqual(lr, lr, rr));
                self.register_allocator.release_register(rr);
                return ExprParseStorage::Register(lr);
            }
            Some(NodeVal::Internal(InternalNodeType::LessThan)) => {
                self.generator
                    .add_instruction(Instruction::Less(lr, lr, rr));
                self.register_allocator.release_register(rr);
                return ExprParseStorage::Register(lr);
            }
            Some(NodeVal::Internal(InternalNodeType::LessEqualThan)) => {
                self.generator
                    .add_instruction(Instruction::LessEqual(lr, lr, rr));
                self.register_allocator.release_register(rr);
                return ExprParseStorage::Register(lr);
            }
            Some(NodeVal::Internal(InternalNodeType::GreaterThan)) => {
                self.generator
                    .add_instruction(Instruction::Greater(lr, lr, rr));
                self.register_allocator.release_register(rr);
                return ExprParseStorage::Register(lr);
            }
            Some(NodeVal::Internal(InternalNodeType::GreaterEqualThan)) => {
                self.generator
                    .add_instruction(Instruction::GreaterEqual(lr, lr, rr));
                self.register_allocator.release_register(rr);
                return ExprParseStorage::Register(lr);
            }
            _ => panic!(),
        }
    }

    /// Given a dot operator node, generates code for it.
    /// Return value is in where/what of the return value of the expression.
    fn generate_dot_operator_code(
        &mut self,
        dot_op: &Node,
        function_entry: &FunctionEntry,
        symbols: &SymbolTable,
    ) -> ExprParseStorage {
        todo!()
    }

    fn generate_arith_expr_code(
        &mut self,
        arith_expr: &Node,
        function_entry: &FunctionEntry,
        symbols: &SymbolTable,
    ) -> ExprParseStorage {
        let mut postfix: Vec<&Node> = Vec::new();
        generate_arith_expr_postfix(arith_expr, &mut postfix);
        let mut eval_stack: Vec<ExprParseStorage> = Vec::new();

        for postfix_token in postfix {
            if is_arith_operator(postfix_token) {
                let op_2 = eval_stack.pop().unwrap();
                let op_1 = eval_stack.pop().unwrap();
                todo!("generate code for loading op_2 and op_1, evaluate with op, push on stack");
            } else if is_arith_operand(postfix_token) {
                match postfix_token.val() {
                    Some(NodeVal::Internal(InternalNodeType::SignedFactor)) => {
                        todo!()
                    }
                    Some(NodeVal::Internal(InternalNodeType::DotOp)) => {
                        eval_stack.push(self.generate_dot_operator_code(
                            postfix_token,
                            function_entry,
                            symbols,
                        ));
                    }
                    Some(NodeVal::Leaf(token)) => {
                        //todo we have to check if it's indexing into the variable, or it's a free function call
                        match token.token_type() {
                            TokenType::Id => {
                                match function_entry.table().find_scope_by_ident(token.lexeme()) {
                                    Some(Scope::Variable(ve)) => {
                                        eval_stack.push(ExprParseStorage::Labelled(
                                            format!(
                                                "var_{}_{}",
                                                function_entry.ident(),
                                                ve.ident()
                                            ),
                                            0,
                                        ));
                                    }
                                    Some(Scope::FunctionParameter(pe)) => {
                                        eval_stack.push(ExprParseStorage::Labelled(
                                            format!(
                                                "param_{}_{}",
                                                function_entry.ident(),
                                                pe.ident()
                                            ),
                                            0,
                                        ));
                                    }
                                    _ => panic!(),
                                }
                            }
                            TokenType::IntegerLit => {
                                eval_stack
                                    .push(ExprParseStorage::Immediate(token.lexeme().to_string()));
                            }
                            TokenType::FloatLit => {
                                log::warn!("FLOATING POINTS WILL NOT WORK PROPERLY");
                                eval_stack
                                    .push(ExprParseStorage::Immediate(token.lexeme().to_string()));
                            }
                            TokenType::StringLit => {
                                todo!("string manipulation is not supported")
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
        }

        return eval_stack.pop().unwrap();
    }

    /// When jumping to a function, we store the return address from R15 into the next slot of the fn ptr stack
    fn store_and_inc_fn_ptr(&mut self) {
        // return instruction address is in R15
        self.fn_pointer_offset += 4; //todo bytes?
                                     // load fn_ptr offset into available register
        let fn_ptr_offset_reg = self.register_allocator.next_free_register();
        self.generator.add_instruction(Instruction::AddImmediate(
            fn_ptr_offset_reg,
            R0,
            self.fn_pointer_offset.to_string(),
        ));
        // store address in fn stack
        self.generator.add_instruction(Instruction::StoreWordLabel(
            R15,
            fn_ptr_offset_reg,
            "fn_ptr_stack".to_string(),
        ));
        self.register_allocator.release_register(fn_ptr_offset_reg);
    }

    /// When returning from a function, we pop & load the return address of the top of the fn ptr stack into R15 and jump
    fn load_and_dec_fn_ptr(&mut self) {
        // load fn_ptr offset
        let fn_ptr_offset_reg = self.register_allocator.next_free_register();
        self.generator.add_instruction(Instruction::AddImmediate(
            fn_ptr_offset_reg,
            R0,
            self.fn_pointer_offset.to_string(),
        ));
        // load return address into R15
        self.generator.add_instruction(Instruction::LoadWordLabel(
            R15,
            fn_ptr_offset_reg,
            "fn_ptr_stack".to_string(),
        ));
        // zero out memory location
        self.generator.add_instruction(Instruction::StoreWordLabel(
            R0,
            fn_ptr_offset_reg,
            "fn_ptr_stack".to_string(),
        ));
        self.fn_pointer_offset -= 4; //todo bytes?
        self.register_allocator.release_register(fn_ptr_offset_reg);
    }
}

pub enum ExprParseStorage {
    /// An immediate value, like an integer or a float
    Immediate(String),
    /// A label, usually for local resources, and an offset
    Labelled(String, i16),
    /// A register
    Register(Register),
}

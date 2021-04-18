use crate::codegen::instruction_set::Register;
use crate::codegen::instruction_set::Register::*;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub(crate) struct LabelAllocator {
    while_statement_count: u32,
    if_statement_count: u32,
    temp_resource_count: u32,
}

impl LabelAllocator {
    pub fn current_while_labels(&self) -> (String, String) {
        (
            format!("while_{}", self.while_statement_count),
            format!("endwhile_{}", self.while_statement_count),
        )
    }

    pub fn next_while_labels(&mut self) -> (String, String) {
        self.while_statement_count += 1;
        self.current_while_labels()
    }

    pub fn current_temp_label(&self) -> String {
        format!("t{}", self.temp_resource_count)
    }

    pub fn next_temp_label(&mut self) -> String {
        self.temp_resource_count += 1;
        self.current_temp_label()
    }

    fn current_if_labels(&self) -> (String, String, String) {
        (
            format!("if_{}", self.if_statement_count),
            format!("else_{}", self.if_statement_count),
            format!("endif_{}", self.if_statement_count),
        )
    }

    pub fn next_if_labels(&mut self) -> (String, String, String) {
        self.if_statement_count += 1;
        self.current_if_labels()
    }
}

#[derive(Debug)]
pub(crate) struct RegisterAllocator(HashMap<Register, bool>);

impl RegisterAllocator {
    pub fn new() -> Self {
        let mut res: HashMap<Register, bool> = HashMap::new();
        res.insert(R1, true);
        res.insert(R2, true);
        res.insert(R3, true);
        res.insert(R4, true);
        res.insert(R5, true);
        res.insert(R6, true);
        res.insert(R7, true);
        res.insert(R8, true);
        res.insert(R9, true);
        res.insert(R10, true);
        res.insert(R11, true);
        res.insert(R12, true);
        res.insert(R13, true);

        Self(res)
    }

    pub fn next_free_register(&mut self) -> Register {
        for (reg, available) in self.0.iter_mut() {
            if *available {
                *available = false;
                return reg.clone();
            }
        }
        log::error!("NO REGISTER AVAILABLE");
        R1
    }

    pub fn release_register(&mut self, reg: Register) {
        match reg {
            R1 | R2 | R3 | R4 | R5 | R6 | R7 | R8 | R9 | R10 | R11 | R12 | R13 => {
                let entry = self.0.entry(reg);
                *entry.or_insert(true) = true;
            }
            R0 => {}
            R14 | R15 => {
                panic!("tried to release R14 / R15")
            }
        }
    }
}

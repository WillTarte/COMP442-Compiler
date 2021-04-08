#![allow(dead_code)]

#[derive(Debug)]
pub enum Instruction
{
    LoadWord(Register, Register, i16),
    LoadByte(Register, Register, i16),
    StoreWord(Register, Register, i16),
    StoreByte(Register, Register, i16),
    Add(Register, Register, Register),
    Substract(Register, Register, Register),
    Multiply(Register, Register, Register),
    Divide(Register, Register, Register),
    Modulus(Register, Register, Register),
    And(Register, Register, Register),
    Or(Register, Register, Register),
    Not(Register, Register),
    Equal(Register, Register, Register),
    NotEqual(Register, Register, Register),
    Less(Register, Register, Register),
    LessEqual(Register, Register, Register),
    Greater(Register, Register, Register),
    GreaterEqual(Register, Register, Register),
    AddImmediate(Register, Register, i16),
    SubstractImmediate(Register, Register, i16),
    MultiplyImmediate(Register, Register, i16),
    DivideImmediate(Register, Register, i16),
    ModulusImmediate(Register, Register, i16),
    AndImmediate(Register, Register, i16),
    OrImmediate(Register, Register, i16),
    EqualImmediate(Register, Register, i16),
    NotEqualImmediate(Register, Register, i16),
    LessImmediate(Register, Register, i16),
    LessEqualImmediate(Register, Register, i16),
    GreaterImmediate(Register, Register, i16),
    GreaterEqualImmediate(Register, Register, i16),
    ShiftLeft(Register, i16),
    ShiftRight(Register, i16),
    GetCharacter(Register),
    PutCharacter(Register),
    BranchIfZero(Register, i16),
    BranchIfNonZero(Register, i16),
    Jump(i16),
    JumpRegister(Register),
    JumpLink(Register, i16),
    JumpLinkRegister(Register, Register),
    NoOp,
    Halt,
}

impl ToString for Instruction
{
    fn to_string(&self) -> String {
        match self
        {
            Instruction::LoadWord(ri, rj, k) => {
                format!("lw {:?},{}({:?})", ri, k, rj)
            }
            Instruction::LoadByte(ri, rj, k) => {
                format!("lb {:?},{}({:?})", ri, k, rj)
            }
            Instruction::StoreWord(ri, rj, k) => {
                format!("sw {}({:?}),({:?})", k, rj, ri)
            }
            Instruction::StoreByte(ri, rj, k) => {
                format!("sb {}({:?}),({:?})", k, rj, ri)
            }
            Instruction::Add(ri, rj, rk) => {
                format!("add {:?},{:?},{:?}", ri, rj, rk)
            }
            Instruction::Substract(ri, rj, rk) => {
                format!("sub {:?},{:?},{:?}", ri, rj, rk)
            }
            Instruction::Multiply(ri, rj, rk) => {
                format!("mul {:?},{:?},{:?}", ri, rj, rk)
            }
            Instruction::Divide(ri, rj, rk) => {
                format!("divide {:?},{:?},{:?}", ri, rj, rk)
            }
            Instruction::Modulus(_, _, _) => { unimplemented!()}
            Instruction::And(ri, rj, rk) => {
                format!("and {:?},{:?},{:?}", ri, rj, rk)
            }
            Instruction::Or(ri, rj, rk) => {
                format!("or {:?},{:?},{:?}", ri, rj, rk)
            }
            Instruction::Not(ri, rj) => {
                format!("not {:?},{:?}", ri, rj)
            }
            Instruction::Equal(ri, rj, rk) => {
                format!("ceq {:?},{:?},{:?}", ri, rj, rk)
            }
            Instruction::NotEqual(ri, rj, rk) => {
                format!("cne {:?},{:?},{:?}", ri, rj, rk)
            }
            Instruction::Less(ri, rj, rk) => {
                format!("clt {:?},{:?},{:?}", ri, rj, rk)
            }
            Instruction::LessEqual(ri, rj, rk) => {
                format!("cle {:?},{:?},{:?}", ri, rj, rk)
            }
            Instruction::Greater(ri, rj, rk) => {
                format!("cgt {:?},{:?},{:?}", ri, rj, rk)
            }
            Instruction::GreaterEqual(ri, rj, rk) => {
                format!("cge {:?},{:?},{:?}", ri, rj, rk)
            }
            Instruction::AddImmediate(ri, rj, k) => {
                format!("addi {:?},{:?},{}", ri, rj, k)
            }
            Instruction::SubstractImmediate(ri, rj, k) => {
                format!("subi {:?},{:?},{}", ri, rj, k)
            }
            Instruction::MultiplyImmediate(ri, rj, k) => {
                format!("muli {:?},{:?},{}", ri, rj, k)
            }
            Instruction::DivideImmediate(ri, rj, k) => {
                format!("devi {:?},{:?},{}", ri, rj, k)
            }
            Instruction::ModulusImmediate(_, _, _) => { unimplemented!() }
            Instruction::AndImmediate(ri, rj, k) => {
                format!("andi {:?},{:?},{}", ri, rj, k)
            }
            Instruction::OrImmediate(ri, rj, k) => {
                format!("ori {:?},{:?},{}", ri, rj, k)
            }
            Instruction::EqualImmediate(ri, rj, k) => {
                format!("ceqi {:?},{:?},{}", ri, rj, k)
            }
            Instruction::NotEqualImmediate(ri, rj, k) => {
                format!("cnei {:?},{:?},{}", ri, rj, k)
            }
            Instruction::LessImmediate(ri, rj, k) => {
                format!("clti {:?},{:?},{}", ri, rj, k)
            }
            Instruction::LessEqualImmediate(ri, rj, k) => {
                format!("clei {:?},{:?},{}", ri, rj, k)
            }
            Instruction::GreaterImmediate(ri, rj, k) => {
                format!("cgti {:?},{:?},{}", ri, rj, k)
            }
            Instruction::GreaterEqualImmediate(ri, rj, k) => {
                format!("cgei {:?},{:?},{}", ri, rj, k)
            }
            Instruction::ShiftLeft(ri, k) => {
                format!("sl {:?},{}", ri, k)
            }
            Instruction::ShiftRight(ri, k) => {
                format!("sr {:?},{}", ri, k)
            }
            Instruction::GetCharacter(ri) => {
                format!("getc {:?}", ri)
            }
            Instruction::PutCharacter(ri) => {
                format!("putc {:?}", ri)
            }
            Instruction::BranchIfZero(ri, k) => {
                format!("bz {:?},{}", ri, k)
            }
            Instruction::BranchIfNonZero(ri, k) => {
                format!("bnz {:?},{}", ri, k)
            }
            Instruction::Jump(k) => {
                format!("j {}", k)
            }
            Instruction::JumpRegister(ri) => {
                format!("jr {:?}", ri)
            }
            Instruction::JumpLink(ri, k) => {
                format!("jl {:?},{}", ri, k)
            }
            Instruction::JumpLinkRegister(ri, rj) => {
                format!("jlr {:?},{:?}", ri, rj)
            }
            Instruction::NoOp => { format!("nop") }
            Instruction::Halt => { format!("hlt") }
        }
    }
}

#[derive(Debug)]
struct TaggedInstruction(String, Instruction);

impl ToString for TaggedInstruction
{
    fn to_string(&self) -> String {
        format!("{}\t{}", self.0, self.1.to_string())
    }
}

#[derive(Debug)]
pub enum Directive
{
    Entry,
    Align,
    Org(u32),
    StoreWord(String),
    StoreByte(String), //todo is this correct
    Res(u32)
}

impl ToString for Directive
{
    fn to_string(&self) -> String {
        match self
        {
            Directive::Entry => {
                format!("entry")
            }
            Directive::Align => {
                format!("align")
            }
            Directive::Org(k) => {
                format!("org {}", k)
            }
            Directive::StoreWord(words) => {
                format!("dw {}", words)
            }
            Directive::StoreByte(bytes) => {
                format!("dw {}", bytes)
            }
            Directive::Res(b) => {
                format!("res {}", b)
            }
        }
    }
}

#[derive(Debug)]
struct TaggedDirective(String, Directive);

impl ToString for TaggedDirective
{
    fn to_string(&self) -> String {
        format!("{}\t{}", self.0, self.1.to_string())
    }
}

#[derive(Debug)]
pub enum Register
{
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R16,
    Tagged(String),
}
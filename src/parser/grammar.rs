use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::lexer::token::TokenType;
use crate::lexer::token::TokenType::*;
use crate::parser::grammar::NamedSymbol::*;
use crate::parser::grammar::GrammarSymbol::*;
use crate::lexer::token::TokenType::OpenSquare;

lazy_static!
{
    pub static ref PARSING_TABLE: HashMap<(GrammarSymbol, GrammarSymbol), GrammarRule> =
    {
        let mut table: HashMap<(GrammarSymbol, GrammarSymbol), GrammarRule> = HashMap::new();

        table.insert((START, Terminal(Main)), GrammarRule{lhs: START, rhs: vec![NonTerminal(Prog)]});
        table.insert((START, Terminal(Func)), GrammarRule{lhs: START, rhs: vec![NonTerminal(Prog)]});
        table.insert((START, Terminal(Class)), GrammarRule{lhs: START, rhs: vec![NonTerminal(Prog)]});

        table.insert((NonTerminal(AddOp), Terminal(Minus)), GrammarRule{lhs: NonTerminal(AddOp), rhs: vec![Terminal(Minus)]});
        table.insert((NonTerminal(AddOp), Terminal(Plus)), GrammarRule{lhs: NonTerminal(AddOp), rhs: vec![Terminal(Plus)]});
        table.insert((NonTerminal(AddOp), Terminal(Or)), GrammarRule{lhs: NonTerminal(AddOp), rhs: vec![Terminal(Or)]});

        table.insert((NonTerminal(ArithExpr), Terminal(Id)), GrammarRule{lhs: NonTerminal(ArithExpr), rhs: vec![NonTerminal(Term), NonTerminal(RightRecArithExpr)]});
        table.insert((NonTerminal(ArithExpr), Terminal(OpenParen)), GrammarRule{lhs: NonTerminal(ArithExpr), rhs: vec![NonTerminal(Term), NonTerminal(RightRecArithExpr)]});
        table.insert((NonTerminal(ArithExpr), Terminal(Minus)), GrammarRule{lhs: NonTerminal(ArithExpr), rhs: vec![NonTerminal(Term), NonTerminal(RightRecArithExpr)]});
        table.insert((NonTerminal(ArithExpr), Terminal(Plus)), GrammarRule{lhs: NonTerminal(ArithExpr), rhs: vec![NonTerminal(Term), NonTerminal(RightRecArithExpr)]});
        table.insert((NonTerminal(ArithExpr), Terminal(Question)), GrammarRule{lhs: NonTerminal(ArithExpr), rhs: vec![NonTerminal(Term), NonTerminal(RightRecArithExpr)]});
        table.insert((NonTerminal(ArithExpr), Terminal(Bang)), GrammarRule{lhs: NonTerminal(ArithExpr), rhs: vec![NonTerminal(Term), NonTerminal(RightRecArithExpr)]});
        table.insert((NonTerminal(ArithExpr), Terminal(StringLit)), GrammarRule{lhs: NonTerminal(ArithExpr), rhs: vec![NonTerminal(Term), NonTerminal(RightRecArithExpr)]});
        table.insert((NonTerminal(ArithExpr), Terminal(FloatLit)), GrammarRule{lhs: NonTerminal(ArithExpr), rhs: vec![NonTerminal(Term), NonTerminal(RightRecArithExpr)]});
        table.insert((NonTerminal(ArithExpr), Terminal(IntegerLit)), GrammarRule{lhs: NonTerminal(ArithExpr), rhs: vec![NonTerminal(Term), NonTerminal(RightRecArithExpr)]});

        table.insert((NonTerminal(ArraySize), Terminal(OpenSquare)), GrammarRule{lhs: NonTerminal(ArraySize), rhs: vec![Terminal(OpenSquare), NonTerminal(ArraySizeAmb1)]});

        table.insert((NonTerminal(ArraySizeAmb1), Terminal(CloseSquare)), GrammarRule{lhs: NonTerminal(ArraySizeAmb1), rhs: vec![Terminal(CloseSquare)]});
        table.insert((NonTerminal(ArraySizeAmb1), Terminal(IntegerLit)), GrammarRule{lhs: NonTerminal(ArraySizeAmb1), rhs: vec![Terminal(IntegerLit), Terminal(CloseSquare)]}); //todo inNum??

        table.insert((NonTerminal(AssignOp), Terminal(Assignment)), GrammarRule{lhs: NonTerminal(AssignOp), rhs: vec![Terminal(Assignment)]});

        table.insert((NonTerminal(ClassDecl), Terminal(Class)), GrammarRule{lhs: NonTerminal(ClassDecl), rhs: vec![Terminal(Class), Terminal(Id), NonTerminal(OptClassDecl), Terminal(OpenCurly), NonTerminal(ReptClassDecl), Terminal(CloseCurly), Terminal(SemiColon)]});

        table.insert((NonTerminal(Expr), Terminal(Id)), GrammarRule{lhs: NonTerminal(Expr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(ExprAmb1)]});
        table.insert((NonTerminal(Expr), Terminal(OpenParen)), GrammarRule{lhs: NonTerminal(Expr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(ExprAmb1)]});
        table.insert((NonTerminal(Expr), Terminal(Minus)), GrammarRule{lhs: NonTerminal(Expr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(ExprAmb1)]});
        table.insert((NonTerminal(Expr), Terminal(Plus)), GrammarRule{lhs: NonTerminal(Expr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(ExprAmb1)]});
        table.insert((NonTerminal(Expr), Terminal(Question)), GrammarRule{lhs: NonTerminal(Expr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(ExprAmb1)]});
        table.insert((NonTerminal(Expr), Terminal(Bang)), GrammarRule{lhs: NonTerminal(Expr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(ExprAmb1)]});
        table.insert((NonTerminal(Expr), Terminal(StringLit)), GrammarRule{lhs: NonTerminal(Expr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(ExprAmb1)]});
        table.insert((NonTerminal(Expr), Terminal(FloatLit)), GrammarRule{lhs: NonTerminal(Expr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(ExprAmb1)]});
        table.insert((NonTerminal(Expr), Terminal(IntegerLit)), GrammarRule{lhs: NonTerminal(Expr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(ExprAmb1)]});

        table.insert((NonTerminal(ExprAmb1), Terminal(CloseParen)), GrammarRule{lhs: NonTerminal(ExprAmb1), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ExprAmb1), Terminal(SemiColon)), GrammarRule{lhs: NonTerminal(ExprAmb1), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ExprAmb1), Terminal(Comma)), GrammarRule{lhs: NonTerminal(ExprAmb1), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ExprAmb1), Terminal(GreaterEqualThan)), GrammarRule{lhs: NonTerminal(ExprAmb1), rhs: vec![NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(ExprAmb1), Terminal(LessEqualThan)), GrammarRule{lhs: NonTerminal(ExprAmb1), rhs: vec![NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(ExprAmb1), Terminal(GreaterThan)), GrammarRule{lhs: NonTerminal(ExprAmb1), rhs: vec![NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(ExprAmb1), Terminal(LessThan)), GrammarRule{lhs: NonTerminal(ExprAmb1), rhs: vec![NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(ExprAmb1), Terminal(NotEq)), GrammarRule{lhs: NonTerminal(ExprAmb1), rhs: vec![NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(ExprAmb1), Terminal(EqEq)), GrammarRule{lhs: NonTerminal(ExprAmb1), rhs: vec![NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(ExprAmb1), Terminal(CloseSquare)), GrammarRule{lhs: NonTerminal(ExprAmb1), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ExprAmb1), Terminal(Colon)), GrammarRule{lhs: NonTerminal(ExprAmb1), rhs: vec![EPSILON]});

        table.insert((NonTerminal(Factor), Terminal(Id)), GrammarRule{lhs: NonTerminal(Factor), rhs: vec![Terminal(Id), NonTerminal(FactorAmb1)]});
        table.insert((NonTerminal(Factor), Terminal(OpenParen)), GrammarRule{lhs: NonTerminal(Factor), rhs: vec![Terminal(OpenParen), NonTerminal(ArithExpr), Terminal(CloseParen)]});
        table.insert((NonTerminal(Factor), Terminal(Minus)), GrammarRule{lhs: NonTerminal(Factor), rhs: vec![NonTerminal(Sign), NonTerminal(Factor)]});
        table.insert((NonTerminal(Factor), Terminal(Plus)), GrammarRule{lhs: NonTerminal(Factor), rhs: vec![NonTerminal(Sign), NonTerminal(Factor)]});
        table.insert((NonTerminal(Factor), Terminal(Question)), GrammarRule{lhs: NonTerminal(Factor), rhs: vec![Terminal(Question), Terminal(OpenSquare), NonTerminal(Expr), Terminal(Colon), NonTerminal(Expr), Terminal(Colon), NonTerminal(Expr), Terminal(CloseSquare)]});
        table.insert((NonTerminal(Factor), Terminal(Bang)), GrammarRule{lhs: NonTerminal(Factor), rhs: vec![Terminal(Bang), NonTerminal(Factor)]});
        table.insert((NonTerminal(Factor), Terminal(StringLit)), GrammarRule{lhs: NonTerminal(Factor), rhs: vec![Terminal(StringLit)]});
        table.insert((NonTerminal(Factor), Terminal(FloatLit)), GrammarRule{lhs: NonTerminal(Factor), rhs: vec![Terminal(FloatLit)]});
        table.insert((NonTerminal(Factor), Terminal(IntegerLit)), GrammarRule{lhs: NonTerminal(Factor), rhs: vec![Terminal(IntegerLit)]});

        table.insert((NonTerminal(FactorAmb1), Terminal(Period)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(CloseParen)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(OpenParen)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![Terminal(OpenParen), NonTerminal(Params), Terminal(CloseParen), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(SemiColon)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(Minus)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(Plus)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(Comma)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(GreaterEqualThan)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(LessEqualThan)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(GreaterThan)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(LessThan)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(NotEq)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(EqEq)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(And)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(Div)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(Mult)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(CloseSquare)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(OpenSquare)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(Colon)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});
        table.insert((NonTerminal(FactorAmb1), Terminal(Or)), GrammarRule{lhs: NonTerminal(FactorAmb1), rhs: vec![NonTerminal(ReptVariable), NonTerminal(FactorAmb2)]});


        table
    };
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum GrammarSymbol
{
    Terminal(TokenType),
    NonTerminal(NamedSymbol),
    EPSILON,
    START,
    STOP
}

#[derive(Eq, PartialEq, Debug)]
pub struct GrammarRule
{
    pub lhs: GrammarSymbol,
    pub rhs: Vec<GrammarSymbol>
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum NamedSymbol
{
    AddOp,
    ArithExpr,
    ArraySize,
    ArraySizeAmb1,
    AssignOp,
    ClassDecl,
    Expr,
    ExprAmb1,
    Factor,
    FactorAmb1,
    FactorAmb2,
    FuncBody,
    FuncDecl,
    FuncDeclAmb1,
    FuncDef,
    FuncHead,
    FuncHeadAmb1,
    FuncHeadAmb2,
    FuncParams,
    Indice,
    MemberDecl,
    MultOp,
    OptClassDecl,
    OptFuncBody,
    Params,
    Prog,
    RelExpr,
    RelOp,
    ReptClassDecl,
    ReptFuncBody,
    ReptFuncParams0,
    ReptFuncParams1,
    ReptFuncParamsTail,
    ReptOptClassDecl,
    ReptOptFuncBody,
    ReptParams,
    ReptProg0,
    ReptProg1,
    ReptStatBlock,
    ReptVarDecl,
    ReptVariable,
    RightRecArithExpr,
    RightRecTerm,
    Sign,
    StatBlock,
    Statement,
    StatementAmb1,
    StatementAmb2,
    StatementAmb3,
    Term,
    Type,
    VarDecl,
    Variable,
    VariableAmb1,
    Visibility
}
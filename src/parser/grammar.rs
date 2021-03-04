use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::lexer::token::{TokenType, Token};
use crate::lexer::token::TokenType::*;
use crate::parser::grammar::NamedSymbol::*;
use crate::parser::grammar::GrammarSymbol::*;
use crate::lexer::token::TokenType::OpenSquare;

const START_FIRST: &'static [GrammarSymbol] = &[Terminal(Main), Terminal(Func), Terminal(Class)];
const STATEMENT_FIRST: &'static [GrammarSymbol] = &[Terminal(Write), Terminal(Return), Terminal(Id), Terminal(If), Terminal(Read), Terminal(While), Terminal(Break), Terminal(Continue)];
const REPTOPTCLASSECL_FIRST: &'static [GrammarSymbol] = &[Terminal(Comma), EPSILON];
//const START_FOLLOW: &'static [TokenType] = &[Main, Func, Class];


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

        table.insert((NonTerminal(FactorAmb2), Terminal(Period)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![Terminal(Id), NonTerminal(FactorAmb1)]});
        table.insert((NonTerminal(FactorAmb2), Terminal(CloseParen)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(SemiColon)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(Minus)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(Plus)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(Comma)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(GreaterEqualThan)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(LessEqualThan)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(GreaterThan)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(LessThan)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(NotEq)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(EqEq)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(And)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(Div)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(Mult)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(CloseSquare)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(Colon)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FactorAmb2), Terminal(Or)), GrammarRule{lhs: NonTerminal(FactorAmb2), rhs: vec![EPSILON]});

        table.insert((NonTerminal(FuncBody), Terminal(OpenCurly)), GrammarRule{lhs: NonTerminal(FuncBody), rhs: vec![Terminal(OpenCurly), NonTerminal(OptFuncBody), NonTerminal(ReptFuncBody), Terminal(CloseCurly)]});

        table.insert((NonTerminal(FuncDecl), Terminal(Func)), GrammarRule{lhs: NonTerminal(FuncDecl), rhs: vec![Terminal(Func), Terminal(Id), Terminal(OpenParen), NonTerminal(FuncParams), Terminal(CloseParen), Terminal(Colon), NonTerminal(FuncDeclAmb1)]});

        table.insert((NonTerminal(FuncDeclAmb1), Terminal(Id)), GrammarRule{lhs: NonTerminal(FuncDeclAmb1), rhs: vec![NonTerminal(Type), Terminal(SemiColon)]});
        table.insert((NonTerminal(FuncDeclAmb1), Terminal(StringType)), GrammarRule{lhs: NonTerminal(FuncDeclAmb1), rhs: vec![NonTerminal(Type), Terminal(SemiColon)]});
        table.insert((NonTerminal(FuncDeclAmb1), Terminal(IntegerType)), GrammarRule{lhs: NonTerminal(FuncDeclAmb1), rhs: vec![NonTerminal(Type), Terminal(SemiColon)]});
        table.insert((NonTerminal(FuncDeclAmb1), Terminal(FloatType)), GrammarRule{lhs: NonTerminal(FuncDeclAmb1), rhs: vec![NonTerminal(Type), Terminal(SemiColon)]});
        table.insert((NonTerminal(FuncDeclAmb1), Terminal(Void)), GrammarRule{lhs: NonTerminal(FuncDeclAmb1), rhs: vec![Terminal(Void), Terminal(SemiColon)]});

        table.insert((NonTerminal(FuncDef), Terminal(Func)), GrammarRule{lhs: NonTerminal(FuncDef), rhs: vec![NonTerminal(FuncHead), NonTerminal(FuncBody)]});

        table.insert((NonTerminal(FuncHead), Terminal(Func)), GrammarRule{lhs: NonTerminal(FuncHead), rhs: vec![Terminal(Func), Terminal(Id), NonTerminal(FuncHeadAmb1)]});

        table.insert((NonTerminal(FuncHeadAmb1), Terminal(OpenParen)), GrammarRule{lhs: NonTerminal(FuncHeadAmb1), rhs: vec![Terminal(OpenParen), NonTerminal(FuncParams), Terminal(CloseParen), Terminal(Colon), NonTerminal(FuncHeadAmb2)]});
        table.insert((NonTerminal(FuncHeadAmb1), Terminal(DoubleColon)), GrammarRule{lhs: NonTerminal(FuncHeadAmb1), rhs: vec![Terminal(DoubleColon), Terminal(Id), Terminal(OpenParen), NonTerminal(FuncParams), Terminal(CloseParen), Terminal(Colon), NonTerminal(FuncHeadAmb2)]});

        table.insert((NonTerminal(FuncHeadAmb2), Terminal(Id)), GrammarRule{lhs: NonTerminal(FuncHeadAmb2), rhs: vec![NonTerminal(Type)]});
        table.insert((NonTerminal(FuncHeadAmb2), Terminal(StringType)), GrammarRule{lhs: NonTerminal(FuncHeadAmb2), rhs: vec![NonTerminal(Type)]});
        table.insert((NonTerminal(FuncHeadAmb2), Terminal(FloatType)), GrammarRule{lhs: NonTerminal(FuncHeadAmb2), rhs: vec![NonTerminal(Type)]});
        table.insert((NonTerminal(FuncHeadAmb2), Terminal(IntegerType)), GrammarRule{lhs: NonTerminal(FuncHeadAmb2), rhs: vec![NonTerminal(Type)]});
        table.insert((NonTerminal(FuncHeadAmb2), Terminal(Void)), GrammarRule{lhs: NonTerminal(FuncHeadAmb2), rhs: vec![Terminal(Void)]});

        table.insert((NonTerminal(FuncParams), Terminal(Id)), GrammarRule{lhs: NonTerminal(FuncParams), rhs: vec![NonTerminal(Type), Terminal(Id), NonTerminal(ReptFuncParams0), NonTerminal(ReptFuncParams1)]});
        table.insert((NonTerminal(FuncParams), Terminal(CloseParen)), GrammarRule{lhs: NonTerminal(FuncParams), rhs: vec![EPSILON]});
        table.insert((NonTerminal(FuncParams), Terminal(StringType)), GrammarRule{lhs: NonTerminal(FuncParams), rhs: vec![NonTerminal(Type), Terminal(Id), NonTerminal(ReptFuncParams0), NonTerminal(ReptFuncParams1)]});
        table.insert((NonTerminal(FuncParams), Terminal(FloatType)), GrammarRule{lhs: NonTerminal(FuncParams), rhs: vec![NonTerminal(Type), Terminal(Id), NonTerminal(ReptFuncParams0), NonTerminal(ReptFuncParams1)]});
        table.insert((NonTerminal(FuncParams), Terminal(IntegerType)), GrammarRule{lhs: NonTerminal(FuncParams), rhs: vec![NonTerminal(Type), Terminal(Id), NonTerminal(ReptFuncParams0), NonTerminal(ReptFuncParams1)]});

        table.insert((NonTerminal(Indice), Terminal(OpenSquare)), GrammarRule{lhs: NonTerminal(Indice), rhs: vec![Terminal(OpenSquare), NonTerminal(ArithExpr), Terminal(CloseSquare)]});

        table.insert((NonTerminal(MemberDecl), Terminal(Id)), GrammarRule{lhs: NonTerminal(MemberDecl), rhs: vec![NonTerminal(VarDecl)]});
        table.insert((NonTerminal(MemberDecl), Terminal(StringType)), GrammarRule{lhs: NonTerminal(MemberDecl), rhs: vec![NonTerminal(VarDecl)]});
        table.insert((NonTerminal(MemberDecl), Terminal(FloatType)), GrammarRule{lhs: NonTerminal(MemberDecl), rhs: vec![NonTerminal(VarDecl)]});
        table.insert((NonTerminal(MemberDecl), Terminal(IntegerType)), GrammarRule{lhs: NonTerminal(MemberDecl), rhs: vec![NonTerminal(VarDecl)]});
        table.insert((NonTerminal(MemberDecl), Terminal(Func)), GrammarRule{lhs: NonTerminal(MemberDecl), rhs: vec![NonTerminal(FuncDecl)]});

        table.insert((NonTerminal(MultOp), Terminal(And)), GrammarRule{lhs: NonTerminal(MultOp), rhs: vec![Terminal(And)]});
        table.insert((NonTerminal(MultOp), Terminal(Div)), GrammarRule{lhs: NonTerminal(MultOp), rhs: vec![Terminal(Div)]});
        table.insert((NonTerminal(MultOp), Terminal(Mult)), GrammarRule{lhs: NonTerminal(MultOp), rhs: vec![Terminal(Mult)]});

        table.insert((NonTerminal(OptClassDecl), Terminal(OpenCurly)), GrammarRule{lhs: NonTerminal(OptClassDecl), rhs: vec![EPSILON]});
        table.insert((NonTerminal(OptClassDecl), Terminal(Inherits)), GrammarRule{lhs: NonTerminal(OptClassDecl), rhs: vec![Terminal(Inherits), Terminal(Id), NonTerminal(ReptOptClassDecl)]});

        table.insert((NonTerminal(OptFuncBody), Terminal(Id)), GrammarRule{lhs: NonTerminal(OptFuncBody), rhs: vec![EPSILON]});
        table.insert((NonTerminal(OptFuncBody), Terminal(Continue)), GrammarRule{lhs: NonTerminal(OptFuncBody), rhs: vec![EPSILON]});
        table.insert((NonTerminal(OptFuncBody), Terminal(Break)), GrammarRule{lhs: NonTerminal(OptFuncBody), rhs: vec![EPSILON]});
        table.insert((NonTerminal(OptFuncBody), Terminal(Return)), GrammarRule{lhs: NonTerminal(OptFuncBody), rhs: vec![EPSILON]});
        table.insert((NonTerminal(OptFuncBody), Terminal(Write)), GrammarRule{lhs: NonTerminal(OptFuncBody), rhs: vec![EPSILON]});
        table.insert((NonTerminal(OptFuncBody), Terminal(Read)), GrammarRule{lhs: NonTerminal(OptFuncBody), rhs: vec![EPSILON]});
        table.insert((NonTerminal(OptFuncBody), Terminal(While)), GrammarRule{lhs: NonTerminal(OptFuncBody), rhs: vec![EPSILON]});
        table.insert((NonTerminal(OptFuncBody), Terminal(If)), GrammarRule{lhs: NonTerminal(OptFuncBody), rhs: vec![EPSILON]});
        table.insert((NonTerminal(OptFuncBody), Terminal(CloseCurly)), GrammarRule{lhs: NonTerminal(OptFuncBody), rhs: vec![EPSILON]});
        table.insert((NonTerminal(OptFuncBody), Terminal(Var)), GrammarRule{lhs: NonTerminal(OptFuncBody), rhs: vec![Terminal(Var), Terminal(OpenCurly), NonTerminal(ReptOptFuncBody), Terminal(CloseCurly)]});

        table.insert((NonTerminal(Params), Terminal(Id)), GrammarRule{lhs: NonTerminal(Params), rhs: vec![NonTerminal(Expr), NonTerminal(ReptParams)]});
        table.insert((NonTerminal(Params), Terminal(CloseParen)), GrammarRule{lhs: NonTerminal(Params), rhs: vec![EPSILON]});
        table.insert((NonTerminal(Params), Terminal(OpenParen)), GrammarRule{lhs: NonTerminal(Params), rhs: vec![NonTerminal(Expr), NonTerminal(ReptParams)]});
        table.insert((NonTerminal(Params), Terminal(Minus)), GrammarRule{lhs: NonTerminal(Params), rhs: vec![NonTerminal(Expr), NonTerminal(ReptParams)]});
        table.insert((NonTerminal(Params), Terminal(Plus)), GrammarRule{lhs: NonTerminal(Params), rhs: vec![NonTerminal(Expr), NonTerminal(ReptParams)]});
        table.insert((NonTerminal(Params), Terminal(Question)), GrammarRule{lhs: NonTerminal(Params), rhs: vec![NonTerminal(Expr), NonTerminal(ReptParams)]});
        table.insert((NonTerminal(Params), Terminal(Bang)), GrammarRule{lhs: NonTerminal(Params), rhs: vec![NonTerminal(Expr), NonTerminal(ReptParams)]});
        table.insert((NonTerminal(Params), Terminal(StringLit)), GrammarRule{lhs: NonTerminal(Params), rhs: vec![NonTerminal(Expr), NonTerminal(ReptParams)]});
        table.insert((NonTerminal(Params), Terminal(FloatLit)), GrammarRule{lhs: NonTerminal(Params), rhs: vec![NonTerminal(Expr), NonTerminal(ReptParams)]});
        table.insert((NonTerminal(Params), Terminal(IntegerLit)), GrammarRule{lhs: NonTerminal(Params), rhs: vec![NonTerminal(Expr), NonTerminal(ReptParams)]});

        table.insert((NonTerminal(Prog), Terminal(Main)), GrammarRule{lhs: NonTerminal(Prog), rhs: vec![NonTerminal(ReptProg0), NonTerminal(ReptProg1), Terminal(Main), NonTerminal(FuncBody)]});
        table.insert((NonTerminal(Prog), Terminal(Func)), GrammarRule{lhs: NonTerminal(Prog), rhs: vec![NonTerminal(ReptProg0), NonTerminal(ReptProg1), Terminal(Main), NonTerminal(FuncBody)]});
        table.insert((NonTerminal(Prog), Terminal(Class)), GrammarRule{lhs: NonTerminal(Prog), rhs: vec![NonTerminal(ReptProg0), NonTerminal(ReptProg1), Terminal(Main), NonTerminal(FuncBody)]});

        table.insert((NonTerminal(RelExpr), Terminal(Id)), GrammarRule{lhs: NonTerminal(RelExpr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(RelExpr), Terminal(OpenParen)), GrammarRule{lhs: NonTerminal(RelExpr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(RelExpr), Terminal(Minus)), GrammarRule{lhs: NonTerminal(RelExpr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(RelExpr), Terminal(Plus)), GrammarRule{lhs: NonTerminal(RelExpr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(RelExpr), Terminal(Question)), GrammarRule{lhs: NonTerminal(RelExpr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(RelExpr), Terminal(Bang)), GrammarRule{lhs: NonTerminal(RelExpr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(RelExpr), Terminal(StringLit)), GrammarRule{lhs: NonTerminal(RelExpr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(RelExpr), Terminal(FloatLit)), GrammarRule{lhs: NonTerminal(RelExpr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(RelOp), NonTerminal(ArithExpr)]});
        table.insert((NonTerminal(RelExpr), Terminal(IntegerLit)), GrammarRule{lhs: NonTerminal(RelExpr), rhs: vec![NonTerminal(ArithExpr), NonTerminal(RelOp), NonTerminal(ArithExpr)]});

        table.insert((NonTerminal(RelOp), Terminal(GreaterEqualThan)), GrammarRule{lhs: NonTerminal(RelOp), rhs: vec![Terminal(GreaterEqualThan)]});
        table.insert((NonTerminal(RelOp), Terminal(LessEqualThan)), GrammarRule{lhs: NonTerminal(RelOp), rhs: vec![Terminal(LessEqualThan)]});
        table.insert((NonTerminal(RelOp), Terminal(GreaterThan)), GrammarRule{lhs: NonTerminal(RelOp), rhs: vec![Terminal(GreaterThan)]});
        table.insert((NonTerminal(RelOp), Terminal(LessThan)), GrammarRule{lhs: NonTerminal(RelOp), rhs: vec![Terminal(LessThan)]});
        table.insert((NonTerminal(RelOp), Terminal(NotEq)), GrammarRule{lhs: NonTerminal(RelOp), rhs: vec![Terminal(NotEq)]});
        table.insert((NonTerminal(RelOp), Terminal(EqEq)), GrammarRule{lhs: NonTerminal(RelOp), rhs: vec![Terminal(EqEq)]});

        table.insert((NonTerminal(ReptClassDecl), Terminal(Private)), GrammarRule{lhs: NonTerminal(ReptClassDecl), rhs: vec![NonTerminal(Visibility), NonTerminal(MemberDecl), NonTerminal(ReptClassDecl)]});
        table.insert((NonTerminal(ReptClassDecl), Terminal(Public)), GrammarRule{lhs: NonTerminal(ReptClassDecl), rhs: vec![NonTerminal(Visibility), NonTerminal(MemberDecl), NonTerminal(ReptClassDecl)]});
        table.insert((NonTerminal(ReptClassDecl), Terminal(Id)), GrammarRule{lhs: NonTerminal(ReptClassDecl), rhs: vec![NonTerminal(Visibility), NonTerminal(MemberDecl), NonTerminal(ReptClassDecl)]});
        table.insert((NonTerminal(ReptClassDecl), Terminal(StringType)), GrammarRule{lhs: NonTerminal(ReptClassDecl), rhs: vec![NonTerminal(Visibility), NonTerminal(MemberDecl), NonTerminal(ReptClassDecl)]});
        table.insert((NonTerminal(ReptClassDecl), Terminal(FloatType)), GrammarRule{lhs: NonTerminal(ReptClassDecl), rhs: vec![NonTerminal(Visibility), NonTerminal(MemberDecl), NonTerminal(ReptClassDecl)]});
        table.insert((NonTerminal(ReptClassDecl), Terminal(IntegerType)), GrammarRule{lhs: NonTerminal(ReptClassDecl), rhs: vec![NonTerminal(Visibility), NonTerminal(MemberDecl), NonTerminal(ReptClassDecl)]});
        table.insert((NonTerminal(ReptClassDecl), Terminal(CloseCurly)), GrammarRule{lhs: NonTerminal(ReptClassDecl), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptClassDecl), Terminal(Func)), GrammarRule{lhs: NonTerminal(ReptClassDecl), rhs: vec![NonTerminal(Visibility), NonTerminal(MemberDecl), NonTerminal(ReptClassDecl)]});

        table.insert((NonTerminal(ReptFuncBody), Terminal(Id)), GrammarRule{lhs: NonTerminal(ReptFuncBody), rhs: vec![NonTerminal(Statement), NonTerminal(ReptFuncBody)]});
        table.insert((NonTerminal(ReptFuncBody), Terminal(Continue)), GrammarRule{lhs: NonTerminal(ReptFuncBody), rhs: vec![NonTerminal(Statement), NonTerminal(ReptFuncBody)]});
        table.insert((NonTerminal(ReptFuncBody), Terminal(Break)), GrammarRule{lhs: NonTerminal(ReptFuncBody), rhs: vec![NonTerminal(Statement), NonTerminal(ReptFuncBody)]});
        table.insert((NonTerminal(ReptFuncBody), Terminal(Return)), GrammarRule{lhs: NonTerminal(ReptFuncBody), rhs: vec![NonTerminal(Statement), NonTerminal(ReptFuncBody)]});
        table.insert((NonTerminal(ReptFuncBody), Terminal(Write)), GrammarRule{lhs: NonTerminal(ReptFuncBody), rhs: vec![NonTerminal(Statement), NonTerminal(ReptFuncBody)]});
        table.insert((NonTerminal(ReptFuncBody), Terminal(Read)), GrammarRule{lhs: NonTerminal(ReptFuncBody), rhs: vec![NonTerminal(Statement), NonTerminal(ReptFuncBody)]});
        table.insert((NonTerminal(ReptFuncBody), Terminal(While)), GrammarRule{lhs: NonTerminal(ReptFuncBody), rhs: vec![NonTerminal(Statement), NonTerminal(ReptFuncBody)]});
        table.insert((NonTerminal(ReptFuncBody), Terminal(If)), GrammarRule{lhs: NonTerminal(ReptFuncBody), rhs: vec![NonTerminal(Statement), NonTerminal(ReptFuncBody)]});
        table.insert((NonTerminal(ReptFuncBody), Terminal(CloseCurly)), GrammarRule{lhs: NonTerminal(ReptFuncBody), rhs: vec![EPSILON]});

        table.insert((NonTerminal(ReptFuncParams0), Terminal(CloseParen)), GrammarRule{lhs: NonTerminal(ReptFuncParams0), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptFuncParams0), Terminal(Comma)), GrammarRule{lhs: NonTerminal(ReptFuncParams0), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptFuncParams0), Terminal(OpenSquare)), GrammarRule{lhs: NonTerminal(ReptFuncParams0), rhs: vec![NonTerminal(ArraySize), NonTerminal(ReptFuncParams0)]});

        table.insert((NonTerminal(ReptFuncParams1), Terminal(CloseParen)), GrammarRule{lhs: NonTerminal(ReptFuncParams1), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptFuncParams1), Terminal(Comma)), GrammarRule{lhs: NonTerminal(ReptFuncParams1), rhs: vec![Terminal(Comma), NonTerminal(Type), Terminal(Id), NonTerminal(ReptFuncParamsTail), NonTerminal(ReptFuncParams1)]});

        table.insert((NonTerminal(ReptFuncParamsTail), Terminal(CloseParen)), GrammarRule{lhs: NonTerminal(ReptFuncParamsTail), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptFuncParamsTail), Terminal(Comma)), GrammarRule{lhs: NonTerminal(ReptFuncParamsTail), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptFuncParamsTail), Terminal(OpenSquare)), GrammarRule{lhs: NonTerminal(ReptFuncParamsTail), rhs: vec![NonTerminal(ArraySize), NonTerminal(ReptFuncParams0)]});

        table.insert((NonTerminal(ReptOptClassDecl), Terminal(OpenCurly)), GrammarRule{lhs: NonTerminal(ReptOptClassDecl), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptOptClassDecl), Terminal(Comma)), GrammarRule{lhs: NonTerminal(ReptOptClassDecl), rhs: vec![Terminal(Comma), Terminal(Id), NonTerminal(ReptOptClassDecl)]});

        table.insert((NonTerminal(ReptOptFuncBody), Terminal(Id)), GrammarRule{lhs: NonTerminal(ReptOptFuncBody), rhs: vec![NonTerminal(VarDecl), NonTerminal(ReptOptFuncBody)]});
        table.insert((NonTerminal(ReptOptFuncBody), Terminal(StringLit)), GrammarRule{lhs: NonTerminal(ReptOptFuncBody), rhs: vec![NonTerminal(VarDecl), NonTerminal(ReptOptFuncBody)]});
        table.insert((NonTerminal(ReptOptFuncBody), Terminal(FloatLit)), GrammarRule{lhs: NonTerminal(ReptOptFuncBody), rhs: vec![NonTerminal(VarDecl), NonTerminal(ReptOptFuncBody)]});
        table.insert((NonTerminal(ReptOptFuncBody), Terminal(IntegerLit)), GrammarRule{lhs: NonTerminal(ReptOptFuncBody), rhs: vec![NonTerminal(VarDecl), NonTerminal(ReptOptFuncBody)]});
        table.insert((NonTerminal(ReptOptFuncBody), Terminal(CloseCurly)), GrammarRule{lhs: NonTerminal(ReptOptFuncBody), rhs: vec![EPSILON]});

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

impl GrammarSymbol
{
    pub fn first_set(&self) -> &[GrammarSymbol]
    {
        match self
        {
            Terminal(_) => { panic!() }
            NonTerminal(symb) => {
                match symb
                {
                    AddOp => {}
                    ArithExpr => {}
                    ArraySize => {}
                    ArraySizeAmb1 => {}
                    AssignOp => {}
                    ClassDecl => {}
                    Expr => {}
                    ExprAmb1 => {}
                    Factor => {}
                    FactorAmb1 => {}
                    FactorAmb2 => {}
                    FuncBody => {}
                    FuncDecl => {}
                    FuncDeclAmb1 => {}
                    FuncDef => {}
                    FuncHead => {}
                    FuncHeadAmb1 => {}
                    FuncHeadAmb2 => {}
                    FuncParams => {}
                    Indice => {}
                    MemberDecl => {}
                    MultOp => {}
                    OptClassDecl => {}
                    OptFuncBody => {}
                    Params => {}
                    Prog => {}
                    RelExpr => {}
                    RelOp => {}
                    ReptClassDecl => {}
                    ReptFuncBody => {}
                    ReptFuncParams0 => {}
                    ReptFuncParams1 => {}
                    ReptFuncParamsTail => {}
                    ReptOptClassDecl => {}
                    ReptOptFuncBody => {}
                    ReptParams => {}
                    ReptProg0 => {}
                    ReptProg1 => {}
                    ReptStatBlock => {}
                    ReptVarDecl => {}
                    ReptVariable => {}
                    RightRecArithExpr => {}
                    RightRecTerm => {}
                    Sign => {}
                    StatBlock => {}
                    Statement => {}
                    StatementAmb1 => {}
                    StatementAmb2 => {}
                    StatementAmb3 => {}
                    Term => {}
                    Type => {}
                    VarDecl => {}
                    Variable => {}
                    VariableAmb1 => {}
                    Visibility => {}
                }
            }
            EPSILON => { panic!() }
            START => { todo!() }
            STOP => { panic!() }
        }
        todo!()
    }

    pub fn follow_set(&self) -> &[GrammarSymbol]
    {
        match self
        {
            Terminal(_) => { panic!() }
            NonTerminal(symb) => {
                match symb
                {
                    AddOp => {}
                    ArithExpr => {}
                    ArraySize => {}
                    ArraySizeAmb1 => {}
                    AssignOp => {}
                    ClassDecl => {}
                    Expr => {}
                    ExprAmb1 => {}
                    Factor => {}
                    FactorAmb1 => {}
                    FactorAmb2 => {}
                    FuncBody => {}
                    FuncDecl => {}
                    FuncDeclAmb1 => {}
                    FuncDef => {}
                    FuncHead => {}
                    FuncHeadAmb1 => {}
                    FuncHeadAmb2 => {}
                    FuncParams => {}
                    Indice => {}
                    MemberDecl => {}
                    MultOp => {}
                    OptClassDecl => {}
                    OptFuncBody => {}
                    Params => {}
                    Prog => {}
                    RelExpr => {}
                    RelOp => {}
                    ReptClassDecl => {}
                    ReptFuncBody => {}
                    ReptFuncParams0 => {}
                    ReptFuncParams1 => {}
                    ReptFuncParamsTail => {}
                    ReptOptClassDecl => {}
                    ReptOptFuncBody => {}
                    ReptParams => {}
                    ReptProg0 => {}
                    ReptProg1 => {}
                    ReptStatBlock => {}
                    ReptVarDecl => {}
                    ReptVariable => {}
                    RightRecArithExpr => {}
                    RightRecTerm => {}
                    Sign => {}
                    StatBlock => {}
                    Statement => {}
                    StatementAmb1 => {}
                    StatementAmb2 => {}
                    StatementAmb3 => {}
                    Term => {}
                    Type => {}
                    VarDecl => {}
                    Variable => {}
                    VariableAmb1 => {}
                    Visibility => {}
                }
            }
            EPSILON => { panic!() }
            START => { todo!() }
            STOP => { panic!() }
        }
        todo!()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct GrammarRule
{
    pub lhs: GrammarSymbol,
    pub rhs: Vec<GrammarSymbol>
}

impl ToString for GrammarRule
{
    fn to_string(&self) -> String {
        let mut ret = format!("{:?} ->", self.lhs);
        for symb in &self.rhs
        {
            ret.push_str(&format!(" {:?}", symb));
        }

        ret
    }
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
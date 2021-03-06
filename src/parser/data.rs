use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::parser::grammar::GrammarSymbol;
use crate::parser::grammar::GrammarSymbol::*;
use crate::parser::grammar::GrammarRule;
use crate::parser::grammar::NamedSymbol::*;
use crate::lexer::token::TokenType::*;


pub const START_FIRST: &'static [GrammarSymbol] = &[Terminal(Class),Terminal(Func),Terminal(Main)];
pub const START_FOLLOW: &'static [GrammarSymbol] = &[STOP];
pub const ADDOP_FIRST: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Or)];
pub const ADDOP_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Id),Terminal(IntegerLit),Terminal(FloatLit),Terminal(StringLit),Terminal(OpenParen),Terminal(Bang),Terminal(Question)];
pub const ARITHEXPR_FIRST: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Id),Terminal(IntegerLit),Terminal(FloatLit),Terminal(StringLit),Terminal(OpenParen),Terminal(Bang),Terminal(Question)];
pub const ARITHEXPR_FOLLOW: &'static [GrammarSymbol] = &[Terminal(OpenSquare),Terminal(SemiColon),Terminal(CloseParen),Terminal(Colon),Terminal(Assignment),Terminal(NotEq),Terminal(LessThan),Terminal(GreaterThan),Terminal(LessEqualThan),Terminal(GreaterEqualThan),Terminal(Comma)];
pub const ARRAYSIZE_FIRST: &'static [GrammarSymbol] = &[Terminal(OpenSquare)];
pub const ARRAYSIZE_FOLLOW: &'static [GrammarSymbol] = &[Terminal(OpenSquare),Terminal(SemiColon),Terminal(CloseParen),Terminal(Comma)];
pub const ARRAYSIZEAMB1_FIRST: &'static [GrammarSymbol] = &[Terminal(IntegerLit),Terminal(OpenSquare)];
pub const ARRAYSIZEAMB1_FOLLOW: &'static [GrammarSymbol] = &[Terminal(OpenSquare),Terminal(SemiColon),Terminal(CloseParen),Terminal(Comma)];
pub const ASSIGNOP_FIRST: &'static [GrammarSymbol] = &[Terminal(EqEq)];
pub const ASSIGNOP_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Id),Terminal(IntegerLit),Terminal(FloatLit),Terminal(StringLit),Terminal(OpenParen),Terminal(Bang),Terminal(Question)];
pub const CLASSDECL_FIRST: &'static [GrammarSymbol] = &[Terminal(Class)];
pub const CLASSDECL_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Class),Terminal(Func),Terminal(Main)];
pub const EXPR_FIRST: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Id),Terminal(IntegerLit),Terminal(FloatLit),Terminal(StringLit),Terminal(OpenParen),Terminal(Bang),Terminal(Question)];
pub const EXPR_FOLLOW: &'static [GrammarSymbol] = &[Terminal(OpenSquare),Terminal(SemiColon),Terminal(CloseParen),Terminal(Colon),Terminal(Comma)];
pub const EXPRAMB1_FIRST: &'static [GrammarSymbol] = &[Terminal(Assignment),Terminal(NotEq),Terminal(LessThan),Terminal(GreaterThan),Terminal(LessEqualThan),Terminal(GreaterEqualThan)];
pub const EXPRAMB1_FOLLOW: &'static [GrammarSymbol] = &[Terminal(OpenSquare),Terminal(SemiColon),Terminal(CloseParen),Terminal(Colon),Terminal(Comma)];
pub const FACTOR_FIRST: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Id),Terminal(IntegerLit),Terminal(FloatLit),Terminal(StringLit),Terminal(OpenParen),Terminal(Bang),Terminal(Question)];
pub const FACTOR_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Or),Terminal(OpenSquare),Terminal(SemiColon),Terminal(CloseParen),Terminal(Colon),Terminal(Mult),Terminal(Div),Terminal(And),Terminal(Assignment),Terminal(NotEq),Terminal(LessThan),Terminal(GreaterThan),Terminal(LessEqualThan),Terminal(GreaterEqualThan),Terminal(Comma)];
pub const FACTORAMB1_FIRST: &'static [GrammarSymbol] = &[Terminal(OpenSquare),Terminal(OpenParen),Terminal(Period)];
pub const FACTORAMB1_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Or),Terminal(OpenSquare),Terminal(SemiColon),Terminal(CloseParen),Terminal(Colon),Terminal(Mult),Terminal(Div),Terminal(And),Terminal(Assignment),Terminal(NotEq),Terminal(LessThan),Terminal(GreaterThan),Terminal(LessEqualThan),Terminal(GreaterEqualThan),Terminal(Comma)];
pub const FACTORAMB2_FIRST: &'static [GrammarSymbol] = &[Terminal(Period)];
pub const FACTORAMB2_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Or),Terminal(OpenSquare),Terminal(SemiColon),Terminal(CloseParen),Terminal(Colon),Terminal(Mult),Terminal(Div),Terminal(And),Terminal(Assignment),Terminal(NotEq),Terminal(LessThan),Terminal(GreaterThan),Terminal(LessEqualThan),Terminal(GreaterEqualThan),Terminal(Comma)];
pub const FUNCBODY_FIRST: &'static [GrammarSymbol] = &[Terminal(OpenCurly)];
pub const FUNCBODY_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Func),Terminal(Main),STOP];
pub const FUNCDECL_FIRST: &'static [GrammarSymbol] = &[Terminal(Func)];
pub const FUNCDECL_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(CloseCurly),Terminal(Func),Terminal(IntegerType),Terminal(FloatType),Terminal(StringType),Terminal(Public),Terminal(Private)];
pub const FUNCDECLAMB1_FIRST: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(Void),Terminal(IntegerType),Terminal(FloatType),Terminal(StringType)];
pub const FUNCDECLAMB1_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(CloseCurly),Terminal(Func),Terminal(IntegerType),Terminal(FloatType),Terminal(StringType),Terminal(Public),Terminal(Private)];
pub const FUNCDEF_FIRST: &'static [GrammarSymbol] = &[Terminal(Func)];
pub const FUNCDEF_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Func),Terminal(Main)];
pub const FUNCHEAD_FIRST: &'static [GrammarSymbol] = &[Terminal(Func)];
pub const FUNCHEAD_FOLLOW: &'static [GrammarSymbol] = &[Terminal(OpenCurly)];
pub const FUNCHEADAMB1_FIRST: &'static [GrammarSymbol] = &[Terminal(OpenParen),Terminal(DoubleColon)];
pub const FUNCHEADAMB1_FOLLOW: &'static [GrammarSymbol] = &[Terminal(OpenCurly)];
pub const FUNCHEADAMB2_FIRST: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(Void),Terminal(IntegerType),Terminal(FloatType),Terminal(StringType)];
pub const FUNCHEADAMB2_FOLLOW: &'static [GrammarSymbol] = &[Terminal(OpenCurly)];
pub const FUNCPARAMS_FIRST: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(IntegerType),Terminal(FloatType),Terminal(StringType)];
pub const FUNCPARAMS_FOLLOW: &'static [GrammarSymbol] = &[Terminal(CloseParen)];
pub const INDICE_FIRST: &'static [GrammarSymbol] = &[Terminal(OpenSquare)];
pub const INDICE_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Or),Terminal(OpenSquare),Terminal(OpenSquare),Terminal(EqEq),Terminal(SemiColon),Terminal(CloseParen),Terminal(Colon),Terminal(Period),Terminal(Mult),Terminal(Div),Terminal(And),Terminal(Assignment),Terminal(NotEq),Terminal(LessThan),Terminal(GreaterThan),Terminal(LessEqualThan),Terminal(GreaterEqualThan),Terminal(Comma)];
pub const MEMBERDECL_FIRST: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(Func),Terminal(IntegerType),Terminal(FloatType),Terminal(StringType)];
pub const MEMBERDECL_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(CloseCurly),Terminal(Func),Terminal(IntegerType),Terminal(FloatType),Terminal(StringType),Terminal(Public),Terminal(Private)];
pub const MULTOP_FIRST: &'static [GrammarSymbol] = &[Terminal(Mult),Terminal(Div),Terminal(And)];
pub const MULTOP_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Id),Terminal(IntegerLit),Terminal(FloatLit),Terminal(StringLit),Terminal(OpenParen),Terminal(Bang),Terminal(Question)];
pub const OPTCLASSDECL_FIRST: &'static [GrammarSymbol] = &[Terminal(Inherits)];
pub const OPTCLASSDECL_FOLLOW: &'static [GrammarSymbol] = &[Terminal(OpenCurly)];
pub const OPTFUNCBODY_FIRST: &'static [GrammarSymbol] = &[Terminal(Var)];
pub const OPTFUNCBODY_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(CloseCurly),Terminal(If),Terminal(While),Terminal(Read),Terminal(Write),Terminal(Return),Terminal(Break),Terminal(Continue)];
pub const PARAMS_FIRST: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Id),Terminal(IntegerLit),Terminal(FloatLit),Terminal(StringLit),Terminal(OpenParen),Terminal(Bang),Terminal(Question)];
pub const PARAMS_FOLLOW: &'static [GrammarSymbol] = &[Terminal(CloseParen)];
pub const PROG_FIRST: &'static [GrammarSymbol] = &[Terminal(Class),Terminal(Func),Terminal(Main)];
pub const PROG_FOLLOW: &'static [GrammarSymbol] = &[STOP];
pub const RELEXPR_FIRST: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Id),Terminal(IntegerLit),Terminal(FloatLit),Terminal(StringLit),Terminal(OpenParen),Terminal(Bang),Terminal(Question)];
pub const RELEXPR_FOLLOW: &'static [GrammarSymbol] = &[Terminal(CloseParen)];
pub const RELOP_FIRST: &'static [GrammarSymbol] = &[Terminal(Assignment),Terminal(NotEq),Terminal(LessThan),Terminal(GreaterThan),Terminal(LessEqualThan),Terminal(GreaterEqualThan)];
pub const RELOP_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Id),Terminal(IntegerLit),Terminal(FloatLit),Terminal(StringLit),Terminal(OpenParen),Terminal(Bang),Terminal(Question)];
pub const REPTCLASSDECL_FIRST: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(Func),Terminal(IntegerType),Terminal(FloatType),Terminal(StringType),Terminal(Public),Terminal(Private)];
pub const REPTCLASSDECL_FOLLOW: &'static [GrammarSymbol] = &[Terminal(CloseCurly)];
pub const REPTFUNCBODY_FIRST: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(If),Terminal(While),Terminal(Read),Terminal(Write),Terminal(Return),Terminal(Break),Terminal(Continue)];
pub const REPTFUNCBODY_FOLLOW: &'static [GrammarSymbol] = &[Terminal(CloseCurly)];
pub const REPTFUNCPARAMS0_FIRST: &'static [GrammarSymbol] = &[Terminal(OpenSquare)];
pub const REPTFUNCPARAMS0_FOLLOW: &'static [GrammarSymbol] = &[Terminal(CloseParen),Terminal(Comma)];
pub const REPTFUNCPARAMS1_FIRST: &'static [GrammarSymbol] = &[Terminal(Comma)];
pub const REPTFUNCPARAMS1_FOLLOW: &'static [GrammarSymbol] = &[Terminal(CloseParen)];
pub const REPTFUNCPARAMSTAIL_FIRST: &'static [GrammarSymbol] = &[Terminal(OpenSquare)];
pub const REPTFUNCPARAMSTAIL_FOLLOW: &'static [GrammarSymbol] = &[Terminal(CloseParen),Terminal(Comma)];
pub const REPTOPTCLASSDECL_FIRST: &'static [GrammarSymbol] = &[Terminal(Comma)];
pub const REPTOPTCLASSDECL_FOLLOW: &'static [GrammarSymbol] = &[Terminal(OpenCurly)];
pub const REPTOPTFUNCBODY_FIRST: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(IntegerType),Terminal(FloatType),Terminal(StringType)];
pub const REPTOPTFUNCBODY_FOLLOW: &'static [GrammarSymbol] = &[Terminal(CloseCurly)];
pub const REPTPARAMS_FIRST: &'static [GrammarSymbol] = &[Terminal(Comma)];
pub const REPTPARAMS_FOLLOW: &'static [GrammarSymbol] = &[Terminal(CloseParen)];
pub const REPTPROG0_FIRST: &'static [GrammarSymbol] = &[Terminal(Class)];
pub const REPTPROG0_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Func),Terminal(Main)];
pub const REPTPROG1_FIRST: &'static [GrammarSymbol] = &[Terminal(Func)];
pub const REPTPROG1_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Main)];
pub const REPTSTATBLOCK_FIRST: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(If),Terminal(While),Terminal(Read),Terminal(Write),Terminal(Return),Terminal(Break),Terminal(Continue)];
pub const REPTSTATBLOCK_FOLLOW: &'static [GrammarSymbol] = &[Terminal(CloseCurly)];
pub const REPTVARDECL_FIRST: &'static [GrammarSymbol] = &[Terminal(OpenSquare)];
pub const REPTVARDECL_FOLLOW: &'static [GrammarSymbol] = &[Terminal(SemiColon)];
pub const REPTVARIABLE_FIRST: &'static [GrammarSymbol] = &[Terminal(OpenSquare)];
pub const REPTVARIABLE_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Or),Terminal(OpenSquare),Terminal(EqEq),Terminal(SemiColon),Terminal(CloseParen),Terminal(Colon),Terminal(Period),Terminal(Mult),Terminal(Div),Terminal(And),Terminal(Assignment),Terminal(NotEq),Terminal(LessThan),Terminal(GreaterThan),Terminal(LessEqualThan),Terminal(GreaterEqualThan),Terminal(Comma)];
pub const RIGHTRECARITHEXPR_FIRST: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Or)];
pub const RIGHTRECARITHEXPR_FOLLOW: &'static [GrammarSymbol] = &[Terminal(OpenSquare),Terminal(SemiColon),Terminal(CloseParen),Terminal(Colon),Terminal(Assignment),Terminal(NotEq),Terminal(LessThan),Terminal(GreaterThan),Terminal(LessEqualThan),Terminal(GreaterEqualThan),Terminal(Comma)];
pub const RIGHTRECTERM_FIRST: &'static [GrammarSymbol] = &[Terminal(Mult),Terminal(Div),Terminal(And)];
pub const RIGHTRECTERM_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Or),Terminal(OpenSquare),Terminal(SemiColon),Terminal(CloseParen),Terminal(Colon),Terminal(Assignment),Terminal(NotEq),Terminal(LessThan),Terminal(GreaterThan),Terminal(LessEqualThan),Terminal(GreaterEqualThan),Terminal(Comma)];
pub const SIGN_FIRST: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus)];
pub const SIGN_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Id),Terminal(IntegerLit),Terminal(FloatLit),Terminal(StringLit),Terminal(OpenParen),Terminal(Bang),Terminal(Question)];
pub const STATBLOCK_FIRST: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(OpenCurly),Terminal(If),Terminal(While),Terminal(Read),Terminal(Write),Terminal(Return),Terminal(Break),Terminal(Continue)];
pub const STATBLOCK_FOLLOW: &'static [GrammarSymbol] = &[Terminal(SemiColon),Terminal(Else)];
pub const STATEMENT_FIRST: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(If),Terminal(While),Terminal(Read),Terminal(Write),Terminal(Return),Terminal(Break),Terminal(Continue)];
pub const STATEMENT_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(CloseCurly),Terminal(SemiColon),Terminal(If),Terminal(Else),Terminal(While),Terminal(Read),Terminal(Write),Terminal(Return),Terminal(Break),Terminal(Continue)];
pub const STATEMENTAMB1_FIRST: &'static [GrammarSymbol] = &[Terminal(OpenSquare),Terminal(EqEq),Terminal(OpenParen)];
pub const STATEMENTAMB1_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(CloseCurly),Terminal(SemiColon),Terminal(If),Terminal(Else),Terminal(While),Terminal(Read),Terminal(Write),Terminal(Return),Terminal(Break),Terminal(Continue)];
pub const STATEMENTAMB2_FIRST: &'static [GrammarSymbol] = &[Terminal(EqEq),Terminal(Period)];
pub const STATEMENTAMB2_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(CloseCurly),Terminal(SemiColon),Terminal(If),Terminal(Else),Terminal(While),Terminal(Read),Terminal(Write),Terminal(Return),Terminal(Break),Terminal(Continue)];
pub const STATEMENTAMB3_FIRST: &'static [GrammarSymbol] = &[Terminal(SemiColon),Terminal(Period)];
pub const STATEMENTAMB3_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(CloseCurly),Terminal(SemiColon),Terminal(If),Terminal(Else),Terminal(While),Terminal(Read),Terminal(Write),Terminal(Return),Terminal(Break),Terminal(Continue)];
pub const TERM_FIRST: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Id),Terminal(IntegerLit),Terminal(FloatLit),Terminal(StringLit),Terminal(OpenParen),Terminal(Bang),Terminal(Question)];
pub const TERM_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Plus),Terminal(Minus),Terminal(Or),Terminal(OpenSquare),Terminal(SemiColon),Terminal(CloseParen),Terminal(Colon),Terminal(Assignment),Terminal(NotEq),Terminal(LessThan),Terminal(GreaterThan),Terminal(LessEqualThan),Terminal(GreaterEqualThan),Terminal(Comma)];
pub const TYPE_FIRST: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(IntegerType),Terminal(FloatType),Terminal(StringType)];
pub const TYPE_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(OpenCurly),Terminal(SemiColon)];
pub const VARDECL_FIRST: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(IntegerType),Terminal(FloatType),Terminal(StringType)];
pub const VARDECL_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(CloseCurly),Terminal(Func),Terminal(IntegerType),Terminal(FloatType),Terminal(StringType),Terminal(Public),Terminal(Private)];
pub const VARIABLE_FIRST: &'static [GrammarSymbol] = &[Terminal(Id)];
pub const VARIABLE_FOLLOW: &'static [GrammarSymbol] = &[Terminal(CloseParen)];
pub const VARIABLEAMB1_FIRST: &'static [GrammarSymbol] = &[Terminal(OpenSquare),Terminal(OpenParen),Terminal(Period)];
pub const VARIABLEAMB1_FOLLOW: &'static [GrammarSymbol] = &[Terminal(CloseParen)];
pub const VISIBILITY_FIRST: &'static [GrammarSymbol] = &[Terminal(Public),Terminal(Private)];
pub const VISIBILITY_FOLLOW: &'static [GrammarSymbol] = &[Terminal(Id),Terminal(Func),Terminal(IntegerType),Terminal(FloatType),Terminal(StringType)];

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
        table.insert((NonTerminal(ArraySizeAmb1), Terminal(IntegerLit)), GrammarRule{lhs: NonTerminal(ArraySizeAmb1), rhs: vec![Terminal(IntegerLit), Terminal(CloseSquare)]});

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

        table.insert((NonTerminal(ReptParams), Terminal(CloseParen)), GrammarRule{lhs: NonTerminal(ReptParams), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptParams), Terminal(Comma)), GrammarRule{lhs: NonTerminal(ReptParams), rhs: vec![Terminal(Comma), NonTerminal(Expr), NonTerminal(ReptParams)]});

        table.insert((NonTerminal(ReptProg0), Terminal(Main)), GrammarRule{lhs: NonTerminal(ReptProg0), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptProg0), Terminal(Func)), GrammarRule{lhs: NonTerminal(ReptProg0), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptProg0), Terminal(Class)), GrammarRule{lhs: NonTerminal(ReptProg0), rhs: vec![NonTerminal(ClassDecl), NonTerminal(ReptProg0)]});

        table.insert((NonTerminal(ReptProg1), Terminal(Main)), GrammarRule{lhs: NonTerminal(ReptProg1), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptProg1), Terminal(Func)), GrammarRule{lhs: NonTerminal(ReptProg1), rhs: vec![NonTerminal(FuncDef), NonTerminal(ReptProg1)]});

        table.insert((NonTerminal(ReptStatBlock), Terminal(Id)), GrammarRule{lhs: NonTerminal(ReptStatBlock), rhs: vec![NonTerminal(Statement), NonTerminal(ReptStatBlock)]});
        table.insert((NonTerminal(ReptStatBlock), Terminal(Continue)), GrammarRule{lhs: NonTerminal(ReptStatBlock), rhs: vec![NonTerminal(Statement), NonTerminal(ReptStatBlock)]});
        table.insert((NonTerminal(ReptStatBlock), Terminal(Break)), GrammarRule{lhs: NonTerminal(ReptStatBlock), rhs: vec![NonTerminal(Statement), NonTerminal(ReptStatBlock)]});
        table.insert((NonTerminal(ReptStatBlock), Terminal(Return)), GrammarRule{lhs: NonTerminal(ReptStatBlock), rhs: vec![NonTerminal(Statement), NonTerminal(ReptStatBlock)]});
        table.insert((NonTerminal(ReptStatBlock), Terminal(Write)), GrammarRule{lhs: NonTerminal(ReptStatBlock), rhs: vec![NonTerminal(Statement), NonTerminal(ReptStatBlock)]});
        table.insert((NonTerminal(ReptStatBlock), Terminal(Read)), GrammarRule{lhs: NonTerminal(ReptStatBlock), rhs: vec![NonTerminal(Statement), NonTerminal(ReptStatBlock)]});
        table.insert((NonTerminal(ReptStatBlock), Terminal(While)), GrammarRule{lhs: NonTerminal(ReptStatBlock), rhs: vec![NonTerminal(Statement), NonTerminal(ReptStatBlock)]});
        table.insert((NonTerminal(ReptStatBlock), Terminal(If)), GrammarRule{lhs: NonTerminal(ReptStatBlock), rhs: vec![NonTerminal(Statement), NonTerminal(ReptStatBlock)]});
        table.insert((NonTerminal(ReptStatBlock), Terminal(CloseCurly)), GrammarRule{lhs: NonTerminal(ReptStatBlock), rhs: vec![EPSILON]});

        table.insert((NonTerminal(ReptVarDecl), Terminal(SemiColon)), GrammarRule{lhs: NonTerminal(ReptVarDecl), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVarDecl), Terminal(OpenSquare)), GrammarRule{lhs: NonTerminal(ReptVarDecl), rhs: vec![NonTerminal(ArraySize), NonTerminal(ReptVarDecl)]});

        table.insert((NonTerminal(ReptVariable), Terminal(Period)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(CloseParen)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(SemiColon)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(Minus)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(Plus)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(Comma)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(GreaterEqualThan)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(LessEqualThan)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(GreaterThan)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(LessThan)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(NotEq)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(EqEq)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(And)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(Div)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(Mult)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(CloseSquare)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(OpenSquare)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![NonTerminal(Indice), NonTerminal(ReptVariable)]});
        table.insert((NonTerminal(ReptVariable), Terminal(Colon)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(Assignment)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});
        table.insert((NonTerminal(ReptVariable), Terminal(Or)), GrammarRule{lhs: NonTerminal(ReptVariable), rhs: vec![EPSILON]});

        table.insert((NonTerminal(RightRecArithExpr), Terminal(CloseParen)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecArithExpr), Terminal(SemiColon)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecArithExpr), Terminal(Minus)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![NonTerminal(AddOp), NonTerminal(Term), NonTerminal(RightRecArithExpr)]});
        table.insert((NonTerminal(RightRecArithExpr), Terminal(Plus)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![NonTerminal(AddOp), NonTerminal(Term), NonTerminal(RightRecArithExpr)]});
        table.insert((NonTerminal(RightRecArithExpr), Terminal(Comma)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecArithExpr), Terminal(GreaterEqualThan)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecArithExpr), Terminal(LessEqualThan)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecArithExpr), Terminal(GreaterThan)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecArithExpr), Terminal(LessThan)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecArithExpr), Terminal(NotEq)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecArithExpr), Terminal(EqEq)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecArithExpr), Terminal(CloseSquare)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecArithExpr), Terminal(Colon)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecArithExpr), Terminal(Or)), GrammarRule{lhs: NonTerminal(RightRecArithExpr), rhs: vec![NonTerminal(AddOp), NonTerminal(Term), NonTerminal(RightRecArithExpr)]});

        table.insert((NonTerminal(RightRecTerm), Terminal(CloseParen)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecTerm), Terminal(SemiColon)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecTerm), Terminal(Minus)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecTerm), Terminal(Plus)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecTerm), Terminal(Comma)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecTerm), Terminal(GreaterEqualThan)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecTerm), Terminal(LessEqualThan)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecTerm), Terminal(GreaterThan)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecTerm), Terminal(LessThan)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecTerm), Terminal(NotEq)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecTerm), Terminal(EqEq)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecTerm), Terminal(And)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![NonTerminal(MultOp), NonTerminal(Factor), NonTerminal(RightRecTerm)]});
        table.insert((NonTerminal(RightRecTerm), Terminal(Div)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![NonTerminal(MultOp), NonTerminal(Factor), NonTerminal(RightRecTerm)]});
        table.insert((NonTerminal(RightRecTerm), Terminal(Mult)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![NonTerminal(MultOp), NonTerminal(Factor), NonTerminal(RightRecTerm)]});
        table.insert((NonTerminal(RightRecTerm), Terminal(CloseSquare)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecTerm), Terminal(Colon)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});
        table.insert((NonTerminal(RightRecTerm), Terminal(Or)), GrammarRule{lhs: NonTerminal(RightRecTerm), rhs: vec![EPSILON]});

        table.insert((NonTerminal(Sign), Terminal(Minus)), GrammarRule{lhs: NonTerminal(Sign), rhs: vec![Terminal(Minus)]});
        table.insert((NonTerminal(RightRecTerm), Terminal(Plus)), GrammarRule{lhs: NonTerminal(Sign), rhs: vec![Terminal(Plus)]});

        table.insert((NonTerminal(StatBlock), Terminal(Id)), GrammarRule{lhs: NonTerminal(StatBlock), rhs: vec![NonTerminal(Statement)]});
        table.insert((NonTerminal(StatBlock), Terminal(SemiColon)), GrammarRule{lhs: NonTerminal(StatBlock), rhs: vec![EPSILON]});
        table.insert((NonTerminal(StatBlock), Terminal(Continue)), GrammarRule{lhs: NonTerminal(StatBlock), rhs: vec![NonTerminal(Statement)]});
        table.insert((NonTerminal(StatBlock), Terminal(Break)), GrammarRule{lhs: NonTerminal(StatBlock), rhs: vec![NonTerminal(Statement)]});
        table.insert((NonTerminal(StatBlock), Terminal(Return)), GrammarRule{lhs: NonTerminal(StatBlock), rhs: vec![NonTerminal(Statement)]});
        table.insert((NonTerminal(StatBlock), Terminal(Write)), GrammarRule{lhs: NonTerminal(StatBlock), rhs: vec![NonTerminal(Statement)]});
        table.insert((NonTerminal(StatBlock), Terminal(Read)), GrammarRule{lhs: NonTerminal(StatBlock), rhs: vec![NonTerminal(Statement)]});
        table.insert((NonTerminal(StatBlock), Terminal(While)), GrammarRule{lhs: NonTerminal(StatBlock), rhs: vec![NonTerminal(Statement)]});
        table.insert((NonTerminal(StatBlock), Terminal(Else)), GrammarRule{lhs: NonTerminal(StatBlock), rhs: vec![EPSILON]});
        table.insert((NonTerminal(StatBlock), Terminal(If)), GrammarRule{lhs: NonTerminal(StatBlock), rhs: vec![NonTerminal(Statement)]});
        table.insert((NonTerminal(StatBlock), Terminal(OpenCurly)), GrammarRule{lhs: NonTerminal(StatBlock), rhs: vec![Terminal(OpenCurly), NonTerminal(ReptStatBlock), Terminal(CloseCurly)]});

        table.insert((NonTerminal(Statement), Terminal(Id)), GrammarRule{lhs: NonTerminal(Statement), rhs: vec![Terminal(Id), NonTerminal(StatementAmb1)]});
        table.insert((NonTerminal(Statement), Terminal(Continue)), GrammarRule{lhs: NonTerminal(Statement), rhs: vec![Terminal(Continue), Terminal(SemiColon)]});
        table.insert((NonTerminal(Statement), Terminal(Break)), GrammarRule{lhs: NonTerminal(Statement), rhs: vec![Terminal(Break), Terminal(SemiColon)]});
        table.insert((NonTerminal(Statement), Terminal(Return)), GrammarRule{lhs: NonTerminal(Statement), rhs: vec![Terminal(Return), Terminal(OpenParen), NonTerminal(Expr), Terminal(CloseParen), Terminal(SemiColon)]});
        table.insert((NonTerminal(Statement), Terminal(Write)), GrammarRule{lhs: NonTerminal(Statement), rhs: vec![Terminal(Write), Terminal(OpenParen), NonTerminal(Expr), Terminal(CloseParen), Terminal(SemiColon)]});
        table.insert((NonTerminal(Statement), Terminal(Read)), GrammarRule{lhs: NonTerminal(Statement), rhs: vec![Terminal(Read), Terminal(OpenParen), NonTerminal(Variable), Terminal(CloseParen), Terminal(SemiColon)]});
        table.insert((NonTerminal(Statement), Terminal(While)), GrammarRule{lhs: NonTerminal(Statement), rhs: vec![Terminal(While), Terminal(OpenParen), NonTerminal(RelExpr), Terminal(CloseParen), NonTerminal(StatBlock), Terminal(SemiColon)]});
        table.insert((NonTerminal(Statement), Terminal(If)), GrammarRule{lhs: NonTerminal(Statement), rhs: vec![Terminal(If), Terminal(OpenParen), NonTerminal(RelExpr), Terminal(CloseParen), Terminal(Then), NonTerminal(StatBlock), Terminal(Else), NonTerminal(StatBlock), Terminal(SemiColon)]});

        table.insert((NonTerminal(StatementAmb1), Terminal(OpenParen)), GrammarRule{lhs: NonTerminal(StatementAmb1), rhs: vec![Terminal(OpenParen), NonTerminal(Params), Terminal(CloseParen), NonTerminal(StatementAmb3)]});
        table.insert((NonTerminal(StatementAmb1), Terminal(OpenSquare)), GrammarRule{lhs: NonTerminal(StatementAmb1), rhs: vec![NonTerminal(Indice), NonTerminal(ReptVariable), NonTerminal(StatementAmb2)]});
        table.insert((NonTerminal(StatementAmb1), Terminal(Assignment)), GrammarRule{lhs: NonTerminal(StatementAmb1), rhs: vec![NonTerminal(AssignOp), NonTerminal(Expr), Terminal(SemiColon)]});

        table.insert((NonTerminal(StatementAmb2), Terminal(Period)), GrammarRule{lhs: NonTerminal(StatementAmb2), rhs: vec![Terminal(Period), Terminal(Id), NonTerminal(StatementAmb1)]});
        table.insert((NonTerminal(StatementAmb2), Terminal(Assignment)), GrammarRule{lhs: NonTerminal(StatementAmb2), rhs: vec![NonTerminal(AssignOp), NonTerminal(Expr), Terminal(SemiColon)]});

        table.insert((NonTerminal(StatementAmb3), Terminal(Period)), GrammarRule{lhs: NonTerminal(StatementAmb3), rhs: vec![Terminal(Period), NonTerminal(StatementAmb1)]});
        table.insert((NonTerminal(StatementAmb3), Terminal(SemiColon)), GrammarRule{lhs: NonTerminal(StatementAmb3), rhs: vec![Terminal(SemiColon)]});

        table.insert((NonTerminal(Term), Terminal(Id)), GrammarRule{lhs: NonTerminal(Term), rhs: vec![NonTerminal(Factor), NonTerminal(RightRecTerm)]});
        table.insert((NonTerminal(Term), Terminal(OpenParen)), GrammarRule{lhs: NonTerminal(Term), rhs: vec![NonTerminal(Factor), NonTerminal(RightRecTerm)]});
        table.insert((NonTerminal(Term), Terminal(Minus)), GrammarRule{lhs: NonTerminal(Term), rhs: vec![NonTerminal(Factor), NonTerminal(RightRecTerm)]});
        table.insert((NonTerminal(Term), Terminal(Plus)), GrammarRule{lhs: NonTerminal(Term), rhs: vec![NonTerminal(Factor), NonTerminal(RightRecTerm)]});
        table.insert((NonTerminal(Term), Terminal(Question)), GrammarRule{lhs: NonTerminal(Term), rhs: vec![NonTerminal(Factor), NonTerminal(RightRecTerm)]});
        table.insert((NonTerminal(Term), Terminal(Bang)), GrammarRule{lhs: NonTerminal(Term), rhs: vec![NonTerminal(Factor), NonTerminal(RightRecTerm)]});
        table.insert((NonTerminal(Term), Terminal(StringLit)), GrammarRule{lhs: NonTerminal(Term), rhs: vec![NonTerminal(Factor), NonTerminal(RightRecTerm)]});
        table.insert((NonTerminal(Term), Terminal(FloatLit)), GrammarRule{lhs: NonTerminal(Term), rhs: vec![NonTerminal(Factor), NonTerminal(RightRecTerm)]});
        table.insert((NonTerminal(Term), Terminal(IntegerLit)), GrammarRule{lhs: NonTerminal(Term), rhs: vec![NonTerminal(Factor), NonTerminal(RightRecTerm)]});

        table.insert((NonTerminal(Type), Terminal(Id)), GrammarRule{lhs: NonTerminal(Type), rhs: vec![Terminal(Id)]});
        table.insert((NonTerminal(Type), Terminal(StringType)), GrammarRule{lhs: NonTerminal(Type), rhs: vec![Terminal(StringType)]});
        table.insert((NonTerminal(Type), Terminal(FloatType)), GrammarRule{lhs: NonTerminal(Type), rhs: vec![Terminal(FloatType)]});
        table.insert((NonTerminal(Type), Terminal(IntegerType)), GrammarRule{lhs: NonTerminal(Type), rhs: vec![Terminal(IntegerType)]});

        table.insert((NonTerminal(VarDecl), Terminal(Id)), GrammarRule{lhs: NonTerminal(VarDecl), rhs: vec![NonTerminal(Type), Terminal(Id), NonTerminal(ReptVarDecl), Terminal(SemiColon)]});
        table.insert((NonTerminal(VarDecl), Terminal(StringType)), GrammarRule{lhs: NonTerminal(VarDecl), rhs: vec![NonTerminal(Type), Terminal(Id), NonTerminal(ReptVarDecl), Terminal(SemiColon)]});
        table.insert((NonTerminal(VarDecl), Terminal(FloatType)), GrammarRule{lhs: NonTerminal(VarDecl), rhs: vec![NonTerminal(Type), Terminal(Id), NonTerminal(ReptVarDecl), Terminal(SemiColon)]});
        table.insert((NonTerminal(VarDecl), Terminal(IntegerType)), GrammarRule{lhs: NonTerminal(VarDecl), rhs: vec![NonTerminal(Type), Terminal(Id), NonTerminal(ReptVarDecl), Terminal(SemiColon)]});

        table.insert((NonTerminal(Variable), Terminal(Id)), GrammarRule{lhs: NonTerminal(Variable), rhs: vec![Terminal(Id), NonTerminal(VariableAmb1)]});

        table.insert((NonTerminal(VariableAmb1), Terminal(Period)), GrammarRule{lhs: NonTerminal(VariableAmb1), rhs: vec![NonTerminal(ReptVariable), Terminal(Period), Terminal(Id), NonTerminal(VariableAmb1)]});
        table.insert((NonTerminal(VariableAmb1), Terminal(CloseParen)), GrammarRule{lhs: NonTerminal(VariableAmb1), rhs: vec![EPSILON]});
        table.insert((NonTerminal(VariableAmb1), Terminal(OpenParen)), GrammarRule{lhs: NonTerminal(VariableAmb1), rhs: vec![Terminal(OpenParen), NonTerminal(Params), Terminal(CloseParen), Terminal(Period), Terminal(Id), NonTerminal(VariableAmb1)]});
        table.insert((NonTerminal(VariableAmb1), Terminal(OpenSquare)), GrammarRule{lhs: NonTerminal(VariableAmb1), rhs: vec![NonTerminal(ReptVariable), Terminal(Period), Terminal(Id), NonTerminal(VariableAmb1)]});

        table.insert((NonTerminal(Visibility), Terminal(Private)), GrammarRule{lhs: NonTerminal(Visibility), rhs: vec![Terminal(Private)]});
        table.insert((NonTerminal(Visibility), Terminal(Public)), GrammarRule{lhs: NonTerminal(Visibility), rhs: vec![Terminal(Public)]});
        table.insert((NonTerminal(Visibility), Terminal(Id)), GrammarRule{lhs: NonTerminal(Visibility), rhs: vec![EPSILON]});
        table.insert((NonTerminal(Visibility), Terminal(StringType)), GrammarRule{lhs: NonTerminal(Visibility), rhs: vec![EPSILON]});
        table.insert((NonTerminal(Visibility), Terminal(FloatType)), GrammarRule{lhs: NonTerminal(Visibility), rhs: vec![EPSILON]});
        table.insert((NonTerminal(Visibility), Terminal(IntegerType)), GrammarRule{lhs: NonTerminal(Visibility), rhs: vec![EPSILON]});
        table.insert((NonTerminal(Visibility), Terminal(Func)), GrammarRule{lhs: NonTerminal(Visibility), rhs: vec![EPSILON]});

        table
    };
}
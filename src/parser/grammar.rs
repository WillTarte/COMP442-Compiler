use crate::lexer::token::TokenType::OpenSquare;
use crate::lexer::token::TokenType::*;
use crate::lexer::token::{Token, TokenType};
use crate::parser::data::*;
use crate::parser::grammar::GrammarSymbol::*;
use crate::parser::grammar::NamedSymbol::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum GrammarSymbol {
    Terminal(TokenType),
    NonTerminal(NamedSymbol),
    EPSILON,
    STOP,
}

impl GrammarSymbol {
    pub fn first_set(&self) -> &[GrammarSymbol] {
        match self {
            Terminal(_) => {
                panic!()
            }
            NonTerminal(symbol) => match symbol {
                Start => {
                    return START_FIRST;
                }
                AddOp => {
                    return ADDOP_FIRST;
                }
                ArithExpr => {
                    return ARITHEXPR_FIRST;
                }
                ArraySize => {
                    return ARRAYSIZE_FIRST;
                }
                ArraySizeAmb1 => {
                    return ARRAYSIZEAMB1_FIRST;
                }
                AssignOp => {
                    return ASSIGNOP_FIRST;
                }
                ClassDecl => {
                    return CLASSDECL_FIRST;
                }
                Expr => {
                    return EXPR_FIRST;
                }
                ExprAmb1 => {
                    return EXPRAMB1_FIRST;
                }
                Factor => {
                    return FACTOR_FIRST;
                }
                FactorAmb1 => {
                    return FACTORAMB1_FIRST;
                }
                FactorAmb2 => {
                    return FACTORAMB2_FIRST;
                }
                FuncBody => {
                    return FUNCBODY_FIRST;
                }
                FuncDecl => {
                    return FUNCDECL_FIRST;
                }
                FuncDeclAmb1 => {
                    return FUNCDECLAMB1_FIRST;
                }
                FuncDef => {
                    return FUNCDEF_FIRST;
                }
                FuncHead => {
                    return FUNCHEAD_FIRST;
                }
                FuncHeadAmb1 => {
                    return FUNCHEADAMB1_FIRST;
                }
                FuncHeadAmb2 => {
                    return FUNCHEADAMB2_FIRST;
                }
                FuncParams => {
                    return FUNCPARAMS_FIRST;
                }
                Indice => {
                    return INDICE_FIRST;
                }
                MemberDecl => {
                    return MEMBERDECL_FIRST;
                }
                MultOp => {
                    return MULTOP_FIRST;
                }
                OptClassDecl => {
                    return OPTCLASSDECL_FIRST;
                }
                OptFuncBody => {
                    return OPTFUNCBODY_FIRST;
                }
                Params => {
                    return PARAMS_FIRST;
                }
                Prog => {
                    return PROG_FIRST;
                }
                RelExpr => {
                    return RELEXPR_FIRST;
                }
                RelOp => {
                    return RELOP_FIRST;
                }
                ReptClassDecl => {
                    return REPTCLASSDECL_FIRST;
                }
                ReptFuncBody => {
                    return REPTFUNCBODY_FIRST;
                }
                ReptFuncParams0 => {
                    return REPTFUNCPARAMS0_FIRST;
                }
                ReptFuncParams1 => {
                    return REPTFUNCPARAMS1_FIRST;
                }
                ReptFuncParamsTail => {
                    return REPTFUNCPARAMSTAIL_FIRST;
                }
                ReptOptClassDecl => {
                    return REPTOPTCLASSDECL_FIRST;
                }
                ReptOptFuncBody => {
                    return REPTOPTFUNCBODY_FIRST;
                }
                ReptParams => {
                    return REPTPARAMS_FIRST;
                }
                ReptProg0 => {
                    return REPTPROG0_FIRST;
                }
                ReptProg1 => {
                    return REPTPROG1_FIRST;
                }
                ReptStatBlock => {
                    return REPTSTATBLOCK_FIRST;
                }
                ReptVarDecl => {
                    return REPTVARDECL_FIRST;
                }
                ReptVariable => {
                    return REPTVARIABLE_FIRST;
                }
                RightRecArithExpr => {
                    return RIGHTRECARITHEXPR_FIRST;
                }
                RightRecTerm => {
                    return RIGHTRECTERM_FIRST;
                }
                Sign => {
                    return SIGN_FIRST;
                }
                StatBlock => {
                    return STATBLOCK_FIRST;
                }
                Statement => {
                    return STATEMENT_FIRST;
                }
                StatementAmb1 => {
                    return STATEMENTAMB1_FIRST;
                }
                StatementAmb2 => {
                    return STATEMENTAMB2_FIRST;
                }
                StatementAmb3 => {
                    return STATEMENTAMB3_FIRST;
                }
                Term => {
                    return TERM_FIRST;
                }
                Type => {
                    return TYPE_FIRST;
                }
                VarDecl => {
                    return VARDECL_FIRST;
                }
                Variable => {
                    return VARIABLE_FIRST;
                }
                VariableAmb1 => {
                    return VARIABLEAMB1_FIRST;
                }
                Visibility => {
                    return VISIBILITY_FIRST;
                }
            },
            EPSILON => {
                panic!()
            }
            STOP => {
                panic!()
            }
        }
    }

    pub fn follow_set(&self) -> &[GrammarSymbol] {
        match self {
            Terminal(_) => {
                panic!()
            }
            NonTerminal(symbol) => match symbol {
                Start => {
                    return START_FOLLOW;
                }
                AddOp => {
                    return ADDOP_FOLLOW;
                }
                ArithExpr => {
                    return ARITHEXPR_FOLLOW;
                }
                ArraySize => {
                    return ARRAYSIZE_FOLLOW;
                }
                ArraySizeAmb1 => {
                    return ARRAYSIZEAMB1_FOLLOW;
                }
                AssignOp => {
                    return ASSIGNOP_FOLLOW;
                }
                ClassDecl => {
                    return CLASSDECL_FOLLOW;
                }
                Expr => {
                    return EXPR_FOLLOW;
                }
                ExprAmb1 => {
                    return EXPRAMB1_FOLLOW;
                }
                Factor => {
                    return FACTOR_FOLLOW;
                }
                FactorAmb1 => {
                    return FACTORAMB1_FOLLOW;
                }
                FactorAmb2 => {
                    return FACTORAMB2_FOLLOW;
                }
                FuncBody => {
                    return FUNCBODY_FOLLOW;
                }
                FuncDecl => {
                    return FUNCDECL_FOLLOW;
                }
                FuncDeclAmb1 => {
                    return FUNCDECLAMB1_FOLLOW;
                }
                FuncDef => {
                    return FUNCDEF_FOLLOW;
                }
                FuncHead => {
                    return FUNCHEAD_FOLLOW;
                }
                FuncHeadAmb1 => {
                    return FUNCHEADAMB1_FOLLOW;
                }
                FuncHeadAmb2 => {
                    return FUNCHEADAMB2_FOLLOW;
                }
                FuncParams => {
                    return FUNCPARAMS_FOLLOW;
                }
                Indice => {
                    return INDICE_FOLLOW;
                }
                MemberDecl => {
                    return MEMBERDECL_FOLLOW;
                }
                MultOp => {
                    return MULTOP_FOLLOW;
                }
                OptClassDecl => {
                    return OPTCLASSDECL_FOLLOW;
                }
                OptFuncBody => {
                    return OPTFUNCBODY_FOLLOW;
                }
                Params => {
                    return PARAMS_FOLLOW;
                }
                Prog => {
                    return PROG_FOLLOW;
                }
                RelExpr => {
                    return RELEXPR_FOLLOW;
                }
                RelOp => {
                    return RELOP_FOLLOW;
                }
                ReptClassDecl => {
                    return REPTCLASSDECL_FOLLOW;
                }
                ReptFuncBody => {
                    return REPTFUNCBODY_FOLLOW;
                }
                ReptFuncParams0 => {
                    return REPTFUNCPARAMS0_FOLLOW;
                }
                ReptFuncParams1 => {
                    return REPTFUNCPARAMS1_FOLLOW;
                }
                ReptFuncParamsTail => {
                    return REPTFUNCPARAMSTAIL_FOLLOW;
                }
                ReptOptClassDecl => {
                    return REPTOPTCLASSDECL_FOLLOW;
                }
                ReptOptFuncBody => {
                    return REPTOPTFUNCBODY_FOLLOW;
                }
                ReptParams => {
                    return REPTPARAMS_FOLLOW;
                }
                ReptProg0 => {
                    return REPTPROG0_FOLLOW;
                }
                ReptProg1 => {
                    return REPTPROG1_FOLLOW;
                }
                ReptStatBlock => {
                    return REPTSTATBLOCK_FOLLOW;
                }
                ReptVarDecl => {
                    return REPTVARDECL_FOLLOW;
                }
                ReptVariable => {
                    return REPTVARIABLE_FOLLOW;
                }
                RightRecArithExpr => {
                    return RIGHTRECARITHEXPR_FOLLOW;
                }
                RightRecTerm => {
                    return RIGHTRECTERM_FOLLOW;
                }
                Sign => {
                    return SIGN_FOLLOW;
                }
                StatBlock => {
                    return STATBLOCK_FOLLOW;
                }
                Statement => {
                    return STATEMENT_FOLLOW;
                }
                StatementAmb1 => {
                    return STATEMENTAMB1_FOLLOW;
                }
                StatementAmb2 => {
                    return STATEMENTAMB2_FOLLOW;
                }
                StatementAmb3 => {
                    return STATEMENTAMB3_FOLLOW;
                }
                Term => {
                    return TERM_FOLLOW;
                }
                Type => {
                    return TYPE_FOLLOW;
                }
                VarDecl => {
                    return VARDECL_FOLLOW;
                }
                Variable => {
                    return VARIABLE_FOLLOW;
                }
                VariableAmb1 => {
                    return VARIABLEAMB1_FOLLOW;
                }
                Visibility => {
                    return VISIBILITY_FOLLOW;
                }
            },
            EPSILON => {
                panic!()
            },
            STOP => {
                panic!()
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct GrammarRule {
    pub lhs: GrammarSymbol,
    pub rhs: Vec<GrammarSymbol>,
}

impl ToString for GrammarRule {
    fn to_string(&self) -> String {
        let mut ret = format!("{:?} ->", self.lhs);
        for symb in &self.rhs {
            ret.push_str(&format!(" {:?}", symb));
        }

        ret
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum NamedSymbol {
    Start,
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
    Visibility,
}
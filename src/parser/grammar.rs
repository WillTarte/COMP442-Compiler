//! Grammar elements

use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::SemanticAction;
use crate::parser::data::*;
use crate::parser::grammar::NamedSymbol::*;

/// Symbols that can be contained in our grammar
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum GrammarSymbol {
    Terminal(TokenType),
    NonTerminal(NamedSymbol),
    SemanticActionType(SemanticAction),
    EPSILON,
    STOP,
}

/// A grammar rule is composed of a lhs symbol and a list of symbols on the rhs
#[derive(Eq, PartialEq, Debug, Clone)]
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

/// A named symbol is a non terminal in the grammar
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

impl NamedSymbol {
    /// Returns the first set of the given non terminal
    pub fn first_set(&self) -> &[GrammarSymbol] {
        return match self {
            Start => START_FIRST,
            AddOp => ADDOP_FIRST,
            ArithExpr => ARITHEXPR_FIRST,
            ArraySize => ARRAYSIZE_FIRST,
            ArraySizeAmb1 => ARRAYSIZEAMB1_FIRST,
            AssignOp => ASSIGNOP_FIRST,
            ClassDecl => CLASSDECL_FIRST,
            Expr => EXPR_FIRST,
            ExprAmb1 => EXPRAMB1_FIRST,
            Factor => FACTOR_FIRST,
            FactorAmb1 => FACTORAMB1_FIRST,
            FactorAmb2 => FACTORAMB2_FIRST,
            FuncBody => FUNCBODY_FIRST,
            FuncDecl => FUNCDECL_FIRST,
            FuncDeclAmb1 => FUNCDECLAMB1_FIRST,
            FuncDef => FUNCDEF_FIRST,
            FuncHead => FUNCHEAD_FIRST,
            FuncHeadAmb1 => FUNCHEADAMB1_FIRST,
            FuncHeadAmb2 => FUNCHEADAMB2_FIRST,
            FuncParams => FUNCPARAMS_FIRST,
            Indice => INDICE_FIRST,
            MemberDecl => MEMBERDECL_FIRST,
            MultOp => MULTOP_FIRST,
            OptClassDecl => OPTCLASSDECL_FIRST,
            OptFuncBody => OPTFUNCBODY_FIRST,
            Params => PARAMS_FIRST,
            Prog => PROG_FIRST,
            RelExpr => RELEXPR_FIRST,
            RelOp => RELOP_FIRST,
            ReptClassDecl => REPTCLASSDECL_FIRST,
            ReptFuncBody => REPTFUNCBODY_FIRST,
            ReptFuncParams0 => REPTFUNCPARAMS0_FIRST,
            ReptFuncParams1 => REPTFUNCPARAMS1_FIRST,
            ReptFuncParamsTail => REPTFUNCPARAMSTAIL_FIRST,
            ReptOptClassDecl => REPTOPTCLASSDECL_FIRST,
            ReptOptFuncBody => REPTOPTFUNCBODY_FIRST,
            ReptParams => REPTPARAMS_FIRST,
            ReptProg0 => REPTPROG0_FIRST,
            ReptProg1 => REPTPROG1_FIRST,
            ReptStatBlock => REPTSTATBLOCK_FIRST,
            ReptVarDecl => REPTVARDECL_FIRST,
            ReptVariable => REPTVARIABLE_FIRST,
            RightRecArithExpr => RIGHTRECARITHEXPR_FIRST,
            RightRecTerm => RIGHTRECTERM_FIRST,
            Sign => SIGN_FIRST,
            StatBlock => STATBLOCK_FIRST,
            Statement => STATEMENT_FIRST,
            StatementAmb1 => STATEMENTAMB1_FIRST,
            StatementAmb2 => STATEMENTAMB2_FIRST,
            StatementAmb3 => STATEMENTAMB3_FIRST,
            Term => TERM_FIRST,
            Type => TYPE_FIRST,
            VarDecl => VARDECL_FIRST,
            Variable => VARIABLE_FIRST,
            VariableAmb1 => VARIABLEAMB1_FIRST,
            Visibility => VISIBILITY_FIRST,
        };
    }
    /// Returns the follow set of the given non terminal
    pub fn follow_set(&self) -> &[GrammarSymbol] {
        return match self {
            Start => START_FOLLOW,
            AddOp => ADDOP_FOLLOW,
            ArithExpr => ARITHEXPR_FOLLOW,
            ArraySize => ARRAYSIZE_FOLLOW,
            ArraySizeAmb1 => ARRAYSIZEAMB1_FOLLOW,
            AssignOp => ASSIGNOP_FOLLOW,
            ClassDecl => CLASSDECL_FOLLOW,
            Expr => EXPR_FOLLOW,
            ExprAmb1 => EXPRAMB1_FOLLOW,
            Factor => FACTOR_FOLLOW,
            FactorAmb1 => FACTORAMB1_FOLLOW,
            FactorAmb2 => FACTORAMB2_FOLLOW,
            FuncBody => FUNCBODY_FOLLOW,
            FuncDecl => FUNCDECL_FOLLOW,
            FuncDeclAmb1 => FUNCDECLAMB1_FOLLOW,
            FuncDef => FUNCDEF_FOLLOW,
            FuncHead => FUNCHEAD_FOLLOW,
            FuncHeadAmb1 => FUNCHEADAMB1_FOLLOW,
            FuncHeadAmb2 => FUNCHEADAMB2_FOLLOW,
            FuncParams => FUNCPARAMS_FOLLOW,
            Indice => INDICE_FOLLOW,
            MemberDecl => MEMBERDECL_FOLLOW,
            MultOp => MULTOP_FOLLOW,
            OptClassDecl => OPTCLASSDECL_FOLLOW,
            OptFuncBody => OPTFUNCBODY_FOLLOW,
            Params => PARAMS_FOLLOW,
            Prog => PROG_FOLLOW,
            RelExpr => RELEXPR_FOLLOW,
            RelOp => RELOP_FOLLOW,
            ReptClassDecl => REPTCLASSDECL_FOLLOW,
            ReptFuncBody => REPTFUNCBODY_FOLLOW,
            ReptFuncParams0 => REPTFUNCPARAMS0_FOLLOW,
            ReptFuncParams1 => REPTFUNCPARAMS1_FOLLOW,
            ReptFuncParamsTail => REPTFUNCPARAMSTAIL_FOLLOW,
            ReptOptClassDecl => REPTOPTCLASSDECL_FOLLOW,
            ReptOptFuncBody => REPTOPTFUNCBODY_FOLLOW,
            ReptParams => REPTPARAMS_FOLLOW,
            ReptProg0 => REPTPROG0_FOLLOW,
            ReptProg1 => REPTPROG1_FOLLOW,
            ReptStatBlock => REPTSTATBLOCK_FOLLOW,
            ReptVarDecl => REPTVARDECL_FOLLOW,
            ReptVariable => REPTVARIABLE_FOLLOW,
            RightRecArithExpr => RIGHTRECARITHEXPR_FOLLOW,
            RightRecTerm => RIGHTRECTERM_FOLLOW,
            Sign => SIGN_FOLLOW,
            StatBlock => STATBLOCK_FOLLOW,
            Statement => STATEMENT_FOLLOW,
            StatementAmb1 => STATEMENTAMB1_FOLLOW,
            StatementAmb2 => STATEMENTAMB2_FOLLOW,
            StatementAmb3 => STATEMENTAMB3_FOLLOW,
            Term => TERM_FOLLOW,
            Type => TYPE_FOLLOW,
            VarDecl => VARDECL_FOLLOW,
            Variable => VARIABLE_FOLLOW,
            VariableAmb1 => VARIABLEAMB1_FOLLOW,
            Visibility => VISIBILITY_FOLLOW,
        };
    }
}

/// A Derivation table keeps track of the derivation steps
#[derive(Debug)]
pub struct DerivationTable(pub(crate) Vec<DerivationRecord>);

impl DerivationTable {
    /// Adds a [DerivationRecord] to the table
    pub fn add_record(&mut self, record: DerivationRecord) {
        self.0.push(record);
    }
    /// Creates a new empty DerivationTable
    pub fn new() -> Self {
        DerivationTable(Vec::new())
    }
}

/// A row in the [DerivationTable]
#[derive(Debug)]
pub struct DerivationRecord {
    pub stack_state: Vec<GrammarSymbol>,
    pub lookahead_token: Option<Token>,
    pub derived_rule: Option<GrammarRule>,
}

impl DerivationRecord {
    pub fn new(
        stack_state: &Vec<GrammarSymbol>,
        lookahead_token: &Option<Token>,
        derived_rule: Option<&GrammarRule>,
    ) -> Self {
        Self {
            stack_state: stack_state.clone(),
            lookahead_token: lookahead_token.clone(),
            derived_rule: derived_rule.cloned(),
        }
    }
}

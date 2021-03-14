use log::{warn, debug};
use crate::lexer::token::{Token};

#[derive(Clone, Debug)]
pub struct Node
{
    val: Option<NodeVal>,
    children: Vec<Node>
}

impl Node
{
    pub fn new_with_val(val: NodeVal) -> Self
    {
        Self
        {
            val: Some(val),
            children: Vec::new()
        }
    }

    pub fn new_empty() -> Self
    {
        Self
        {
            val: None,
            children: Vec::new()
        }
    }

    pub fn add_child(&mut self, child: Node)
    {
        self.children.push(child);
    }
}

pub struct SemanticStack(Vec<Node>);

impl SemanticStack
{
    pub fn new() -> Self
    {
        Self(Vec::new())
    }

    pub fn make_family_root(&mut self, node_t: InternalNodeType)
    {
        debug!("Making family with {:?}", node_t);
        self.0.push(Node::new_with_val(NodeVal::Internal(node_t)));
    }

    pub fn make_terminal_node(&mut self, token: &Token)
    {
        debug!("Making terminal node with: {:?}", token);
        self.0.push(Node::new_with_val(NodeVal::Leaf(token.clone())));
    }

    pub fn make_relative_operation(&mut self)
    {
        let rhs = self.0.pop();
        let op = self.0.pop();
        let lhs = self.0.pop();

        if rhs.is_none() || op.is_none() || lhs.is_none()
        {
            warn!("Failed to make relative operation node: {:?} {:?} {:?}", lhs, op, rhs);
            return;
        }

        let lhs = lhs.unwrap();
        let mut op = op.unwrap();
        let rhs = rhs.unwrap();

        debug!("Making relative operation node: {:?} {:?} {:?}", lhs.val, op.val, rhs.val);

        op.add_child(lhs);
        op.add_child(rhs);
        self.0.push(op);
    }

    pub fn make_empty_node(&mut self)
    {
        self.0.push(Node::new_empty());
        debug!("Added empty node")
    }

    pub fn add_child(&mut self)
    {
        let child = self.0.pop();
        let top = self.0.last_mut();

        if child.is_none() || top.is_none()
        {
            warn!("Failed to add child {:?} to {:?}", child, top);
            return;
        }

        let child = child.unwrap();
        let mut top = top.unwrap();

        debug!("Adding {:?} as a child of {:?}", child.val, top.val);
        top.children.push(child);

    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SemanticAction
{
    MakeFamilyRootNode(InternalNodeType),
    MakeTerminalNode,
    MakeRelativeOperation,
    MakeEmptyNode,
    AddChild
}

#[derive(Clone, Debug)]
pub enum NodeVal
{
    Leaf(Token),
    Internal(InternalNodeType)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum InternalNodeType
{
    Root,
    FuncCallParams,
    Add,
    Sub,
    Or,
    Assignment,
    ClassDeclaration,
    MemberDeclaration,
    MemberFuncDeclaration,
    MemberVarDeclaration,
    FuncDeclaration,
    VarDeclaration,
    Expr,
    ArithExpr,
    RelExpr,
    FuncParams,
    FuncParam,
    FuncParamDim,
    InheritList,
    MemberList,
    ArrayDim,
    Negation,
    SignedFactor,
    TernaryOperation,
    Factor,
    FuncBody,
    StatementList,
    FuncDef,
    Indice,
    Mult,
    Div,
    And,
    VarBlock,
    ClassDeclarations,
    FunctionDefinitions,
    Main,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterEqualThan,
    LessEqualThan,
    IfStatement,
    WhileStatement,
    ReadStatement,
    WriteStatement,
    ReturnStatement,
    BreakStatement,
    ContinueStatement,
    GenericStatement,
    Variable
}

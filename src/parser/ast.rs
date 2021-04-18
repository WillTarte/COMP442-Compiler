//! Elements related to an Abstract Syntax Tree

use crate::lexer::token::Token;
use crate::lexer::token::TokenType::StringLit;
use log::{debug, warn};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

/// A node in the abstact syntax tree.
/// Contains an optional [NodeVal] and a list of children
#[derive(Clone)]
pub struct Node {
    val: Option<NodeVal>,
    children: Vec<Node>,
}

impl Node {
    /// Creates a new node
    /// # Arguments
    /// * `val` - a [NodeVal]
    pub fn new_with_val(val: NodeVal) -> Self {
        Self {
            val: Some(val),
            children: Vec::new(),
        }
    }

    /// Creates an empty node
    pub fn new_empty() -> Self {
        Self {
            val: None,
            children: Vec::new(),
        }
    }

    /// Adds a childrent this this node
    /// # Arguments
    /// * `child` - a Node
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn val(&self) -> Option<&NodeVal> {
        match &self.val {
            None => None,
            Some(nodeval) => Some(nodeval),
        }
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Node> {
        &mut self.children
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.val {
            None => {
                write!(f, "None")
            }
            Some(node_val) => match node_val {
                NodeVal::Leaf(token) => {
                    if token.token_type() == StringLit {
                        write!(f, "Token: {}", "String Lit")
                    } else {
                        write!(f, "{}", token)
                    }
                }
                NodeVal::Internal(internal) => {
                    write!(f, "{}", internal)
                }
            },
        }
    }
}

/// The semantic stack is used to proccess semantic actions into an Abstract Syntax Tree composed of [Node]s
#[derive(Debug)]
pub struct SemanticStack(pub(crate) Vec<Node>);

impl SemanticStack {
    /// Creates a new empty semantic stack
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates & pushes a new internal [Node] on the semantic stack
    /// # Arguments
    /// * `node_t` - the [InternalNodeType] to create a node with
    pub fn make_family_root(&mut self, node_t: InternalNodeType) {
        debug!("Making family with {:?}", node_t);
        self.0.push(Node::new_with_val(NodeVal::Internal(node_t)));
    }

    /// Creates & pushes a new leaf [Node] on the semantic stack.
    /// # Arguments
    /// * `token` - the [Token] to create a node with
    pub fn make_terminal_node(&mut self, token: &Token) {
        debug!("Making terminal node with: {:?}", token);
        self.0
            .push(Node::new_with_val(NodeVal::Leaf(token.clone())));
    }

    /// Consumes the top 3 nodes, if possible, to create a new relative operation (lhs - rhs children of middle).
    pub fn make_relative_operation(&mut self) {
        let rhs = self.0.pop();
        let op = self.0.pop();
        let lhs = self.0.pop();

        if rhs.is_none() || op.is_none() || lhs.is_none() {
            warn!(
                "Failed to make relative operation node: {:?} {:?} {:?}",
                lhs, op, rhs
            );
            return;
        }

        let lhs = lhs.unwrap();
        let mut op = op.unwrap();
        let rhs = rhs.unwrap();

        debug!(
            "Making relative operation node: {:?} {:?} {:?}",
            lhs.val, op.val, rhs.val
        );

        op.add_child(lhs);
        op.add_child(rhs);
        self.0.push(op);
    }

    /// Creates & pushes a new empty [Node].
    pub fn make_empty_node(&mut self) {
        self.0.push(Node::new_empty());
        debug!("Added empty node")
    }

    /// Pops the top [Node] and adds it as a child of the next top [Node].
    pub fn add_child(&mut self) {
        let child = self.0.pop();
        let top = self.0.last_mut();

        if child.is_none() || top.is_none() {
            warn!("Failed to add child {:?} to {:?}", child, top);
            return;
        }

        let child = child.unwrap();
        let top = top.unwrap();

        debug!("Adding {:?} as a child of {:?}", child.val, top.val);
        top.children.push(child);
    }

    pub fn into_ast_root(mut self) -> Result<Node, ()> {
        if self.0.len() == 0 || self.0.len() > 1 {
            return Err(());
        } else {
            return Ok(self.0.pop().unwrap());
        }
    }
}

impl Display for SemanticStack {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Semantic Stack -> ")?;
        for node in self.0.iter() {
            write!(f, "{} ", node)?;
        }
        fmt::Result::Ok(())
    }
}

/// Represents the different semantic actions of the grammar. Maps to functions of [SemanticStack]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SemanticAction {
    MakeFamilyRootNode(InternalNodeType),
    MakeTerminalNode,
    MakeRelativeOperation,
    MakeEmptyNode,
    AddChild,
}

impl Display for SemanticAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents possible values held by [Node]s
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeVal {
    Leaf(Token),
    Internal(InternalNodeType),
}

/// Represents different semantic concepts in our grammar
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum InternalNodeType {
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
    Variable,
    Term,
    StatBlock,
    DotOp,
}

impl Display for InternalNodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

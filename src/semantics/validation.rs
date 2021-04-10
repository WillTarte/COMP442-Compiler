use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::{InternalNodeType, Node, NodeVal};
use crate::semantics::checking::SemanticError;
use crate::semantics::symbol_table::Scope::{FunctionParameter, Variable};
use crate::semantics::symbol_table::Type::{Integer, Void, Float};
use crate::semantics::symbol_table::{FunctionEntry, Scope, SymbolTable, Type};
use crate::semantics::utils::map_token_to_type;

#[allow(dead_code)]
fn validate_statement(
    statement: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<(), SemanticError> {
    match statement.val() {
        Some(NodeVal::Internal(InternalNodeType::GenericStatement)) => {
            match statement.children()[0].val()
            {
                Some(NodeVal::Leaf(func_id)) => {
                    return validate_free_func_call(func_id, &statement.children()[0].children()[0], function_entry, global).and(Ok(()));
                },
                Some(NodeVal::Internal(InternalNodeType::Assignment)) => {
                    return validate_assignment(&statement.children()[0], function_entry, global).and(Ok(()));
                },
                _ => { panic!() }
            }
        }
        Some(NodeVal::Internal(InternalNodeType::IfStatement)) => {
            return validate_if_statement(&statement.children()[0], function_entry, global).and(Ok(()));
        }
        Some(NodeVal::Internal(InternalNodeType::WhileStatement)) => {
            return validate_while_statement(&statement.children()[0], function_entry, global).and(Ok(()));
        }
        Some(NodeVal::Internal(InternalNodeType::ReadStatement)) => {
            return validate_read_statement(&statement.children()[0], function_entry, global).and(Ok(()));
        }
        Some(NodeVal::Internal(InternalNodeType::WriteStatement)) => {
            return validate_write_statement(&statement.children()[0], function_entry, global).and(Ok(()));
        }
        Some(NodeVal::Internal(InternalNodeType::ReturnStatement)) => {
           // return validate_return_statement(&statement.children()[0], function_entry, global);
            return todo!();
        }
        Some(NodeVal::Internal(InternalNodeType::BreakStatement))
        | Some(NodeVal::Internal(InternalNodeType::ContinueStatement)) => {
            return todo!();//Ok(Void); // todo should only be called inside while loop
        },
        None => {
            return Ok(());
        }
        _ => {
            log::warn!("validate_statement called on a non-statement node")
        }
    };

    //Ok(Integer)
    Ok(())
}

// todo Assumes that the lhs are just variables (can be class members)
fn validate_assignment(
    assignment: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable
) -> Result<Type, SemanticError>
{
    // check lhs semantics
    let lhs: &Node = &assignment.children()[0];
    let (lhs_lex, line_num) = match lhs.val() {
        None => { panic!() }
        Some(NodeVal::Leaf(t)) => { (t.lexeme(), t.line_num()) }
        _ => { panic!() }
    };
    let rhs: &Node = &assignment.children()[1];
    let rhs_res: Result<Type, SemanticError> = validate_expr(rhs, function_entry, global);;
    if rhs_res.is_err()
    {
        return rhs_res;
    }
    let rhs_res = rhs_res.unwrap();

    if lhs_lex != "." //todo
    {
        // lex is ident
        if let Some(Scope::FunctionParameter(e)) = function_entry.table().find_scope_by_ident(lhs_lex)
        {
            if &rhs_res == e.param_type()
            {
                return Ok(Void);
            }
            else {
                return Err(SemanticError::TypeMistmatch(format!("Type mistmatch: Expected {:?}, but got {:?}: line {}", e.param_type(), rhs_res, line_num)));
            }
        }
        else if let Some(Scope::Variable(e)) = function_entry.table().find_scope_by_ident(lhs_lex)
        {
            if &rhs_res == e.var_type()
            {
                return Ok(Void);
            }
            else {
                return Err(SemanticError::TypeMistmatch(format!("Type mistmatch: Expected {:?}, but got {:?}: line {}", e.var_type(), rhs_res, line_num)));
            }
        }
        else {
            return Err(SemanticError::UndeclaredVariable(format!("Undeclared variable {}: line {}", lhs_lex, line_num)));
        }
    }
    else {
        let lhs_res = validate_dot_operator(lhs, function_entry, global);
        if lhs_res.is_err()
        {
            return lhs_res;
        }
        else {
            let lhs_res = lhs_res.unwrap();
            if lhs_res == rhs_res
            {
                return Ok(Void);
            }
            else {
                return Err(SemanticError::TypeMistmatch(format!("Type mistmatch: Expected {:?}, but got {:?}: line {}", lhs_res, rhs_res, line_num)));
            }
    }
    }
}

/// Validates if a function is called correctly (is defined, correct num of params, correct type of params) and returns the return type of the call.
fn validate_free_func_call(
    ident_token: &Token,
    func_call_params: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable
) -> Result<Type, SemanticError> {

    // check if function exists
    //todo recursion?
    if let Some(Scope::Function(e)) = global.find_scope_by_ident(ident_token.lexeme())
    {
        // parse func call params
        let mut call_types: Vec<Type> = Vec::new();
        for param in func_call_params.children().iter()
        {
            if param.val().is_none() // no params
            {
                break;
            }
            else {
                assert_eq!(param.val(), Some(&NodeVal::Internal(InternalNodeType::Expr)));
                let res: Result<Type, SemanticError> = validate_expr(param, function_entry, global);;
                if res.is_err()
                {
                    return res;
                }
                else {
                    call_types.push(res.unwrap());
                }
            }
        }
        // check if function is called correctly
        if call_types == e.type_sig().0
        {
            return Ok(e.type_sig().1.clone())
        }
        else {
            return Err(SemanticError::InvalidParameters(format!("Invalid parameters used for function {}: line {}", ident_token.lexeme(), ident_token.line_num())));
        }
    }
    else {
        return Err(SemanticError::FunctionNotFound(format!("Free function {} not found: line {}", ident_token.lexeme(), ident_token.lexeme())));
    }
}

/// Validates Indice -> idx should be arith expr returning an integer
fn validate_indices(
    ident_token: &Token,
    indices: &Vec<Node>,
    function_entry: &FunctionEntry,
    global: &SymbolTable
) -> Result<(), SemanticError>
{
    /// validate Arith expr -> should return integer
    for indice in indices
    {
        let res = validate_arith_expr(indice, function_entry, global);
        if res.is_err()
        {
            return Err(res.unwrap_err());
        }
        else {
            let res = res.unwrap();
            if res != Integer
            {
                return Err(SemanticError::TypeMistmatch(format!("Expected integer array index. Got {:?} instead: line {}", res, ident_token.line_num())));
            }
        }
    }

    Ok(())
}

/// Validate an If statement -> {rel expr, stat block, stat block}
fn validate_if_statement(if_statement: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<(), SemanticError>
{
    // validate rel expr
    let rel_expr_res: Result<Type, SemanticError> = validate_rel_expr(&if_statement.children()[0], function_entry, global);
    if rel_expr_res.is_err()
    {
        return Err(rel_expr_res.unwrap_err());
    }

    // validate statblock 1 (then)
    for statement in if_statement.children()[1].children()
    {
        let res = validate_statement(statement, function_entry, global);
        if res.is_err()
        {
            return res;
        }
        else {
            continue;
        }
    }

    // validate statblock 2 (else)
    for statement in if_statement.children()[2].children()
    {
        let res = validate_statement(statement, function_entry, global);
        if res.is_err()
        {
            return res;
        }
        else {
            continue;
        }
    }

    Ok(())
}

/// Validates a While statement -> {rel expr, stat block)
fn validate_while_statement(while_statement: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<(), SemanticError>
{
    // validate rel expr
    let rel_expr_res: Result<Type, SemanticError> = validate_rel_expr(&while_statement.children()[0], function_entry, global);
    if rel_expr_res.is_err()
    {
        return Err(rel_expr_res.unwrap_err());
    }
    // validate statblock
    for statement in while_statement.children()[1].children()
    {
        let res = validate_statement(statement, function_entry, global);
        if res.is_err()
        {
            return res;
        }
        else {
            continue;
        }
    }

    Ok(())
}

fn validate_read_statement(read_statement: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<(), SemanticError>
{
    // validate variable
    //<variable> ::= #MakeFamilyRootNode("Variable") MakeTerminalNode 'id' <variableAmb1> #AddChild
    //<variableAmb1> ::= <rept-variable> #MakeTerminalNode '.' MakeTerminalNode 'id'  #MakeRelativeOperation <variableAmb1>
    //<variableAmb1> ::= '(' <params> #AddChild ')' #MakeTerminalNode '.' #MakeTerminalNode 'id'  #MakeRelativeOperation <variableAmb1>
    //<variableAmb1> ::= EPSILON

    let res: Result<Type, SemanticError> = todo!("validate variable: read_statement.children()[0]");

    if res.is_err()
    {
        Err(res.unwrap_err())
    }
    else {
        Ok(())
    }
}

fn validate_write_statement(write_statement: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<(), SemanticError>
{
    //validate expr
    let res: Result<Type, SemanticError> = validate_expr(&write_statement.children()[0], function_entry, global);;

    if res.is_err()
    {
        Err(res.unwrap_err())
    }
    else {
        Ok(())
    }
}

fn validate_return_statement(return_statement: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<Type, SemanticError>
{
    // validate expr
    let res: Result<Type, SemanticError> = validate_expr(&return_statement.children()[0], function_entry, global);
    return res;
}

fn validate_dot_operator(dot_op: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<Type, SemanticError>
{
    let lhs: Result<Type, SemanticError> = match dot_op.children()[0].val()
    {
        Some(NodeVal::Leaf(token)) => {
            if token.lexeme() == "." //todo
            {
                validate_dot_operator(&dot_op.children()[0], function_entry, global)
            }
            else {
                // We have an identifier that should be present in the current function scope
                // todo what if this function is a member of the class
                let lhs_ident = token.lexeme();
                if let Some(Scope::Variable(ve)) = function_entry.table().find_scope_by_ident(lhs_ident)
                {
                    // todo validate indices
                    Ok(ve.var_type().clone())
                }
                else if let Some(Scope::FunctionParameter(pe)) = function_entry.table().find_scope_by_ident(lhs_ident)
                {
                    //todo validate indices
                    Ok(pe.param_type().clone())
                }
                else if  let Some(Scope::Function(fe)) = global.find_scope_by_ident(lhs_ident){
                    //todo validate func call params
                    Ok(fe.type_sig().1.clone())
                }
                else {
                    Err(SemanticError::UndeclaredVariable(format!("Undeclared variable {}: line {}", lhs_ident, token.line_num())))
                }
            }
        },
        _ => { panic!() }
    };

    if lhs.is_err()
    {
        return lhs;
    }
    let lhs = lhs.unwrap();

    //let rhs: Result<Type, SemanticError> = match &Node
    todo!()
}

/// Validates an Expr
fn validate_expr(expr: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<Type, SemanticError>
{
    /*
    <expr> ::= #MakeFamilyRootNode("Expr") <arithExpr> <exprAmb1>

    <exprAmb1> ::= <relOp> <arithExpr> #MakeRelativeOperation #AddChild
    <exprAmb1> ::= EPSILON #AddChild

    <arithExpr> ::= #MakeFamilyRootNode("ArithExpr") <term> <rightrec-arithExpr> #AddChild

    <relOp> ::= #MakeFamilyRootNode("Equal") 'eq'
    <relOp> ::= #MakeFamilyRootNode("NotEqual") 'neq'
    <relOp> ::= #MakeFamilyRootNode("LessThan") 'lt'
    <relOp> ::= #MakeFamilyRootNode("GreaterThan") 'gt'
    <relOp> ::= #MakeFamilyRootNode("LessEqualThan") 'leq'
    <relOp> ::= #MakeFamilyRootNode("GreaterEqualThan") 'geq'
     */

    let res: Result<Type, SemanticError> = match expr.children()[0].val()
    {
       Some(NodeVal::Internal(InternalNodeType::Equal))
       | Some(NodeVal::Internal(InternalNodeType::NotEqual))
       | Some(NodeVal::Internal(InternalNodeType::LessThan))
       | Some(NodeVal::Internal(InternalNodeType::GreaterThan))
       | Some(NodeVal::Internal(InternalNodeType::LessEqualThan))
       | Some(NodeVal::Internal(InternalNodeType::GreaterEqualThan)) => {
           let rel_op = &expr.children()[0];
           let lhs_res = validate_arith_expr(&rel_op.children()[0], function_entry, global);
           let rhs_res = validate_arith_expr(&rel_op.children()[1], function_entry, global);
           if lhs_res.is_err()
           {
               lhs_res
           }
           else if rhs_res.is_err()
           {
               rhs_res
           }
           else {
               let lhs_res = lhs_res.unwrap();
               let rhs_res = rhs_res.unwrap();
               if lhs_res != rhs_res
               {
                   Err(SemanticError::TypeMistmatch(format!("Type Mistmatch in relative operation: {:?} {:?} {:?}: line {}", lhs_res, rel_op.val(), rhs_res, 999))) //todo line num
               }
               else {
                   Ok(Integer) //todo
               }
           }
       },
        Some(NodeVal::Internal(InternalNodeType::ArithExpr)) => {
            let arith_res = validate_arith_expr(&expr.children()[0], function_entry, global);
            arith_res
        },
        _ => { panic!("failed match expr for {:?}", expr.children()[0].val()) }
    };

    return res;
}

///Validates a Rel Expr
fn validate_rel_expr(rel_expr: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<Type, SemanticError>
{
    let lhs_res = validate_arith_expr(&rel_expr.children()[0], function_entry, global);
    if lhs_res.is_err()
    {
        return lhs_res;
    }
    let lhs_res = lhs_res.unwrap();

    let rhs_res = validate_arith_expr(&rel_expr.children()[1], function_entry, global);
    if rhs_res.is_err()
    {
        return rhs_res
    }
    let rhs_res = rhs_res.unwrap();

    if lhs_res != rhs_res
    {
        return Err(SemanticError::TypeMistmatch(format!("Type mistmatch in rel expr -> {:?} {:?} {:?}", lhs_res, rel_expr.val(), rhs_res)));
    }

    return Ok(lhs_res);
}

/// Validate an Arith Expr
fn validate_arith_expr(arith_expr: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<Type, SemanticError>
{
    /*
    <arithExpr> ::= #MakeFamilyRootNode("ArithExpr") <term> <rightrec-arithExpr> #AddChild

    <term> ::= #MakeFamilyRootNode("Term") <factor> <rightrec-term> #AddChild

    <rightrec-arithExpr> ::= <addOp> <term> #MakeRelativeOperation <rightrec-arithExpr>
    <rightrec-arithExpr> ::= EPSILON

    <rightrec-term> ::= <multOp> <factor> #MakeRelativeOperation <rightrec-term>
    <rightrec-term> ::= EPSILON

    <factor> ::= #MakeTerminalNode 'intLit'
    <factor> ::= #MakeTerminalNode 'floatLit'
    <factor> ::= #MakeTerminalNode 'stringLit'
    <factor> ::= '(' <arithExpr> ')'
    <factor> ::= #MakeFamilyRootNode("Negation") 'not' <factor> #AddChild
    <factor> ::= #MakeFamilyRootNode("SignedFactor") <sign> #AddChild <factor> #AddChild
    <factor> ::= #MakeFamilyRootNode("TernaryOperation") 'qm' '[' <expr> #AddChild ':' <expr> #AddChild ':' <expr> #AddChild ']'

    <factor> ::= #MakeFamilyRootNode("Factor") #MakeTerminalNode 'id' <factorAmb1> #AddChild
    <factorAmb1> ::= <rept-variable> <factorAmb2>
    <factorAmb1> ::= '(' <params> ')' #AddChild <factorAmb2>
    <factorAmb2> ::= #MakeTerminalNode '.' #MakeTerminalNode 'id' #MakeRelativeOperation <factorAmb1>
    <factorAmb2> ::= EPSILON

    <addOp> ::= #MakeFamilyRootNode("Add") '+'
    <addOp> ::= #MakeFamilyRootNode("Sub") '-'
    <addOp> ::= #MakeFamilyRootNode("Or") 'or'

    <multOp> ::= #MakeFamilyRootNode("Mult") '*'
    <multOp> ::= #MakeFamilyRootNode("Div") '/'
    <multOp> ::= #MakeFamilyRootNode("And") 'and'
     */

    let arith_expr_res: Result<Type, SemanticError> = match arith_expr.children()[0].val()
    {
        Some(NodeVal::Internal(InternalNodeType::Add))
        | Some(NodeVal::Internal(InternalNodeType::Sub))
        | Some(NodeVal::Internal(InternalNodeType::Or)) => {
            validate_add_op(&arith_expr.children()[0], function_entry, global)
        },
        Some(NodeVal::Internal(InternalNodeType::Term)) => {
            validate_term(&arith_expr.children()[0], function_entry, global)
        },
        _ => { panic!("Failed match arith expr") }
    };

    return arith_expr_res;
}

/// Validates a Term
fn validate_term(term: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<Type, SemanticError>
{
    /*
    <term> ::= #MakeFamilyRootNode("Term") <factor> <rightrec-term> #AddChild

    <rightrec-term> ::= <multOp> <factor> #MakeRelativeOperation <rightrec-term>
    <rightrec-term> ::= EPSILON

    <factor> ::= #MakeTerminalNode 'intLit'
    <factor> ::= #MakeTerminalNode 'floatLit'
    <factor> ::= #MakeTerminalNode 'stringLit'
    <factor> ::= '(' <arithExpr> ')'
    <factor> ::= #MakeFamilyRootNode("Negation") 'not' <factor> #AddChild
    <factor> ::= #MakeFamilyRootNode("SignedFactor") <sign> #AddChild <factor> #AddChild
    <factor> ::= #MakeFamilyRootNode("TernaryOperation") 'qm' '[' <expr> #AddChild ':' <expr> #AddChild ':' <expr> #AddChild ']'

    <factor> ::= #MakeFamilyRootNode("Factor") #MakeTerminalNode 'id' <factorAmb1> #AddChild
    <factorAmb1> ::= <rept-variable> <factorAmb2>
    <factorAmb1> ::= '(' <params> ')' #AddChild <factorAmb2>
    <factorAmb2> ::= #MakeTerminalNode '.' #MakeTerminalNode 'id' #MakeRelativeOperation <factorAmb1>
    <factorAmb2> ::= EPSILON

    <multOp> ::= #MakeFamilyRootNode("Mult") '*'
    <multOp> ::= #MakeFamilyRootNode("Div") '/'
    <multOp> ::= #MakeFamilyRootNode("And") 'and'
     */

    let term_res: Result<Type, SemanticError> = match term.children()[0].val()
    {
        Some(NodeVal::Internal(InternalNodeType::Mult))
        | Some(NodeVal::Internal(InternalNodeType::Div))
        | Some(NodeVal::Internal(InternalNodeType::And)) => {
            validate_mult_op(&term.children()[0], function_entry, global)
        },
        Some(NodeVal::Leaf(token)) => {
            Ok(map_token_to_type(token))
        },
        Some(NodeVal::Internal(InternalNodeType::Factor)) => { validate_ident_factor(&term.children()[0], function_entry, global) } // Factor node has id child
        Some(NodeVal::Internal(InternalNodeType::ArithExpr)) => { validate_arith_expr(&term.children()[0], function_entry, global) }
        Some(NodeVal::Internal(InternalNodeType::SignedFactor)) => {
            match term.children()[0].children()[1].val() {
                Some(NodeVal::Internal(InternalNodeType::Factor)) => { validate_ident_factor(&term.children()[0].children()[1], function_entry, global) },
                Some(NodeVal::Internal(InternalNodeType::ArithExpr)) => { validate_arith_expr(&term.children()[0], function_entry, global) },
                Some(NodeVal::Leaf(token)) => {
                    Ok(map_token_to_type(token))
                }
                _ => { panic!() }
            }
        },
        Some(NodeVal::Internal(InternalNodeType::Negation)) => {
            // Factor, token, arith expr
            match term.children()[0].children()[0].val() {
                Some(NodeVal::Internal(InternalNodeType::Factor)) => { validate_ident_factor(&term.children()[0].children()[1], function_entry, global) },
                Some(NodeVal::Internal(InternalNodeType::ArithExpr)) => { validate_arith_expr(&term.children()[0], function_entry, global) },
                Some(NodeVal::Leaf(token)) => {
                    Ok(map_token_to_type(token))
                }
                _ => { panic!() }
            }
        },
        Some(NodeVal::Internal(InternalNodeType::TernaryOperation)) => {
            validate_ternary_operation(&term.children()[0], function_entry, global)
        }
        _ => { panic!("Failed match term") }
    };

   return term_res;
}

//Todo factor that is just identifiers (and maybe some indexing and func params)
fn validate_ident_factor(ident_factor: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<Type, SemanticError>
{
    /*
    <factor> ::= #MakeFamilyRootNode("Factor") #MakeTerminalNode 'id' <factorAmb1> #AddChild
    <factorAmb1> ::= <rept-variable> <factorAmb2>
    <factorAmb1> ::= '(' <params> ')' #AddChild <factorAmb2>
    <factorAmb2> ::= #MakeTerminalNode '.' #MakeTerminalNode 'id' #MakeRelativeOperation <factorAmb1>
    <factorAmb2> ::= EPSILON
     */

    match ident_factor.children()[0].val() {
        Some(NodeVal::Leaf(token)) =>
        {
            let lex = token.lexeme();
            if lex == "." //todo
            {
                return validate_dot_operator(&ident_factor.children()[0], function_entry, global);
            }
            else {
                return validate_ident(token, &ident_factor.children()[0], function_entry, global);
            }
        },
        _ => { panic!() }
    }
}

/// Essentially validates starting from an ident. Can be a free function call or array indexing
fn validate_ident(ident_token: &Token, ident_node: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<Type, SemanticError>
{
    if let Some(Scope::FunctionParameter(e)) = function_entry.table().find_scope_by_ident(ident_token.lexeme())
    {
        if ident_node.children().len() > 0
        {
            match ident_node.children()[0].val()
            {
                Some(NodeVal::Internal(InternalNodeType::FuncCallParams)) => {
                    return Err(SemanticError::NotCallable(format!("Function Param {} is not callable: line {}", e.ident(), ident_token.line_num())));
                },
                Some(NodeVal::Internal(InternalNodeType::Indice)) => {
                    match e.param_type()
                    {
                        Type::IntegerArray(dim) |
                        Type::FloatArray(dim) |
                        Type::StringArray(dim) |
                        Type::CustomArray(_, dim) => {
                            if dim.len() < ident_node.children().len()
                            {
                                return Err(SemanticError::TooManyIndices(format!("Tried indexing param {} with {} indices, but it only has {} dimensions: line {}", e.ident(), ident_node.children().len(), dim.len(), ident_token.line_num())));
                            }
                            let idx_res = validate_indices(ident_token, &ident_node.children(), function_entry, global);
                            if idx_res.is_err()
                            {
                                return Err(idx_res.unwrap_err());
                            }
                            if dim.len() == ident_node.children().len()
                            {
                                return Ok(e.param_type().clone());
                            }
                            else {
                                return Ok(e.param_type().to_array_type(dim[ident_node.children().len()..].to_vec()))
                            }
                        },
                        _ => { return Err(SemanticError::NotIndexable(format!("Func Param {} is not indexable: line {}", e.ident(), ident_token.line_num()))) }
                    }
                },
                _ => { panic!("Did not expect these children() to id token") }
            }
        }
        else {
            return Ok(e.param_type().clone());
        }

    }
    else if let Some(Scope::Variable(e)) = function_entry.table().find_scope_by_ident(ident_token.lexeme())
    {
        if ident_node.children().len() > 0
        {
            match ident_node.children()[0].val()
            {
                Some(NodeVal::Internal(InternalNodeType::FuncCallParams)) => {
                    return Err(SemanticError::NotCallable(format!("Function Param {} is not callable: line {}", e.ident(), ident_token.line_num())));
                },
                Some(NodeVal::Internal(InternalNodeType::Indice)) => {
                    match e.var_type()
                    {
                        Type::IntegerArray(dim) |
                        Type::FloatArray(dim) |
                        Type::StringArray(dim) |
                        Type::CustomArray(_, dim) => {
                            if dim.len() < ident_node.children().len()
                            {
                                return Err(SemanticError::TooManyIndices(format!("Tried indexing param {} with {} indices, but it only has {} dimensions: line {}", e.ident(), ident_node.children().len(), dim.len(), ident_token.line_num())));
                            }
                            let idx_res = validate_indices(ident_token, &ident_node.children(), function_entry, global);
                            if idx_res.is_err()
                            {
                                return Err(idx_res.unwrap_err());
                            }
                            if dim.len() == ident_node.children().len()
                            {
                                return Ok(e.var_type().clone());
                            }
                            else {
                                return Ok(e.var_type().to_array_type(dim[ident_node.children().len()..].to_vec()))
                            }
                        },
                        _ => { return Err(SemanticError::NotIndexable(format!("Func Param {} is not indexable: line {}", e.ident(), ident_token.line_num()))) }
                    }
                },
                _ => { panic!("Did not expect these children() to id token") }
            }
        }
        else {
            return Ok(e.var_type().clone());
        }
    }
    else if let Some(Scope::Function(e)) = global.find_scope_by_ident(ident_token.lexeme()) {
        return validate_free_func_call(ident_token, &ident_node.children()[0], function_entry, global);
    }
    else {
        return Err(SemanticError::UndeclaredVariable(format!("Undeclared variable {}: line {}", ident_token.lexeme(), ident_token.line_num())));
    }
}

/// Validates an add_op (+, -, or)
fn validate_add_op(op_node: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<Type, SemanticError> {
    // lhs might be a term, or another add op
    // rhs is a term

    let lhs_res = match op_node.children()[0].val()
    {
        Some(NodeVal::Internal(InternalNodeType::Term)) => {
            validate_term(&op_node.children()[0], function_entry, global)
        },
        Some(NodeVal::Internal(InternalNodeType::Add))
        | Some(NodeVal::Internal(InternalNodeType::Sub))
        | Some(NodeVal::Internal(InternalNodeType::Or)) => {
            validate_add_op(&op_node.children()[0], function_entry, global)
        },
        _ => { panic!("Failed to match child of add op") }
    };
    if lhs_res.is_err()
    {
        return lhs_res;
    }
    let lhs_res = lhs_res.unwrap();

    let rhs_res = validate_term(&op_node.children()[1], function_entry, global);
    if rhs_res.is_err()
    {
        return rhs_res;
    }
    let rhs_res = rhs_res.unwrap();

    if lhs_res != rhs_res
    {
        return Err(SemanticError::TypeMistmatch(format!("Type Mistmatch in add op: lhs {:?}, rhs {:?}: line {}", lhs_res, rhs_res, 888))) //todo
    }
    else {
        return Ok(lhs_res);
    }
}

/// Validates a mult_op (*, /, and)
fn validate_mult_op(op_node: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<Type, SemanticError> {
    // lhs might be a factor, or another mult op
    // rhs is a factor or token

    let lhs_res = match op_node.children()[0].val()
    {
        Some(NodeVal::Internal(InternalNodeType::Factor)) => {
            validate_ident_factor(&op_node.children()[0], function_entry, global)
        },
        Some(NodeVal::Internal(InternalNodeType::Mult))
        | Some(NodeVal::Internal(InternalNodeType::Div))
        | Some(NodeVal::Internal(InternalNodeType::And)) => {
            validate_mult_op(&op_node.children()[0], function_entry, global)
        },
        _ => { panic!("Failed to match child of add op") }
    };
    if lhs_res.is_err()
    {
        return lhs_res;
    }
    let lhs_res = lhs_res.unwrap();

    let rhs_res = match op_node.children()[1].val()
    {
        Some(NodeVal::Internal(InternalNodeType::Factor)) => {todo!()},
        Some(NodeVal::Leaf(token)) => {
            match token.token_type()
            {
                TokenType::IntegerLit => {
                    Ok(Integer)
                },
                TokenType::FloatLit => {
                    Ok(Float)
                },
                TokenType::Id => { validate_ident(token, &op_node.children()[1], function_entry, global) },
                _ => { panic!() }
            }
        }
        _ => { panic!() }
    };
    if rhs_res.is_err()
    {
        return rhs_res;
    }
    let rhs_res = rhs_res.unwrap();

    if lhs_res != rhs_res
    {
        return Err(SemanticError::TypeMistmatch(format!("Type Mistmatch in mult op: lhs {:?}, rhs {:?}: line {}", lhs_res, rhs_res, 888))) //todo
    }
    else {
        return Ok(lhs_res);
    }
}

/// Validates a ternary operation
fn validate_ternary_operation(ter: &Node, function_entry: &FunctionEntry, global: &SymbolTable) -> Result<Type, SemanticError>
{
    //validate 3 expressions
    let cond_expr = validate_expr(&ter.children()[0], function_entry, global);
    if cond_expr.is_err()
    {
        return cond_expr;
    }
    let cond_expr = cond_expr.unwrap();

    let then_expr = validate_expr(&ter.children()[1], function_entry, global);
    if then_expr.is_err()
    {
        return then_expr;
    }
    let then_expr = then_expr.unwrap();

    let else_expr = validate_expr(&ter.children()[2], function_entry, global);
    if else_expr.is_err()
    {
        return else_expr;
    }
    let else_expr = else_expr.unwrap();

    if cond_expr != Integer
    {
        return Err(SemanticError::TypeMistmatch(format!("Expected integer for condition of ternary operation. Got {:?} instead", cond_expr)));
    }

    if then_expr != else_expr
    {
        return Err(SemanticError::TypeMistmatch(format!("Mistmatched ternary op types -> {:?} : {:?}", then_expr, else_expr)))
    }

    return Ok(then_expr);
}
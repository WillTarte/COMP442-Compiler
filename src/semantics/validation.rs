use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::{InternalNodeType, Node, NodeVal};
use crate::semantics::checking::SemanticError;
use crate::semantics::symbol_table::Type::{Float, Integer};
use crate::semantics::symbol_table::{FunctionEntry, Scope, SymbolTable, Type};
use crate::semantics::utils::{get_class_hierarchy_functions, map_token_to_type};

#[allow(dead_code)]
pub(crate) fn validate_statement(
    statement: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<(), SemanticError> {
    match statement.val() {
        Some(NodeVal::Internal(InternalNodeType::GenericStatement)) => {
            match statement.children()[0].val() {
                Some(NodeVal::Leaf(func_id)) => {
                    return validate_func_call_statement(
                        func_id,
                        &statement.children()[0],
                        function_entry,
                        global,
                    );
                }
                Some(NodeVal::Internal(InternalNodeType::Assignment)) => {
                    return validate_assignment(&statement.children()[0], function_entry, global);
                }
                Some(NodeVal::Internal(InternalNodeType::DotOp)) => {
                    return validate_dot_operator(&statement.children()[0], function_entry, global)
                        .and(Ok(()));
                }
                _ => {
                    panic!()
                }
            }
        }
        Some(NodeVal::Internal(InternalNodeType::IfStatement)) => {
            return validate_if_statement(statement, function_entry, global);
        }
        Some(NodeVal::Internal(InternalNodeType::WhileStatement)) => {
            return validate_while_statement(statement, function_entry, global);
        }
        Some(NodeVal::Internal(InternalNodeType::ReadStatement)) => {
            return validate_read_statement(statement, function_entry, global);
        }
        Some(NodeVal::Internal(InternalNodeType::WriteStatement)) => {
            return validate_write_statement(statement, function_entry, global);
        }
        Some(NodeVal::Internal(InternalNodeType::ReturnStatement)) => {
            return validate_return_statement(statement, function_entry, global);
        }
        Some(NodeVal::Internal(InternalNodeType::BreakStatement))
        | Some(NodeVal::Internal(InternalNodeType::ContinueStatement)) => {
            log::error!("Break | Continue semantic checking: NOT IMPLEMENTED");
            return Ok(());
        }
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

fn validate_assignment(
    assignment: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<(), SemanticError> {
    log::warn!("validating assigment statement");
    // check lhs semantics
    let lhs: &Node = &assignment.children()[0];
    let lhs_ty = match lhs.val() {
        Some(NodeVal::Internal(InternalNodeType::DotOp)) => {
            validate_dot_operator(lhs, function_entry, global)
        }
        Some(NodeVal::Leaf(token)) => validate_ident(token, lhs, function_entry, global),
        _ => {
            panic!()
        }
    };
    if lhs_ty.is_err() {
        return Err(lhs_ty.unwrap_err());
    }
    let lhs_ty = lhs_ty.unwrap();

    let rhs: &Node = &assignment.children()[1];
    let rhs_ty: Result<Type, SemanticError> = validate_expr(rhs, function_entry, global);
    if rhs_ty.is_err() {
        return Err(rhs_ty.unwrap_err());
    }
    let rhs_ty = rhs_ty.unwrap();

    if lhs_ty == rhs_ty {
        return Ok(());
    } else {
        return Err(SemanticError::TypeMistmatch(format!(
            "Assignment lhs expected {:?}, but got {:?} instead",
            lhs_ty, rhs_ty
        )));
    }
}
/// Validates a function call statement (always free function ?)
fn validate_func_call_statement(
    func_id: &Token,
    func_call_statement: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<(), SemanticError> {
    log::warn!(
        "validating function call statement for {}",
        func_id.lexeme()
    );
    let mut func_call_errs: Vec<SemanticError> = Vec::new();
    // member function call if statement is within member function
    if function_entry.member_of().is_some() {
        let class_ident = function_entry.member_of().unwrap();
        if let Some(Scope::Class(ce)) = global.find_scope_by_ident(class_ident) {
            let (functions, mut errors) = get_class_hierarchy_functions(ce, global);
            if errors.len() > 0 {
                return Err(errors.pop().unwrap());
            } else {
                for fe in functions {
                    let func_call_res = validate_function_call(
                        func_id,
                        &func_call_statement.children()[0],
                        fe,
                        function_entry,
                        global,
                    );
                    if func_call_res.is_ok() {
                        return Ok(());
                    } else {
                        func_call_errs.push(func_call_res.unwrap_err());
                    }
                }
            }
        }
    } else {
        // free function call
        for scope in global.find_all_scopes_by_ident(func_id.lexeme()) {
            if let Scope::Function(fe) = scope {
                if fe.ident() == func_id.lexeme() {
                    let func_call_res = validate_function_call(
                        func_id,
                        &func_call_statement.children()[0],
                        fe,
                        function_entry,
                        global,
                    );
                    if func_call_res.is_ok() {
                        return Ok(());
                    } else {
                        func_call_errs.push(func_call_res.unwrap_err());
                    }
                }
            }
        }
    }

    return Err(func_call_errs.pop().unwrap());
}

/// Validates if a function is called correctly (is defined, correct num of params, correct type of params) and returns the return type of the call.
fn validate_function_call(
    ident_token: &Token,
    func_call_params: &Node,
    called_function: &FunctionEntry,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<Type, SemanticError> {
    log::warn!("validating function call");
    log::warn!(
        "{}({} params)",
        ident_token.lexeme(),
        func_call_params.children().len()
    );
    if called_function == function_entry {
        return Err(SemanticError::RecursionNotSupported(format!(
            "Recursion not supported: line {}",
            ident_token.line_num()
        )));
    }

    let mut param_types: Vec<Type> = Vec::new();
    for param in func_call_params.children().iter() {
        if param.val().is_none()
        // no params
        {
            break;
        } else {
            assert_eq!(
                param.val(),
                Some(&NodeVal::Internal(InternalNodeType::Expr))
            );
            let res: Result<Type, SemanticError> = validate_expr(param, function_entry, global);
            if res.is_err() {
                return res;
            } else {
                param_types.push(res.unwrap());
            }
        }
    }

    log::warn!("called with : {:?}", param_types);
    log::warn!("expected : {:?}", called_function.type_sig().0);

    for (idx, (provided, expected)) in param_types
        .iter()
        .zip(called_function.type_sig().0.iter())
        .enumerate()
    {
        if provided != expected {
            return Err(SemanticError::InvalidParameters(format!("Invalid parameters at index {} used for function {}: line {}. Expected {:?}, but got {:?}", idx, ident_token.lexeme(), ident_token.line_num(), expected, provided)));
        }
    }

    return Ok(called_function.type_sig().1.clone());
}

/// Validates Indice -> idx should be arith expr returning an integer
fn validate_indices(
    ident_token: &Token,
    indices: &Vec<Node>,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<(), SemanticError> {
    for indice in indices {
        let res = validate_arith_expr(&indice.children()[0], function_entry, global);
        if res.is_err() {
            return Err(res.unwrap_err());
        } else {
            let res = res.unwrap();
            if res != Integer {
                return Err(SemanticError::TypeMistmatch(format!(
                    "Expected integer array index. Got {:?} instead: line {}",
                    res,
                    ident_token.line_num()
                )));
            }
        }
    }

    Ok(())
}

/// Validate an If statement -> {rel expr, stat block, stat block}
fn validate_if_statement(
    if_statement: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<(), SemanticError> {
    log::warn!("validating if statement");
    // validate rel expr
    let rel_expr_res: Result<Type, SemanticError> =
        validate_rel_expr(&if_statement.children()[0], function_entry, global);
    if rel_expr_res.is_err() {
        return Err(rel_expr_res.unwrap_err());
    }
    let rel_expr_res = rel_expr_res.unwrap();
    if rel_expr_res != Integer {
        return Err(SemanticError::TypeMistmatch(format!(
            "If condition expected integer. Got {:?} instead: in {}",
            rel_expr_res,
            function_entry.ident()
        )));
    }

    // validate statblock 1 (then)
    for statement in if_statement.children()[1].children() {
        let res = validate_statement(statement, function_entry, global);
        if res.is_err() {
            return res;
        } else {
            continue;
        }
    }

    // validate statblock 2 (else)
    for statement in if_statement.children()[2].children() {
        let res = validate_statement(statement, function_entry, global);
        if res.is_err() {
            return res;
        } else {
            continue;
        }
    }

    Ok(())
}

/// Validates a While statement -> {rel expr, stat block)
fn validate_while_statement(
    while_statement: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<(), SemanticError> {
    log::warn!("validating while statement");
    // validate rel expr
    let rel_expr_res: Result<Type, SemanticError> =
        validate_rel_expr(&while_statement.children()[0], function_entry, global);
    if rel_expr_res.is_err() {
        return Err(rel_expr_res.unwrap_err());
    }
    let rel_expr_res = rel_expr_res.unwrap();
    if rel_expr_res != Integer {
        return Err(SemanticError::TypeMistmatch(format!(
            "While condition expected integer. Got {:?} instead: in {}",
            rel_expr_res,
            function_entry.ident()
        )));
    }

    // validate statblock
    for statement in while_statement.children()[1].children() {
        let res = validate_statement(statement, function_entry, global);
        if res.is_err() {
            return res;
        } else {
            continue;
        }
    }

    Ok(())
}

fn validate_read_statement(
    _read_statement: &Node,
    _function_entry: &FunctionEntry,
    _global: &SymbolTable,
) -> Result<(), SemanticError> {
    // validate variable
    //<variable> ::= #MakeFamilyRootNode("Variable") MakeTerminalNode 'id' <variableAmb1> #AddChild
    //<variableAmb1> ::= <rept-variable> #MakeTerminalNode '.' MakeTerminalNode 'id'  #MakeRelativeOperation <variableAmb1>
    //<variableAmb1> ::= '(' <params> #AddChild ')' #MakeTerminalNode '.' #MakeTerminalNode 'id'  #MakeRelativeOperation <variableAmb1>
    //<variableAmb1> ::= EPSILON

    log::error!("VALIDATE READ STATEMENT: NOT IMPLEMENTED");
    return Ok(());
}

fn validate_write_statement(
    write_statement: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<(), SemanticError> {
    log::warn!("validating write statement");
    //validate expr
    let res: Result<Type, SemanticError> =
        validate_expr(&write_statement.children()[0], function_entry, global);

    if res.is_err() {
        Err(res.unwrap_err())
    } else {
        Ok(())
    }
}

fn validate_return_statement(
    return_statement: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<(), SemanticError> {
    log::warn!("validating return statement");
    // validate expr
    let res: Result<Type, SemanticError> =
        validate_expr(&return_statement.children()[0], function_entry, global);
    if res.is_err() {
        return Err(res.unwrap_err());
    }
    let res = res.unwrap();

    return if res != function_entry.type_sig().1 {
        Err(SemanticError::TypeMistmatch(format!(
            "Return Statement: function {} expected {:?}, but got {:?} instead",
            function_entry.ident(),
            function_entry.type_sig().1,
            res
        )))
    } else {
        Ok(())
    };
}

fn validate_dot_operator(
    dot_op: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<Type, SemanticError> {
    log::warn!("validating dot operator");
    let lhs = match dot_op.children()[0].val() {
        Some(NodeVal::Leaf(token)) => {
            // We have an identifier that should be present in the current function scope
            validate_ident(token, &dot_op.children()[0], function_entry, global)
        }
        Some(NodeVal::Internal(InternalNodeType::DotOp)) => {
            validate_dot_operator(&dot_op.children()[0], function_entry, global)
        }
        _ => {
            panic!()
        }
    };

    if lhs.is_err() {
        return lhs;
    }
    let lhs = lhs.unwrap();

    let rhs_token = match dot_op.children()[0].val() {
        Some(NodeVal::Leaf(token)) => token,
        _ => {
            panic!()
        }
    };

    match lhs {
        Type::Custom(class_ident) => {
            if let Some(Scope::Class(ce)) = global.find_scope_by_ident(&class_ident) {
                match ce.table().find_scope_by_ident(rhs_token.lexeme()) {
                    Some(Scope::Function(_)) => {
                        if dot_op.children().len() < 3 {
                            return Err(SemanticError::InvalidParameters(format!(
                                "No function matched parameters for {}: line {}",
                                rhs_token.lexeme(),
                                rhs_token.line_num()
                            )));
                        } else if dot_op.children().len() == 3 {
                            if dot_op.children()[2].val()
                                != Some(&NodeVal::Internal(InternalNodeType::FuncCallParams))
                            {
                                return Err(SemanticError::InvalidParameters(format!(
                                    "No function matched parameters for {}: line {}",
                                    rhs_token.lexeme(),
                                    rhs_token.line_num()
                                )));
                            }
                        } else {
                            return Err(SemanticError::NotIndexable(format!(
                                "Function {} is not indexable: line {}",
                                rhs_token.lexeme(),
                                rhs_token.line_num()
                            )));
                        }

                        let (function_candidates, _errors) =
                            get_class_hierarchy_functions(ce, global);
                        for function in function_candidates {
                            let res = validate_function_call(
                                rhs_token,
                                &dot_op.children()[2],
                                function,
                                function_entry,
                                global,
                            );
                            if res.is_ok() {
                                return Ok(function.type_sig().1.clone());
                            }
                        }
                        return Err(SemanticError::InvalidParameters(format!(
                            "No function matched parameters for {}: line {}",
                            rhs_token.lexeme(),
                            rhs_token.line_num()
                        )));
                    }
                    Some(Scope::Variable(cve)) => {
                        return if dot_op.children().len() < 3 {
                            Ok(cve.var_type().clone())
                        } else if dot_op.children()[2].val()
                            == Some(&NodeVal::Internal(InternalNodeType::FuncCallParams))
                        {
                            Err(SemanticError::NotCallable(format!(
                                "Data member {} is not callabled: line {}",
                                rhs_token.lexeme(),
                                rhs_token.line_num()
                            )))
                        } else {
                            let indices: &Vec<Node> = &dot_op.children()[2..].into();
                            let indices_res =
                                validate_indices(rhs_token, &indices, function_entry, global);
                            if indices_res.is_err() {
                                return Err(indices_res.unwrap_err());
                            }

                            if indices.len() > 0 {
                                match cve.var_type() {
                                    Type::IntegerArray(dim)
                                    | Type::FloatArray(dim)
                                    | Type::StringArray(dim)
                                    | Type::CustomArray(_, dim) => {
                                        if dim.len() < indices.len() {
                                            Err(SemanticError::TooManyIndices(format!(
                                                "Found {} indices, but dimension is {}: line {}",
                                                indices.len(),
                                                dim.len(),
                                                rhs_token.line_num()
                                            )))
                                        } else if dim.len() == indices.len() {
                                            Ok(cve.var_type().to_simple_type())
                                        } else {
                                            Ok(cve
                                                .var_type()
                                                .to_array_type(dim[indices.len()..].to_vec()))
                                        }
                                    }
                                    ty => Err(SemanticError::NotIndexable(format!(
                                        "Cannot index variable of type {:?}: line {}",
                                        ty,
                                        rhs_token.line_num()
                                    ))),
                                }
                            } else {
                                Ok(cve.var_type().clone())
                            }
                        }
                    }
                    _ => panic!(),
                }
            } else {
                return Err(SemanticError::UndeclaredClass(format!(
                    "Undeclared class {}: line {}",
                    class_ident,
                    rhs_token.line_num()
                )));
            }
        }
        ty => {
            return Err(SemanticError::NotClassType(format!(
                "{:?} is not a class type: line {}",
                ty,
                rhs_token.line_num()
            )));
        }
    }
}

/// Validates an Expr
fn validate_expr(
    expr: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<Type, SemanticError> {
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
    log::warn!("validating expr");
    let res: Result<Type, SemanticError> = match expr.children()[0].val() {
        Some(NodeVal::Internal(InternalNodeType::Equal))
        | Some(NodeVal::Internal(InternalNodeType::NotEqual))
        | Some(NodeVal::Internal(InternalNodeType::LessThan))
        | Some(NodeVal::Internal(InternalNodeType::GreaterThan))
        | Some(NodeVal::Internal(InternalNodeType::LessEqualThan))
        | Some(NodeVal::Internal(InternalNodeType::GreaterEqualThan)) => {
            let rel_op = &expr.children()[0];
            let lhs_res = validate_arith_expr(&rel_op.children()[0], function_entry, global);
            let rhs_res = validate_arith_expr(&rel_op.children()[1], function_entry, global);
            if lhs_res.is_err() {
                lhs_res
            } else if rhs_res.is_err() {
                rhs_res
            } else {
                let lhs_res = lhs_res.unwrap();
                let rhs_res = rhs_res.unwrap();
                if lhs_res != rhs_res {
                    Err(SemanticError::TypeMistmatch(format!(
                        "Type Mistmatch in relative operation: {:?} {:?} {:?}: line {}",
                        lhs_res,
                        rel_op.val(),
                        rhs_res,
                        999
                    )))
                } else {
                    Ok(Integer) // In relative operations, the resulting type is an integer (where anything but 0 is true)
                }
            }
        }
        Some(NodeVal::Internal(InternalNodeType::ArithExpr)) => {
            let arith_res = validate_arith_expr(&expr.children()[0], function_entry, global);
            arith_res
        }
        _ => {
            panic!("failed match expr for {:?}", expr.children()[0].val())
        }
    };

    return res;
}

///Validates a Rel Expr
fn validate_rel_expr(
    rel_expr: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<Type, SemanticError> {
    log::warn!("validating rel expr");
    let lhs_res = validate_arith_expr(
        &rel_expr.children()[0].children()[0],
        function_entry,
        global,
    );
    if lhs_res.is_err() {
        return lhs_res;
    }
    let lhs_res = lhs_res.unwrap();

    let rhs_res = validate_arith_expr(
        &rel_expr.children()[0].children()[1],
        function_entry,
        global,
    );
    if rhs_res.is_err() {
        return rhs_res;
    }
    let rhs_res = rhs_res.unwrap();

    if lhs_res != rhs_res {
        return Err(SemanticError::TypeMistmatch(format!(
            "Type mistmatch in rel expr -> {:?} {:?} {:?}",
            lhs_res,
            rel_expr.val(),
            rhs_res
        )));
    }

    return Ok(lhs_res);
}

/// Validate an Arith Expr
fn validate_arith_expr(
    arith_expr: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<Type, SemanticError> {
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
    log::warn!("validating arith expr");
    let arith_expr_res: Result<Type, SemanticError> = match arith_expr.children()[0].val() {
        Some(NodeVal::Internal(InternalNodeType::Add))
        | Some(NodeVal::Internal(InternalNodeType::Sub))
        | Some(NodeVal::Internal(InternalNodeType::Or)) => {
            validate_add_op(&arith_expr.children()[0], function_entry, global)
        }
        Some(NodeVal::Internal(InternalNodeType::Term)) => {
            validate_term(&arith_expr.children()[0], function_entry, global)
        }
        _ => {
            panic!("Failed match arith expr")
        }
    };

    return arith_expr_res;
}

/// Validates a Term
fn validate_term(
    term: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<Type, SemanticError> {
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
    log::warn!("validating term");
    let term_res: Result<Type, SemanticError> = match term.children()[0].val() {
        Some(NodeVal::Internal(InternalNodeType::Mult))
        | Some(NodeVal::Internal(InternalNodeType::Div))
        | Some(NodeVal::Internal(InternalNodeType::And)) => {
            validate_mult_op(&term.children()[0], function_entry, global)
        }
        Some(NodeVal::Leaf(token)) => Ok(map_token_to_type(token)),
        Some(NodeVal::Internal(InternalNodeType::Factor)) => {
            validate_ident_factor(&term.children()[0], function_entry, global)
        } // Factor node has id child
        Some(NodeVal::Internal(InternalNodeType::ArithExpr)) => {
            validate_arith_expr(&term.children()[0], function_entry, global)
        }
        Some(NodeVal::Internal(InternalNodeType::SignedFactor)) => {
            match term.children()[0].children()[1].val() {
                Some(NodeVal::Internal(InternalNodeType::Factor)) => {
                    validate_ident_factor(&term.children()[0].children()[1], function_entry, global)
                }
                Some(NodeVal::Internal(InternalNodeType::ArithExpr)) => {
                    validate_arith_expr(&term.children()[0], function_entry, global)
                }
                Some(NodeVal::Leaf(token)) => Ok(map_token_to_type(token)),
                _ => {
                    panic!()
                }
            }
        }
        Some(NodeVal::Internal(InternalNodeType::Negation)) => {
            // Factor, token, arith expr
            match term.children()[0].children()[0].val() {
                Some(NodeVal::Internal(InternalNodeType::Factor)) => {
                    validate_ident_factor(&term.children()[0].children()[1], function_entry, global)
                }
                Some(NodeVal::Internal(InternalNodeType::ArithExpr)) => {
                    validate_arith_expr(&term.children()[0], function_entry, global)
                }
                Some(NodeVal::Leaf(token)) => Ok(map_token_to_type(token)),
                _ => {
                    panic!()
                }
            }
        }
        Some(NodeVal::Internal(InternalNodeType::TernaryOperation)) => {
            validate_ternary_operation(&term.children()[0], function_entry, global)
        }
        _ => {
            panic!("Failed match term")
        }
    };

    return term_res;
}

fn validate_ident_factor(
    ident_factor: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<Type, SemanticError> {
    /*
    <factor> ::= #MakeFamilyRootNode("Factor") #MakeTerminalNode 'id' <factorAmb1> #AddChild
    <factorAmb1> ::= <rept-variable> <factorAmb2>
    <factorAmb1> ::= '(' <params> ')' #AddChild <factorAmb2>
    <factorAmb2> ::= #MakeTerminalNode '.' #MakeTerminalNode 'id' #MakeRelativeOperation <factorAmb1>
    <factorAmb2> ::= EPSILON
     */
    log::warn!("validating ident factor");
    match ident_factor.children()[0].val() {
        Some(NodeVal::Leaf(token)) => {
            return validate_ident(token, &ident_factor.children()[0], function_entry, global);
        }
        Some(NodeVal::Internal(InternalNodeType::DotOp)) => {
            return validate_dot_operator(&ident_factor.children()[0], function_entry, global);
        }
        _ => {
            panic!()
        }
    }
}

/// Essentially validates starting from an ident. Can be a free function call or array indexing
fn validate_ident(
    ident_token: &Token,
    ident_node: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<Type, SemanticError> {
    log::warn!("validating ident");
    if let Some(Scope::FunctionParameter(e)) = function_entry
        .table()
        .find_scope_by_ident(ident_token.lexeme())
    {
        if ident_node.children().len() > 0 {
            match ident_node.children()[0].val() {
                Some(NodeVal::Internal(InternalNodeType::FuncCallParams)) => {
                    return Err(SemanticError::NotCallable(format!(
                        "Function Param {} is not callable: line {}",
                        e.ident(),
                        ident_token.line_num()
                    )));
                }
                Some(NodeVal::Internal(InternalNodeType::Indice)) => {
                    return match e.param_type() {
                        Type::IntegerArray(dim)
                        | Type::FloatArray(dim)
                        | Type::StringArray(dim)
                        | Type::CustomArray(_, dim) => {
                            if dim.len() < ident_node.children().len() {
                                return Err(SemanticError::TooManyIndices(format!("Tried indexing param {} with {} indices, but it only has {} dimensions: line {}", e.ident(), ident_node.children().len(), dim.len(), ident_token.line_num())));
                            }
                            let idx_res = validate_indices(
                                ident_token,
                                &ident_node.children(),
                                function_entry,
                                global,
                            );
                            if idx_res.is_err() {
                                return Err(idx_res.unwrap_err());
                            }
                            if dim.len() == ident_node.children().len() {
                                Ok(e.param_type().to_simple_type())
                            } else {
                                Ok(e.param_type()
                                    .to_array_type(dim[ident_node.children().len()..].to_vec()))
                            }
                        }
                        _ => Err(SemanticError::NotIndexable(format!(
                            "Func Param {} is not indexable: line {}",
                            e.ident(),
                            ident_token.line_num()
                        ))),
                    }
                }
                _ => {
                    panic!("Did not expect these children() to id token")
                }
            }
        } else {
            return Ok(e.param_type().clone());
        }
    } else if let Some(Scope::Variable(e)) = function_entry
        .table()
        .find_scope_by_ident(ident_token.lexeme())
    {
        if ident_node.children().len() > 0 {
            match ident_node.children()[0].val() {
                Some(NodeVal::Internal(InternalNodeType::FuncCallParams)) => {
                    return Err(SemanticError::NotCallable(format!(
                        "Function Param {} is not callable: line {}",
                        e.ident(),
                        ident_token.line_num()
                    )));
                }
                Some(NodeVal::Internal(InternalNodeType::Indice)) => {
                    return match e.var_type() {
                        Type::IntegerArray(dim)
                        | Type::FloatArray(dim)
                        | Type::StringArray(dim)
                        | Type::CustomArray(_, dim) => {
                            if dim.len() < ident_node.children().len() {
                                return Err(SemanticError::TooManyIndices(format!("Tried indexing param {} with {} indices, but it only has {} dimensions: line {}", e.ident(), ident_node.children().len(), dim.len(), ident_token.line_num())));
                            }
                            let idx_res = validate_indices(
                                ident_token,
                                &ident_node.children(),
                                function_entry,
                                global,
                            );
                            if idx_res.is_err() {
                                return Err(idx_res.unwrap_err());
                            }
                            if dim.len() == ident_node.children().len() {
                                Ok(e.var_type().to_simple_type().clone())
                            } else {
                                Ok(e.var_type()
                                    .to_array_type(dim[ident_node.children().len()..].to_vec()))
                            }
                        }
                        _ => Err(SemanticError::NotIndexable(format!(
                            "Func Param {} is not indexable: line {}",
                            e.ident(),
                            ident_token.line_num()
                        ))),
                    }
                }
                _ => {
                    panic!("Did not expect these children() to id token")
                }
            }
        } else {
            return Ok(e.var_type().clone());
        }
    } else if let Some(Scope::Function(e)) = global.find_scope_by_ident(ident_token.lexeme()) {
        return if ident_node.children()[0].val()
            == Some(&NodeVal::Internal(InternalNodeType::FuncCallParams))
        {
            // return validate_function_call(ident_token, &ident_node.children()[0], function_entry, global);
            validate_function_call(
                ident_token,
                &ident_node.children()[0],
                e,
                function_entry,
                global,
            )
        } else {
            Err(SemanticError::InvalidParameters(format!(
                "Function call parameters for {} not found.",
                ident_token.lexeme()
            )))
        };
    } else if function_entry.member_of().is_some() {
        return if let Some(Scope::Class(ce)) =
            global.find_scope_by_ident(function_entry.member_of().unwrap())
        {
            if ident_node.children().len() > 0
                && ident_node.children()[0].val()
                    == Some(&NodeVal::Internal(InternalNodeType::FuncCallParams))
            {
                let (member_functions, mut errors) = get_class_hierarchy_functions(ce, global);
                if errors.len() > 0 {
                    return Err(errors.pop().unwrap());
                }
                for member_function in member_functions {
                    let res = validate_function_call(
                        ident_token,
                        &ident_node.children()[0],
                        member_function,
                        function_entry,
                        global,
                    );
                    if res.is_ok() {
                        return res;
                    }
                }
                Err(SemanticError::InvalidParameters(format!(
                    "No function matched parameters for {}: line {}",
                    ident_token.lexeme(),
                    ident_token.line_num()
                )))
            } else if let Some(Scope::Variable(cve)) =
                ce.table().find_scope_by_ident(ident_token.lexeme())
            {
                if ident_node.children().len() > 0 {
                    let indices = ident_node.children();
                    let indices_res =
                        validate_indices(ident_token, indices, function_entry, global);
                    if indices_res.is_err() {
                        return Err(indices_res.unwrap_err());
                    }
                    match cve.var_type() {
                        Type::IntegerArray(dim)
                        | Type::FloatArray(dim)
                        | Type::StringArray(dim)
                        | Type::CustomArray(_, dim) => {
                            if dim.len() < indices.len() {
                                return Err(SemanticError::TooManyIndices(format!("Tried indexing param {} with {} indices, but it only has {} dimensions: line {}", cve.ident(), indices.len(), dim.len(), ident_token.line_num())));
                            }

                            if dim.len() == indices.len() {
                                Ok(cve.var_type().to_simple_type())
                            } else {
                                Ok(cve
                                    .var_type()
                                    .to_array_type(dim[ident_node.children().len()..].to_vec()))
                            }
                        }
                        _ => Err(SemanticError::NotIndexable(format!(
                            "Member {} is not indexable: line {}",
                            cve.ident(),
                            ident_token.line_num()
                        ))),
                    }
                } else {
                    Ok(cve.var_type().clone())
                }
            } else {
                Err(SemanticError::UndeclaredVariable(format!(
                    "Undeclared variable {}: line {}",
                    ident_token.lexeme(),
                    ident_token.line_num()
                )))
            }
        } else {
            Err(SemanticError::UndeclaredClass(format!(
                "Class {} not found: line {}",
                function_entry.member_of().unwrap(),
                ident_token.line_num()
            )))
        };
    } else {
        return Err(SemanticError::UndeclaredVariable(format!(
            "Undeclared variable {}: line {}",
            ident_token.lexeme(),
            ident_token.line_num()
        )));
    }
}

/// Validates an add_op (+, -, or)
fn validate_add_op(
    op_node: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<Type, SemanticError> {
    // lhs might be a term, or another add op
    // rhs is a term
    log::warn!("validating add/sub/or operator");
    let lhs_res = match op_node.children()[0].val() {
        Some(NodeVal::Internal(InternalNodeType::Term)) => {
            validate_term(&op_node.children()[0], function_entry, global)
        }
        Some(NodeVal::Internal(InternalNodeType::Add))
        | Some(NodeVal::Internal(InternalNodeType::Sub))
        | Some(NodeVal::Internal(InternalNodeType::Or)) => {
            validate_add_op(&op_node.children()[0], function_entry, global)
        }
        _ => {
            panic!("Failed to match child of add op")
        }
    };
    if lhs_res.is_err() {
        return lhs_res;
    }
    let lhs_res = lhs_res.unwrap();

    let rhs_res = validate_term(&op_node.children()[1], function_entry, global);
    if rhs_res.is_err() {
        return rhs_res;
    }
    let rhs_res = rhs_res.unwrap();

    if lhs_res != rhs_res {
        return Err(SemanticError::TypeMistmatch(format!(
            "Type Mistmatch in add op: lhs {:?}, rhs {:?}: line {}",
            lhs_res, rhs_res, 888
        )));
    } else {
        return Ok(lhs_res);
    }
}

/// Validates a mult_op (*, /, and)
fn validate_mult_op(
    op_node: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<Type, SemanticError> {
    // lhs might be a factor, or another mult op
    // rhs is a factor or token
    log::warn!("validating mult/div/and operator");
    let lhs_res = match op_node.children()[0].val() {
        Some(NodeVal::Internal(InternalNodeType::Factor)) => {
            validate_ident_factor(&op_node.children()[0], function_entry, global)
        }
        Some(NodeVal::Internal(InternalNodeType::Mult))
        | Some(NodeVal::Internal(InternalNodeType::Div))
        | Some(NodeVal::Internal(InternalNodeType::And)) => {
            validate_mult_op(&op_node.children()[0], function_entry, global)
        }
        Some(NodeVal::Leaf(token)) => match token.token_type() {
            TokenType::Id => validate_ident(token, &op_node.children()[1], function_entry, global),
            TokenType::IntegerLit => Ok(Type::Integer),
            TokenType::FloatLit => Ok(Type::Float),
            _ => panic!(),
        },
        _ => {
            log::error!("{:?}", op_node.children()[0]);
            panic!("Failed to match child of mult op")
        }
    };
    if lhs_res.is_err() {
        return lhs_res;
    }
    let lhs_res = lhs_res.unwrap();

    let rhs_res = match op_node.children()[1].val() {
        Some(NodeVal::Internal(InternalNodeType::Factor)) => {
            todo!()
        }
        Some(NodeVal::Leaf(token)) => match token.token_type() {
            TokenType::IntegerLit => Ok(Integer),
            TokenType::FloatLit => Ok(Float),
            TokenType::Id => validate_ident(token, &op_node.children()[1], function_entry, global),
            _ => {
                panic!()
            }
        },
        _ => {
            panic!()
        }
    };
    if rhs_res.is_err() {
        return rhs_res;
    }
    let rhs_res = rhs_res.unwrap();

    if lhs_res != rhs_res {
        return Err(SemanticError::TypeMistmatch(format!(
            "Type Mistmatch in mult op: lhs {:?}, rhs {:?}: line {}",
            lhs_res, rhs_res, 888
        )));
    } else {
        return Ok(lhs_res);
    }
}

/// Validates a ternary operation
fn validate_ternary_operation(
    ter: &Node,
    function_entry: &FunctionEntry,
    global: &SymbolTable,
) -> Result<Type, SemanticError> {
    log::warn!("validating ternary operator");
    //validate 3 expressions
    let cond_expr = validate_expr(&ter.children()[0], function_entry, global);
    if cond_expr.is_err() {
        return cond_expr;
    }
    let cond_expr = cond_expr.unwrap();

    let then_expr = validate_expr(&ter.children()[1], function_entry, global);
    if then_expr.is_err() {
        return then_expr;
    }
    let then_expr = then_expr.unwrap();

    let else_expr = validate_expr(&ter.children()[2], function_entry, global);
    if else_expr.is_err() {
        return else_expr;
    }
    let else_expr = else_expr.unwrap();

    if cond_expr != Integer {
        return Err(SemanticError::TypeMistmatch(format!(
            "Expected integer for condition of ternary operation. Got {:?} instead",
            cond_expr
        )));
    }

    if then_expr != else_expr {
        return Err(SemanticError::TypeMistmatch(format!(
            "Mistmatched ternary op types -> {:?} : {:?}",
            then_expr, else_expr
        )));
    }

    return Ok(then_expr);
}

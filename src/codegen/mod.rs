mod allocator;
pub mod generator;
mod utils;
mod instruction_set;

/*
public void visit(VarDeclNode node){
  // First, propagate accepting the same visitor to all the children
  // This effectively achieves Depth-First AST Traversal
  for (Node child : node.getChildren() )
    child.accept(this);
  // Then, do the processing of this nodes' visitor
  if (node.getChildren().get(0).getData() == "int")
    moonDataCode += "        % space for variable " + node.getChildren().get(1).getData() + "\n";
    moonDataCode += String.format("%-7s" ,node.getChildren().get(1).getData()) + " res 4\n";
}



/// Given the AST and the SymbolTable
/// 1 - Start at the root of the AST
/// 2 - Check the type of the current node. If we should, visit the children of the current node.
/// 3 - Generate code based on the current node



//

 */
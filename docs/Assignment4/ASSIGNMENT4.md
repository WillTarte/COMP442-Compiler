# Assignment 4 Document
## Implementation Checklist
  - Memory Allocation
    - [X] Basic types
    - [X] Basic arrays
    - [X] Objects
    - [X] Object Arrays
  - Functions
    - [X] Branch in/out
    - [X] Pass parameters
    - [X] Return value
    - [ ] Member functions
  - Statements
    - [X] Assignment statement
    - [X] Conditional statement
    - [X] Loop statement
    - [~] Input/output statements
  - Array Indexing
    - [X] For basic type arrays
    - [ ] For Objects arrays
  - Expressions
    - [X] Compute complex expressions (arithmetics, free function calls, array indexing)
      - Dot operator was not implemented and will crash the compiler
    - [X] Array indexing with expression
    - [ ] Object factor referring to object member
### Implementation notes
  - Floating point arithmetics are incorrect (I convert them to their u32 representation and treat them as integers)
  - Dot operator is not implemented
___
## Design
Starting with `instruction_set.rs`, I have an `Instruction` enum that represents the instruction set of the MOON VM.
I have also encoded all 16 registers in the `Register` enum.
Finally, I encode my possible instructions in the `TaggedInstruction` struct since every instruction can optionally have a tag/label.

In the `allocator.rs` file, I have different allocators used during the code generation.
`LabelAllocator` generates labels for while loops, if statements (includes ternary ops) and temporary variables.
`RegisterAllocator` allocates/frees registers during code generation.

In the `utils.rs` file I have some utilities. Notably, the `sizeof` function to calculate the size of variables.
There's also the code to compute arithmetic expressions into a postfix expression.

Finally, the actual code generation happens in the `generator.rs` file.
The `CodegenOutput` struct holds a list of instructions, and can easily be iterated over.
The `CodeGenerator` struct holds `CodegenOutput`s for actual instructions, and allocated resources. It allows easy adding of instructions/resources to the output.
The `ExprParseStorage` enum represents the different possible ways to store values (Immediate values like integers, floats; Values store in program memory; Registers; Pointers (for arrays)).
The `MoonGenerator` is the struct that generates the code. It takes in an AST and a symbol table, and generates the moon assembly code.
I do not use the visitor pattern. I'd say the code is more patterned like recursive pattern matching. For each function + main, I do some preliminary setup (align, entry, res) and then I loop through each statement in the body, generating code.

### Phase
  - Lexing(file) -> token stream
  - Parsing(token stream) -> Abstract Syntax Tree
  - Symbol Generation (AST) -> Symbol Table + basic type/identifier checking
  - Semantic Checking(Symbol Table) -> Expression/Statement checking
  - Code generation (AST, Symbol Table) -> MOON code
___
## Use of tools
See previous assignments

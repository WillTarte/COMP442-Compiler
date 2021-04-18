##
For class declarations:
  - Generate code for member functions:
    - Class data members should be copied & stored?
    - Parse parameter expressions and allocate
    - Parse local var declarations and allocate
    - Allocate for return value
    - Jump instruction to address of first statement of of the function
    - Generate code for body
  - Generate code for free functions:
    - See previous, except no class data members
  - For var declarations:
    - Allocate local variable with proper label
___
### Implemented
  - Generate Function Code
    - [X] Allocate for parameters, return value
    - [X] Handle function call statement (no recursion)
    - [X] Handle return statement
    - [X] Handle while statement
    - [X] handle if statement
    - [X] handle break/continue statements
    - [ ] Handle read/write statements
    - [X] Handle assignment statement
    - [ ] Handle array indexing
### Not implemented
    - Correct floating point arithmetics
    - Class semantics (can allocate, but won't compile if they are used)
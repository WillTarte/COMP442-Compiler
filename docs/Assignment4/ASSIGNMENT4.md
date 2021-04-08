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
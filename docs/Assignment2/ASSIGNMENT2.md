# Assignment 2 Document
## Work to submit
    - Report
        - [ ] LL(1) transformed gramnmar
            - [ ] Remove all EBNF repetition and optionality constructs (grammar tool)
            - [ ] Replace all left recursion by right recursion (grammar tool)
            - [ ] Remove all ambiguities (ucalgary tool + by hand)
        - [ ] First & Follow sets (grammar tool)
        - [ ] Design
        - [ ] Use of tools
    - Implementation
        - [ ] Parser (recursive descent or table driven)
        - [ ] Derivation output
        - [ ] AST output
        - [ ] Error reporting
        - [ ] Error recovery
        - [ ] Test cases
        - [ ] Driver

## Steps to transform grammar to LL(1)
    - Run it through the grammar tool to:
        - Remove optionality constructs
        - Remove repetitions (0 or more and 1 or more)
        - Remove left recursion
        - Generate UCalgary version for further processing
    - Remove ambiguities:

The grammar is not LL(1) because:
ARRAYSIZE has a first set conflict. [X] Remove ambiguity
EXPR has a first set conflict.
FACTOR has a first set conflict.
FUNCDECL has a first set conflict.
FUNCHEAD has a first set conflict.
IDNEST has a first set conflict.
OPTFUNCHEAD1 is nullable with clashing first and follow sets.
REPTFUNCTIONCALL0 is nullable with clashing first and follow sets.
REPTVARIABLE0 is nullable with clashing first and follow sets.
STATEMENT has a first set conflict.
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
        - First set clashes & Factorizations

https://www.cs.bgu.ac.il/~comp171/wiki.files/ps5.pdf
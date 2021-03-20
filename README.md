# COMP442-Compiler
## Author: William Tarte 40087900
## Submitted to: Professor Joey Paquet in the context of COMP 442: Compiler Design Winter 2021
___
## Assignment 1 - Lexical Analyzer
### Work to submit:
 - Document
    - [X] Lexical Specification
    - [X] Finite State Automaton
    - [X] Design
    - [X] Use of tools
 - Implementation
    - [X] Lexical Analyzer
    - [X] Test Cases
    - [X] Driver
## Assignment 2 - Parser
### Work to submit:
- Report
  - [X] LL(1) transformed grammar
  - [X] Remove all EBNF repetition and optionality constructs (grammar tool)
  - [X] Replace all left recursion by right recursion (grammar tool)
  - [X] Remove all ambiguities (ucalgary tool + by hand)
  - [X] First & Follow sets (grammar tool)
  - [X] Design
  - [X] Use of tools
   - Implementation
      - [X] Parser (recursive descent or table driven)
      - [X] Derivation output
      - [X] AST output
      - [X] Error reporting
      - [X] Error recovery
      - [X] Test cases
      - [X] Driver

### How to Run
  - Requires Rust 2018 Edition, which can be installed from  https://www.rust-lang.org/
  - From the command line `cargo run -- --file <file> [--lexer | --parser]`
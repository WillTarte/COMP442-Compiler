if __name__ == "__main__":

    table = """
|       START        |           | Endable  |                      class, func, main                      |                                                       $                                                        |
|       ADDOP        |           |          |                       plus, minus, or                       |                          plus, minus, id, intlit, floatlit, stringlit, lpar, not, qm                           |
|     ARITHEXPR      |           |          | plus, minus, id, intlit, floatlit, stringlit, lpar, not, qm |                           rsqbr, semi, rpar, colon, eq, neq, lt, gt, leq, geq, comma                           |
|     ARRAYSIZE      |           |          |                            lsqbr                            |                                            lsqbr, semi, rpar, comma                                            |
|   ARRAYSIZEAMB1    |           |          |                        intnum, rsqbr                        |                                            lsqbr, semi, rpar, comma                                            |
|      ASSIGNOP      |           |          |                            equal                            |                          plus, minus, id, intlit, floatlit, stringlit, lpar, not, qm                           |
|     CLASSDECL      |           |          |                            class                            |                                               class, func, main                                                |
|        EXPR        |           |          | plus, minus, id, intlit, floatlit, stringlit, lpar, not, qm |                                        rsqbr, semi, rpar, colon, comma                                         |
|      EXPRAMB1      | Nullable  |          |                  eq, neq, lt, gt, leq, geq                  |                                        rsqbr, semi, rpar, colon, comma                                         |
|       FACTOR       |           |          | plus, minus, id, intlit, floatlit, stringlit, lpar, not, qm |          plus, minus, or, rsqbr, semi, rpar, colon, mult, div, and, eq, neq, lt, gt, leq, geq, comma           |
|     FACTORAMB1     | Nullable  |          |                      lsqbr, lpar, dot                       |          plus, minus, or, rsqbr, semi, rpar, colon, mult, div, and, eq, neq, lt, gt, leq, geq, comma           |
|     FACTORAMB2     | Nullable  |          |                             dot                             |          plus, minus, or, rsqbr, semi, rpar, colon, mult, div, and, eq, neq, lt, gt, leq, geq, comma           |
|      FUNCBODY      |           | Endable  |                           lcurbr                            |                                                 func, main, $                                                  |
|      FUNCDECL      |           |          |                            func                             |                           id, rcurbr, func, integer, float, string, public, private                            |
|    FUNCDECLAMB1    |           |          |              id, void, integer, float, string               |                           id, rcurbr, func, integer, float, string, public, private                            |
|      FUNCDEF       |           |          |                            func                             |                                                   func, main                                                   |
|      FUNCHEAD      |           |          |                            func                             |                                                     lcurbr                                                     |
|    FUNCHEADAMB1    |           |          |                          lpar, sr                           |                                                     lcurbr                                                     |
|    FUNCHEADAMB2    |           |          |              id, void, integer, float, string               |                                                     lcurbr                                                     |
|     FUNCPARAMS     | Nullable  |          |                 id, integer, float, string                  |                                                      rpar                                                      |
|       INDICE       |           |          |                            lsqbr                            | plus, minus, or, lsqbr, rsqbr, equal, semi, rpar, colon, dot, mult, div, and, eq, neq, lt, gt, leq, geq, comma |
|     MEMBERDECL     |           |          |              id, func, integer, float, string               |                           id, rcurbr, func, integer, float, string, public, private                            |
|       MULTOP       |           |          |                       mult, div, and                        |                          plus, minus, id, intlit, floatlit, stringlit, lpar, not, qm                           |
|    OPTCLASSDECL    | Nullable  |          |                          inherits                           |                                                     lcurbr                                                     |
|    OPTFUNCBODY     | Nullable  |          |                             var                             |                          id, rcurbr, if, while, read, write, return, break, continue                           |
|       PARAMS       | Nullable  |          | plus, minus, id, intlit, floatlit, stringlit, lpar, not, qm |                                                      rpar                                                      |
|        PROG        |           | Endable  |                      class, func, main                      |                                                       $                                                        |
|      RELEXPR       |           |          | plus, minus, id, intlit, floatlit, stringlit, lpar, not, qm |                                                      rpar                                                      |
|       RELOP        |           |          |                  eq, neq, lt, gt, leq, geq                  |                          plus, minus, id, intlit, floatlit, stringlit, lpar, not, qm                           |
|   REPTCLASSDECL    | Nullable  |          |      id, func, integer, float, string, public, private      |                                                     rcurbr                                                     |
|    REPTFUNCBODY    | Nullable  |          |     id, if, while, read, write, return, break, continue     |                                                     rcurbr                                                     |
|  REPTFUNCPARAMS0   | Nullable  |          |                            lsqbr                            |                                                  rpar, comma                                                   |
|  REPTFUNCPARAMS1   | Nullable  |          |                            comma                            |                                                      rpar                                                      |
| REPTFUNCPARAMSTAIL | Nullable  |          |                            lsqbr                            |                                                  rpar, comma                                                   |
|  REPTOPTCLASSDECL  | Nullable  |          |                            comma                            |                                                     lcurbr                                                     |
|  REPTOPTFUNCBODY   | Nullable  |          |                 id, integer, float, string                  |                                                     rcurbr                                                     |
|     REPTPARAMS     | Nullable  |          |                            comma                            |                                                      rpar                                                      |
|     REPTPROG0      | Nullable  |          |                            class                            |                                                   func, main                                                   |
|     REPTPROG1      | Nullable  |          |                            func                             |                                                      main                                                      |
|   REPTSTATBLOCK    | Nullable  |          |     id, if, while, read, write, return, break, continue     |                                                     rcurbr                                                     |
|    REPTVARDECL     | Nullable  |          |                            lsqbr                            |                                                      semi                                                      |
|    REPTVARIABLE    | Nullable  |          |                            lsqbr                            |    plus, minus, or, rsqbr, equal, semi, rpar, colon, dot, mult, div, and, eq, neq, lt, gt, leq, geq, comma     |
| RIGHTRECARITHEXPR  | Nullable  |          |                       plus, minus, or                       |                           rsqbr, semi, rpar, colon, eq, neq, lt, gt, leq, geq, comma                           |
|    RIGHTRECTERM    | Nullable  |          |                       mult, div, and                        |                  plus, minus, or, rsqbr, semi, rpar, colon, eq, neq, lt, gt, leq, geq, comma                   |
|        SIGN        |           |          |                         plus, minus                         |                          plus, minus, id, intlit, floatlit, stringlit, lpar, not, qm                           |
|     STATBLOCK      | Nullable  |          | id, lcurbr, if, while, read, write, return, break, continue |                                                   semi, else                                                   |
|     STATEMENT      |           |          |     id, if, while, read, write, return, break, continue     |                    id, rcurbr, semi, if, else, while, read, write, return, break, continue                     |
|   STATEMENTAMB1    |           |          |                     lsqbr, equal, lpar                      |                    id, rcurbr, semi, if, else, while, read, write, return, break, continue                     |
|   STATEMENTAMB2    |           |          |                         equal, dot                          |                    id, rcurbr, semi, if, else, while, read, write, return, break, continue                     |
|   STATEMENTAMB3    |           |          |                          semi, dot                          |                    id, rcurbr, semi, if, else, while, read, write, return, break, continue                     |
|        TERM        |           |          | plus, minus, id, intlit, floatlit, stringlit, lpar, not, qm |                  plus, minus, or, rsqbr, semi, rpar, colon, eq, neq, lt, gt, leq, geq, comma                   |
|        TYPE        |           |          |                 id, integer, float, string                  |                                                id, lcurbr, semi                                                |
|      VARDECL       |           |          |                 id, integer, float, string                  |                           id, rcurbr, func, integer, float, string, public, private                            |
|      VARIABLE      |           |          |                             id                              |                                                      rpar                                                      |
|    VARIABLEAMB1    | Nullable  |          |                      lsqbr, lpar, dot                       |                                                      rpar                                                      |
|     VISIBILITY     | Nullable  |          |                       public, private                       |                                        id, func, integer, float, string                                        |
"""
    lines = table.split("\n")
    lines = lines[1:]
    for line in lines:
        line = line.replace(" ", "")
        line = line.replace("\t", "")
        line = line[1:]
        cols = line.split("|")
        first_symbols = ""
        #print(cols[3].split(","))
        for symbol in cols[3].split(","):
            first_symbols += "Terminal({0}),".format(symbol)
        follow_symbols = ""
        #print(cols[4].split(","))
        for symbol in cols[4].split(","):
            follow_symbols += "Terminal({0}),".format(symbol)
        print("const {varName}: &'static [GrammarSymbol] = &[{symbols}];".format(varName = cols[0] + "_FIRST", symbols = first_symbols))
        print("const {varName}: &'static [GrammarSymbol] = &[{symbols}];".format(varName = cols[0] + "_FOLLOW", symbols = follow_symbols))
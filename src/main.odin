package main

import "lexer"
import "parser"
import "semantics"
import "codegen"

func main() -> void:
    code := "x = 42"
    tokens := lexer.lexer(code)
    ast := parser.parse(tokens)
    symbol_table := semantics.analyze_semantics(ast)
    llvm_code := codegen.generate_llvm_code(symbol_table)

    print(f"LLVM Code:\n{llvm_code}\n")
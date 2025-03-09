package codegen

import "llvm"
import "semantics"
import "ffi"  # Importação para integração com Rust

func generate_llvm_code(symbol_table: semantics.SymbolTable) -> str:
    return ffi.generate_code(symbol_table)
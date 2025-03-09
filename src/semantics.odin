package semantics

import "parser"

struct Symbol:
    name: string
    value: string

func symbol_exists(symbol_table: []Symbol, name: string) -> bool:
    for symbol in symbol_table:
        if symbol.name == name:
            return true
    return false

func analyze_semantics(ast: []parser.ASTNode) -> []Symbol:
    var symbol_table: []Symbol

    for node in ast:
        if node.node_type == parser.ASTNodeType.Variable:
            if !symbol_exists(symbol_table, node.value):
                symbol_table = append(symbol_table, Symbol{node.value, ""})

    return symbol_table

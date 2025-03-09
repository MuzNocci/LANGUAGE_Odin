package parser

import "lexer"

enum ASTNodeType:
    NumberLiteral, Variable, Operator

struct ASTNode:
    node_type: ASTNodeType
    value: string

func parse(tokens: []lexer.Token) -> []ASTNode:
    var ast: []ASTNode

    for token in tokens:
        if token.type == lexer.TokenType.Identifier:
            ast = append(ast, ASTNode{ASTNodeType.Variable, token.value})
        elif token.type == lexer.TokenType.Number:
            ast = append(ast, ASTNode{ASTNodeType.NumberLiteral, token.value})

    return ast

package lexer

enum TokenType:
    Keyword, Identifier, Number, Operator, String, EndOfFile, Invalid

struct Token:
    type: TokenType
    value: string

func is_whitespace(c: rune) -> bool:
    return c in [' ', '\t', '\n', '\r']

func new_token(t: TokenType, v: string) -> Token:
    return Token{t, v}

func lexer(input: string) -> []Token:
    var tokens: []Token
    var current_value: string

    for c in input:
        if is_whitespace(c):
            continue
        else:
            current_value += string(c)

    if current_value:
        tokens = append(tokens, new_token(TokenType.Identifier, current_value))

    return tokens

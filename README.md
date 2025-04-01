# Odin Compiler

Odin Compiler é um compilador para a linguagem Odin, desenvolvido em Rust.

## Estrutura do Projeto

```plaintext
odin/
├── Cargo.toml                        # Configuração do projeto Rust
├── src/
│   ├── main.rs                       # Ponto de entrada do compilador/interpretador
│   ├── lexer/
│   │   ├── mod.rs                    # Definições do módulo lexer
│   │   ├── token.rs                  # Definições de tokens
│   │   └── lexer.rs                  # Analisador léxico
│   ├── parser/
│   │   ├── mod.rs                    # Definições do módulo parser
│   │   ├── ast.rs                    # Árvore de sintaxe abstrata
│   │   └── parser.rs                 # Analisador sintático
│   ├── interpreter/
│   │   ├── mod.rs                    # Definições do módulo interpreter
│   │   ├── environment.rs            # Ambiente de execução
│   │   ├── evaluator.rs              # Avaliador de expressões
│   │   └── object.rs                 # Representação de objetos
│   ├── compiler/
│   │   ├── mod.rs                    # Definições do módulo compiler
│   │   ├── bytecode.rs               # Definição do bytecode
│   │   └── compiler.rs               # Compilador
│   ├── vm/
│   │   ├── mod.rs                    # Definições do módulo vm
│   │   └── vm.rs                     # Máquina virtual
│   ├── stdlib/
│   │   ├── mod.rs                    # Definições do módulo stdlib
│   │   ├── math.rs                   # Funções matemáticas
│   │   ├── io.rs                     # Funções de entrada/saída
│   │   └── string.rs                 # Funções de manipulação de strings
│   └── repl/
│       ├── mod.rs                    # Definições do módulo repl
│       └── repl.rs                   # REPL (Read-Eval-Print Loop)
├── examples/                         # Exemplos de código na linguagem Odin
│   ├── hello_world.odin              # Exemplo básico "Hello World"
│   ├── fibonacci.odin                # Implementação de Fibonacci
│   └── classes.odin                  # Exemplo de classes e objetos
└── tests/                            # Testes para o compilador/interpretador
    ├── lexer_tests.rs                # Testes para o lexer
    ├── parser_tests.rs               # Testes para o parser
    └── integration_tests.rs          # Testes de integração
```

## Instalação

Para compilar e rodar o Odin Compiler, certifique-se de ter o Rust instalado. Em seguida, clone o repositório e execute:

```sh
cargo build
```

## Uso

Para executar o compilador com um arquivo Odin:

```sh
cargo run -- examples/hello_world.odin
```

## Contribuição

Contribuições são bem-vindas! Siga as diretrizes abaixo:

1. Faça um fork do repositório.
2. Crie uma branch para sua feature (`git checkout -b minha-feature`).
3. Faça commit das mudanças (`git commit -m 'Minha nova feature'`).
4. Faça push para a branch (`git push origin minha-feature`).
5. Abra um Pull Request.

## Licença

Este projeto está sob a licença MIT. Para mais detalhes, consulte o arquivo `LICENSE`.
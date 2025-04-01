# Linguagem de Programação Odin

Odin é uma linguagem de programação que pode ser interpretada ou compilada. Ela foi projetada para ser simples, rápida e eficiente. Este repositório contém o código-fonte do compilador e do interpretador da linguagem Odin.

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
│   │   ├── ast.rs                     # Árvore de Sintaxe Abstrata (AST)
│   │   └── parser.rs                 # Analisador sintático
│   ├── interpreter/
│   │   ├── mod.rs                    # Definições do módulo interpretador
│   │   ├── environment.rs            # Ambiente de execução
│   │   ├── evaluator.rs              # Avaliador de expressões
│   │   └── object.rs                 # Representação de objetos
│   ├── compiler/
│   │   ├── mod.rs                    # Definições do módulo compilador
│   │   ├── bytecode.rs               # Definição do bytecode
│   │   └── compiler.rs               # Compilador
│   ├── vm/
│   │   ├── mod.rs                    # Definições do módulo Máquina Virtual (VM)
│   │   └── vm.rs                     # Máquina Virtual
│   ├── stdlib/
│   │   ├── mod.rs                    # Definições do módulo da biblioteca padrão
│   │   ├── math.rs                   # Funções matemáticas
│   │   ├── io.rs                     # Funções de entrada/saída
│   │   └── string.rs                 # Funções de manipulação de strings
│   └── repl/
│       ├── mod.rs                    # Definições do módulo REPL
│       └── repl.rs                    # Loop de Leitura-Avaliação-Impressão (REPL)
├── examples/                         # Exemplos de programas em Odin
│   ├── hello_world.odin              # Exemplo básico "Hello World"
│   ├── fibonacci.odin                # Implementação de Fibonacci
│   └── classes.odin                  # Exemplo de classes e objetos
└── tests/                            # Testes para o compilador/interpretador
    ├── lexer_tests.rs                # Testes para o lexer
    ├── parser_tests.rs               # Testes para o parser
    └── integration_tests.rs          # Testes de integração
```

## Funcionalidades

- **Análise Léxica**: Tokeniza o código-fonte.
- **Análise Sintática**: Converte o código tokenizado em uma Árvore de Sintaxe Abstrata (AST).
- **Interpretador**: Avalia o código Odin diretamente.
- **Compilador**: Compila o código Odin para uma representação intermediária (bytecode).
- **Máquina Virtual**: Executa o bytecode compilado.
- **Biblioteca Padrão**: Fornece utilitários embutidos para matemática, entrada/saída e manipulação de strings.
- **REPL**: Permite a execução interativa do código Odin.

## Como Começar

### Pré-requisitos
- Rust instalado (versão estável mais recente recomendada)

### Compilar e Executar
```sh
# Clonar o repositório
git clone https://github.com/your-repo/odin-lang.git
cd odin-lang

# Compilar o projeto
cargo build --release

# Executar o interpretador
cargo run -- examples/hello_world.odin
```

### Executando Testes
```sh
cargo test
```

## Contribuindo
Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou enviar pull requests.

## Licença
Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para mais detalhes.
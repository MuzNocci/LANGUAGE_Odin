# LANGUAGE_Odin
Manifesto da Linguagem Odin



## Introdução
Odin é uma linguagem de programação compilada, projetada para fornecer uma combinação única de eficiência, segurança e legibilidade. Inspirada pela precisão e potência do Rust e pela clareza do Python, Odin foi criada para desenvolvedores que exigem alto desempenho e controle sobre os recursos do sistema, ao mesmo tempo que apreciam uma sintaxe concisa e intuitiva.

## Princípios Fundamentais
1. Desempenho e Eficiência
- Odin foi projetada para ser rápida e eficiente. Sua natureza compilada permite otimizações de baixo nível que garantem uma execução de código próxima ao hardware, minimizando o uso de recursos e maximizando a performance.
- O compilador de Odin será altamente otimizado para gerar código de máquina rápido e leve, aproveitando as melhores práticas da compilação moderna.

2. Segurança em Primeiro Lugar
- Odin coloca a segurança como um dos pilares fundamentais. A linguagem utiliza verificações rigorosas de tipo e mecanismos de controle de memória para evitar falhas comuns de segurança, como estouros de buffer e vazamentos de memória.
- O sistema de tipos será robusto e consistente, reduzindo erros e garantindo que o código seja seguro sem sacrificar a performance.

3. Sintaxe Clara e Intuitiva
- A sintaxe de Odin será limpa, intuitiva e acessível, com foco na facilidade de leitura e manutenção do código.
- Inspirada na simplicidade do Python, Odin permite que os desenvolvedores escrevam código de forma rápida e eficaz, sem abrir mão da clareza ou complexidade desnecessária.

4. Controle Total sobre o Sistema
- Odin oferece aos desenvolvedores controle total sobre o gerenciamento de memória e recursos do sistema, sem abstrações desnecessárias.
- Ferramentas de otimização, como a gestão manual de memória e o controle explícito de threads e concorrência, permitirão que os desenvolvedores criem software com desempenho de nível máximo.

5. Evolução Contínua com a Comunidade
- A linguagem será desenvolvida de forma colaborativa, com a comunidade desempenhando um papel ativo no crescimento e evolução do Odin.
- O foco será na transparência do desenvolvimento e na busca constante pela inovação, assegurando que a linguagem esteja alinhada com as necessidades do mercado e das melhores práticas de programação.

## Missão
Fornecer uma linguagem de programação compilada que combine alto desempenho, controle preciso e segurança, ao mesmo tempo em que mantém uma sintaxe intuitiva e de fácil compreensão, capacitando desenvolvedores a criar software eficiente e confiável.

## Visão
Odin será reconhecida como a escolha ideal para desenvolvedores que buscam uma linguagem de programação poderosa, segura e eficiente, capaz de competir com as melhores linguagens de baixo nível, enquanto mantém uma experiência de desenvolvimento moderna e acessível.


## Dependências (Fedora)
- sudo dnf install llvm llvm-devel clang
- sudo dnf install rust cargo
- sudo dnf install libffi-devel
- sudo dnf install zlib-devel libxml2-devel


## Compile a integração
cargo build --release

## Compile o Compilador Odin
cd odin
odin src/main.odin
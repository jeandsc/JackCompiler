# JackCompiler
Repositório para desenvolvimento de compilador para a linguagem Jack

## Integrantes da Equipe
- Jeanderson da Silva Campos - 20250013677

## Linguagem de Programação
A linguagem de programação escolhida para a construção do compilador é:

- Rust

## Requisitos
- Rust 1.80 ou superior
- Cargo (gerenciador de pacotes do Rust)

## Como Executar

### Gerar arquivos XML
1. Coloque os arquivos `.jack` que deseja processar no path tests/nand2tetris_files/Square
2. Compile e execute o programa principal:

cargo run

O parser irá processar todos os arquivos .jack encontrados no diretório e gerar os arquivos XML de saída em output/Square/ com o sufixo T.xml (ex.: MainT.xml, SquareT.xml, SquareGameT.xml).

### Executar Testes

Para executar os testes do scanner:
cargo test --test test_scanner

Para executar os testes do parser:
cargo test --test test_parser

Para executar todos os testes:
cargo test

## ✅ Status da Validação contra Arquivos Oficiais

O parser foi validado com os três programas de exemplo do Nand2Tetris (Main.jack, Square.jack, SquareGame.jack). A saída XML gerada é estruturalmente idêntica aos arquivos de referência (Main.xml, Square.xml, SquareGame.xml). Todos os testes automatizados são aprovados.

## 🧪 Relato dos Desafios Encontrados

Durante o desenvolvimento do analisador sintático, os principais desafios foram:

- Tratamento de precedência e associatividade: a linguagem Jack não define precedência entre operadores (exceto parênteses). Optou-se por uma avaliação estritamente da esquerda para a direita, implementada por meio de um loop direto em parse_expression.
- Expressões aninhadas e parênteses: a recursão entre parse_term e parse_expression exigiu cuidado para evitar loops infinitos e garantir a geração correta das tags <expression> e <term>.

- Indexação de arrays e chamadas de métodos: a análise de term deve olhar o próximo token ([ ou (/. ) para decidir entre varName, array access ou subroutine call.
- Escapamento de XML: caracteres como <, >, & devem ser convertidos para &lt;, &gt;, &amp; dentro das tags <symbol> e <stringConstant>.

- Recursão e empilhamento: a implementação em Rust exigiu gerenciamento cuidadoso dos empréstimos (borrow checker) e o uso de Result para propagação de erros.

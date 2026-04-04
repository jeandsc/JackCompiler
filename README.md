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

```bash
cargo run
```
### Executar Testes

Para executar os testes do scanner, rode o seguinte comando no terminal:

```bash
cargo test --test test_scanner
```
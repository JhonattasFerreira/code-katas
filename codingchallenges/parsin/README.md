# parsin

A JSON parser built in Rust as part of the [Coding Challenges](https://codingchallenges.fyi/) series by John Crickett.

## Technologies

| Technology | Role |
| ---------- | ---- |
| [Rust](https://www.rust-lang.org/) (stdlib only) | Language — no external crates |
| `std::env` | CLI argument parsing |
| `std::fs` | File input |
| `std::process` | Exit codes |

## Usage

```bash
# Validate a JSON file
parsin file.json
```

Prints `valid` and exits with code `0` for valid JSON, or `invalid: <reason>` and exits with code `1` for invalid JSON.

## Running

Build without installing:

```bash
cargo build --release
# binary at: target/release/parsin
```

## Running the tests

```bash
cargo test
```

This runs both unit tests (inside each module) and integration tests that read from `tests/step*/` fixtures and call `run()` directly.

---

## Architecture

```text
src/
  main.rs     — CLI entry point: reads file, calls run(), prints result and exits
  lexer.rs    — transforms raw text into a token stream
  parser.rs   — validates the token sequence against JSON grammar rules
tests/
  step1/      — valid/invalid fixtures for step 1
  step2/      — valid/invalid fixtures for step 2
  step3/      — valid/invalid fixtures for step 3
  step4/      — valid/invalid fixtures for step 4
```

**Two-phase pipeline:** `run()` calls `lexer::tokenize()` to get a `Vec<Token>`, then passes it to `parser::parse()` to validate the structure.

**Lexer responsibility:** transforms raw characters into typed tokens (`LeftBrace`, `RightBrace`, `String`, `Number`, `Bool`, `Null`, etc.). Has no knowledge of JSON validity — it only rejects unrecognizable characters.

**Parser responsibility:** sole validator of the JSON grammar. Uses a **recursive descent** approach — `parse_object`, `parse_array`, `parse_key_value`, and `parse_value` all consume tokens from a shared `Peekable` iterator. Detects errors like missing colons, trailing commas, and unclosed brackets.

## Development approach

- **TDD**: tests are written before implementation.
- Unit tests live alongside the code in `#[cfg(test)]` modules.
- Integration tests in `main.rs` read fixture files via `std::fs::read_to_string` and call `run()` — no subprocess spawning needed.
- Error messages are kept simple; tests only assert `is_err()`, not the message content.

## Challenge steps

- [x] Step 1: Parse `{}` as valid and empty input as invalid
- [x] Step 2: Support string keys and string values: `{"key": "value"}`
- [x] Step 3: Support boolean, null, and numeric values
- [x] Step 4: Support nested objects and arrays

---

## Versão em Português Brasileiro

Um parser de JSON construído em Rust como parte da série [Coding Challenges](https://codingchallenges.fyi/) de John Crickett.

### Tecnologias

| Tecnologia | Função |
| ---------- | ------ |
| [Rust](https://www.rust-lang.org/) (stdlib apenas) | Linguagem — sem crates externas |
| `std::env` | Parsing de argumentos CLI |
| `std::fs` | Entrada de arquivo |
| `std::process` | Códigos de saída |

### Uso

```bash
# Validar um arquivo JSON
parsin arquivo.json
```

Imprime `valid` e sai com código `0` para JSON válido, ou `invalid: <motivo>` e sai com código `1` para JSON inválido.

### Rodando

Compilar sem instalar:

```bash
cargo build --release
# binário em: target/release/parsin
```

### Rodando os testes

```bash
cargo test
```

Executa tanto os testes unitários (dentro de cada módulo) quanto os testes de integração, que leem fixtures de `tests/step*/` e chamam `run()` diretamente.

---

### Arquitetura

```text
src/
  main.rs     — ponto de entrada CLI: lê o arquivo, chama run(), imprime o resultado e encerra
  lexer.rs    — transforma texto bruto em um fluxo de tokens
  parser.rs   — valida a sequência de tokens contra as regras da gramática JSON
tests/
  step1/      — fixtures válidas/inválidas para o step 1
  step2/      — fixtures válidas/inválidas para o step 2
  step3/      — fixtures válidas/inválidas para o step 3
  step4/      — fixtures válidas/inválidas para o step 4
```

**Pipeline em duas fases:** `run()` chama `lexer::tokenize()` para obter um `Vec<Token>`, depois passa para `parser::parse()` validar a estrutura.

**Responsabilidade do lexer:** transforma caracteres brutos em tokens tipados (`LeftBrace`, `RightBrace`, `String`, `Number`, `Bool`, `Null`, etc.). Não tem conhecimento de validade JSON — apenas rejeita caracteres não reconhecidos.

**Responsabilidade do parser:** único validador da gramática JSON. Usa abordagem de **descida recursiva** — `parse_object`, `parse_array`, `parse_key_value` e `parse_value` consomem tokens de um iterador `Peekable` compartilhado. Detecta erros como dois pontos ausentes, vírgulas sobrando e colchetes não fechados.

### Abordagem de desenvolvimento

- **TDD**: testes são escritos antes da implementação.
- Testes unitários ficam junto ao código em módulos `#[cfg(test)]`.
- Testes de integração em `main.rs` leem fixtures via `std::fs::read_to_string` e chamam `run()` diretamente — sem necessidade de subprocessos.
- Mensagens de erro são mantidas simples; os testes apenas verificam `is_err()`, não o conteúdo da mensagem.

### Steps do desafio

- [x] Step 1: Reconhecer `{}` como válido e entrada vazia como inválida
- [x] Step 2: Suporte a chaves e valores string: `{"key": "value"}`
- [x] Step 3: Suporte a valores booleanos, null e numéricos
- [x] Step 4: Suporte a objetos aninhados e arrays

# ccwc

A clone of the Unix `wc` tool, built in Rust as part of the [Coding Challenges](https://codingchallenges.fyi/) series by John Crickett.

## Technologies

| Technology | Role |
| ---------- | ---- |
| [Rust](https://www.rust-lang.org/) (stdlib only) | Language — no external crates |
| `std::env` | CLI argument parsing |
| `std::io::BufReader` | Buffered reading (8 KB chunks, never loads full file into memory) |
| `std::fs::File` | File input |
| `std::process` | Exit codes and stderr error reporting |

## Usage

```bash
# Count lines, words, and bytes (default)
ccwc file.txt

# Count only lines
ccwc -l file.txt

# Count only words
ccwc -w file.txt

# Count only bytes
ccwc -c file.txt

# Count only characters (UTF-8 aware)
ccwc -m file.txt

# Read from stdin
cat file.txt | ccwc -l
```

## Running

Install the binary to `~/.cargo/bin/`:

```bash
cargo install --path .
```

Build without installing:

```bash
cargo build --release
# binary at: target/release/ccwc
```

## Running the tests

```bash
cargo test
```

This runs both unit tests (inside each module) and integration tests that execute the compiled binary as a real process.

## Uninstalling

```bash
cargo uninstall ccwc
```

---

## Versão em Português Brasileiro

Um clone da ferramenta Unix `wc`, construído em Rust como parte da série [Coding Challenges](https://codingchallenges.fyi/) de John Crickett.

### Tecnologias

| Tecnologia | Função |
| ---------- | ------ |
| [Rust](https://www.rust-lang.org/) (stdlib apenas) | Linguagem — sem crates externas |
| `std::env` | Parsing de argumentos CLI |
| `std::io::BufReader` | Leitura bufferizada (chunks de 8 KB, nunca carrega o arquivo inteiro na memória) |
| `std::fs::File` | Entrada de arquivo |
| `std::process` | Códigos de saída e erros no stderr |

### Uso

```bash
# Contar linhas, palavras e bytes (padrão)
ccwc arquivo.txt

# Contar apenas linhas
ccwc -l arquivo.txt

# Contar apenas palavras
ccwc -w arquivo.txt

# Contar apenas bytes
ccwc -c arquivo.txt

# Contar apenas caracteres (UTF-8)
ccwc -m arquivo.txt

# Ler da entrada padrão (stdin)
cat arquivo.txt | ccwc -l
```

### Rodando

Instale o binário em `~/.cargo/bin/`:

```bash
cargo install --path .
```

Compilar sem instalar:

```bash
cargo build --release
# binário em: target/release/ccwc
```

### Rodando os testes

```bash
cargo test
```

Executa tanto os testes unitários (dentro de cada módulo) quanto os testes de integração, que rodam o binário compilado como um processo real.

### Desinstalando

```bash
cargo uninstall ccwc
```

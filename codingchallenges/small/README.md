# small

A Huffman compression/decompression tool built in Rust as part of the [Coding Challenges](https://codingchallenges.fyi/) series by John Crickett.

## Technologies

| Technology | Role |
| ---------- | ---- |
| [Rust](https://www.rust-lang.org/) (stdlib only) | Language — no external crates |
| `std::env` | CLI argument parsing |
| `std::fs` | File input and output |
| `std::process` | Exit codes and stderr error reporting |

## Usage

```bash
# Compress a file
ccli compress input.txt -o output.huff

# Decompress a file
ccli decompress output.huff -o result.txt
```

## Running

Build without installing:

```bash
cargo build --release
# binary at: target/release/ccli
```

## Running the tests

```bash
cargo test
```

This runs both unit tests (inside each module) and integration tests.

---

## Architecture

```text
src/
  main.rs        — entry point, CLI wiring, compress/decompress dispatch
  cli.rs         — argument parsing (parse_args)
  frequency.rs   — byte frequency counting
  tree.rs        — HuffNode, tree construction
  table.rs       — prefix-code table generation
  bits.rs        — BitWriter and BitReader
  encoder.rs     — compression: header + data
  decoder.rs     — decompression
```

**Pipeline:** `encoder::encode` counts byte frequencies, builds a Huffman tree (`tree.rs`), generates a prefix-code table (`table.rs`), and packs bits via `BitWriter` (`bits.rs`). The compressed format stores the original file size and frequency table in the header so `decoder::decode` can rebuild the tree and recover the original bytes exactly.

**Prefix-free codes:** all characters are placed at the leaves of the tree, never at internal nodes — this guarantees that no code is a prefix of another, making decoding unambiguous.

**Header format:** 8-byte original size (`u64` LE) + 2-byte distinct-byte count (`u16` LE) + N×9-byte `(byte, freq)` pairs, followed by the compressed bit stream.

## Development approach

- **TDD**: tests are written before implementation.
- Unit tests live alongside the code in `#[cfg(test)]` modules.
- No external crates — stdlib only.
- Errors are propagated with `?` or handled with readable messages in `main`; no `unwrap` in production code.

## Challenge steps

- [x] Step 1: Byte frequency counting
- [x] Step 2: Build Huffman tree
- [x] Step 3: Generate prefix-code table
- [x] Step 4: Encode (compress) files
- [x] Step 5: Decode (decompress) files

---

## Versão em Português Brasileiro

Uma ferramenta de compressão/descompressão Huffman construída em Rust como parte da série [Coding Challenges](https://codingchallenges.fyi/) de John Crickett.

### Tecnologias

| Tecnologia | Função |
| ---------- | ------ |
| [Rust](https://www.rust-lang.org/) (stdlib apenas) | Linguagem — sem crates externas |
| `std::env` | Parsing de argumentos CLI |
| `std::fs` | Entrada e saída de arquivos |
| `std::process` | Códigos de saída e erros no stderr |

### Uso

```bash
# Comprimir um arquivo
ccli compress entrada.txt -o saida.huff

# Descomprimir um arquivo
ccli decompress saida.huff -o resultado.txt
```

### Rodando

Compilar sem instalar:

```bash
cargo build --release
# binário em: target/release/ccli
```

### Rodando os testes

```bash
cargo test
```

Executa tanto os testes unitários (dentro de cada módulo) quanto os testes de integração.

---

### Arquitetura

```text
src/
  main.rs        — ponto de entrada, wiring do CLI, despacho compress/decompress
  cli.rs         — parsing de argumentos (parse_args)
  frequency.rs   — contagem de frequência de bytes
  tree.rs        — HuffNode, construção da árvore
  table.rs       — geração da tabela de códigos prefixo
  bits.rs        — BitWriter e BitReader
  encoder.rs     — compressão: cabeçalho + dados
  decoder.rs     — descompressão
```

**Pipeline:** `encoder::encode` conta frequências de bytes, constrói a árvore de Huffman (`tree.rs`), gera a tabela de códigos prefixo (`table.rs`) e empacota bits via `BitWriter` (`bits.rs`). O formato comprimido armazena o tamanho original do arquivo e a tabela de frequências no cabeçalho para que `decoder::decode` possa reconstruir a árvore e recuperar os bytes originais exatamente.

**Códigos livre de prefixo:** todos os caracteres são colocados nas folhas da árvore, nunca nos nós internos — isso garante que nenhum código seja prefixo de outro, tornando a decodificação inequívoca.

**Formato do cabeçalho:** 8 bytes de tamanho original (`u64` LE) + 2 bytes de quantidade de bytes distintos (`u16` LE) + N×9 bytes de pares `(byte, freq)`, seguidos do fluxo de bits comprimido.

### Abordagem de desenvolvimento

- **TDD**: testes são escritos antes da implementação.
- Testes unitários ficam junto ao código em módulos `#[cfg(test)]`.
- Sem crates externas — apenas stdlib.
- Erros são propagados com `?` ou tratados com mensagens legíveis em `main`; sem `unwrap` em código de produção.

### Steps do desafio

- [x] Step 1: Contagem de frequência de bytes
- [x] Step 2: Construção da árvore de Huffman
- [x] Step 3: Geração da tabela de códigos prefixo
- [x] Step 4: Codificação (compressão) de arquivos
- [x] Step 5: Decodificação (descompressão) de arquivos

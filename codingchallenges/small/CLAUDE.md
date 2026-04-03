# Huffman Compression Tool

## What is this project

A command-line tool in Rust that compresses and decompresses text files using the Huffman Encoding/Decoding algorithm. Based on the "Build Your Own Compression Tool" challenge by John Crickett.

The test file is `test.txt` — the complete Les Misérables book. **Do not open or read this file**, it is too large and will consume too many tokens. Use it only to run the binary and validate output values.

---

## Environment

- `cargo` is not in the default shell PATH. Always use the full path: `~/.cargo/bin/cargo`

---

## Development approach

- **TDD**: always write tests before implementation. Only implement after tests exist and are failing.
- **No external dependencies**: the project uses only the Rust stdlib. Do not add crates.
- **Manual CLI**: uses `std::env::args` directly, no `clap` or similar.
- **Error handling with `Result`**: no `unwrap` in production code — errors propagated with `?` or handled with readable messages in `main`.

---

## CLI interface

```sh
ccli compress <input> -o <output>
ccli decompress <input> -o <output>
```

Examples:

```sh
ccli compress test.txt -o test.huff
ccli decompress test.huff -o result.txt
```

With `cargo run`:

```sh
~/.cargo/bin/cargo run -- compress test.txt -o test.huff
~/.cargo/bin/cargo run -- decompress test.huff -o result.txt
```

---

## Module structure

```text
src/
├── main.rs        — entry point, CLI wiring, compress/decompress dispatch
├── cli.rs         — argument parsing (parse_args)
├── frequency.rs   — byte frequency counting
├── tree.rs        — HuffNode, tree construction
├── table.rs       — prefix-code table generation
├── bits.rs        — BitWriter and BitReader
├── encoder.rs     — compression: header + data
└── decoder.rs     — decompression
```

---

## Current state

### Completed

**`frequency.rs`** — `pub fn count(data: &[u8]) -> [u64; 256]`

- Receives a byte slice, returns a fixed 256-element array where the index is the byte value and the value is its count
- 9 tests passing, covering: empty input, single byte, all same, multiple distinct, all 256 values, non-ASCII bytes, null byte, total sum equals input length, and absence of non-present bytes
- Validated against `test.txt`: `'X' = 333` and `'t' = 223000` (values expected by the challenge)

**`cli.rs`** — `pub fn parse_args(args: &[String]) -> Result<Args, String>`

- Receives a string slice (does not read env directly — deliberate decision to enable unit testing)
- Returns `Args { command: Command, input: String, output: String }`
- `Command` is an enum with variants `Compress` and `Decompress`
- 8 tests passing, covering: valid compress, valid decompress, input/output captured correctly, unknown command (error message contains the typo), missing `-o` flag, missing value after `-o`, too few args, missing input

**`main.rs`**

- Collects `std::env::args`, calls `parse_args`, reads the file with `fs::read`, calls `frequency::count`, prints the frequency table (ASCII graphic characters shown as `'x'`, others as `0xHH`)
- Errors are printed to `stderr` with `eprintln!` and exit with code 1

**`tree.rs`** — `pub fn build_tree(freq: &[u64; 256]) -> HuffNode`

- `HuffNode` is a recursive enum with two variants:
  - `Leaf { byte: u8, freq: u64 }` — represents a single character
  - `Internal { freq: u64, left: Box<HuffNode>, right: Box<HuffNode> }` — internal node
- `HuffNode::freq()` returns the frequency of any node
- Uses `BinaryHeap<Reverse<HeapEntry>>` internally as a min-heap (HeapEntry is a private wrapper that implements `Ord` by frequency only)
- Construction algorithm: insert all non-zero bytes as leaves, repeatedly pop two lowest-freq nodes, merge into an Internal node, reinsert, until one node remains
- 6 tests passing, covering: single byte produces leaf root, two bytes produces internal root with freq=sum, OpenDSA example root freq=306, zero-freq bytes ignored, high-freq chars have lower depth than low-freq, all non-zero bytes reachable

**`table.rs`** — `pub fn build_table(root: &HuffNode) -> [Option<Vec<bool>>; 256]`

- Receives the Huffman tree root and returns a 256-slot array; index = byte value; value = `Some(code)` if the byte exists in the tree, `None` otherwise
- `Vec<bool>` represents the bit sequence: `false` = left (0), `true` = right (1)
- Implemented as a recursive DFS via private `traverse` function, accumulating the path down to each leaf
- Edge case: a single symbol produces an empty code `Some(vec![])` — no bits needed
- 6 tests passing, covering: single byte with empty code, two bytes with 1-bit codes, high-frequency byte gets shorter code, all present bytes have `Some`, absent bytes have `None`, prefix-free property

**`bits.rs`** — `BitWriter` and `BitReader`

- `BitWriter::new()`, `write_bit(&mut self, bit: bool)`, `finish(self) -> Vec<u8>`
  - Packs bits into bytes MSB first; partial last byte is zero-padded on the right
- `BitReader::new(data: &[u8])`, `read_bit(&mut self) -> Option<bool>`
  - Reads bits MSB first; returns `None` when exhausted
- No padding metadata returned — the encoder stores the original file size in the header; the decoder stops by byte count, not bit count
- 9 tests passing, covering: empty writer, 8-bit flush, MSB-first order, partial byte padding, 16-bit output, empty reader, MSB-first reading, exhaustion, round-trip

**`encoder.rs`** — `pub fn encode(data: &[u8]) -> Vec<u8>`

- Counts frequencies, builds tree and code table, then serializes:
  - 8 bytes: original file size (`u64` LE)
  - 2 bytes: number of distinct bytes (`u16` LE)
  - N × 9 bytes: `(byte: u8, freq: u64 LE)` pairs
  - Compressed bits packed via `BitWriter`
- Edge case: single distinct byte → 0 bits written (decoder handles via byte count)
- 5 tests passing, covering: original size in header, entry count, entry values, empty data section for single byte, compressed size < original for skewed input
- Validated: `test.txt` (3.3 MB) → `test.huff` (1.9 MB)

**`decoder.rs`** — `pub fn decode(compressed: &[u8]) -> Vec<u8>`

- Reads header to reconstruct frequency table, rebuilds Huffman tree, decodes bits via `BitReader`
- Stops emitting bytes when `original_size` is reached (ignores padding bits)
- Edge case: single distinct byte → emits `original_size` copies without reading any bits
- 5 tests passing, all round-trip: short string, single byte, two bytes, all 256 byte values, skewed data

**`main.rs`**

- Dispatches to `encoder::encode` or `decoder::decode` based on CLI command
- Writes result to output file with `fs::write`

### Total tests: 48 passing, 0 failing

---

## Project complete

---

## Huffman algorithm — conceptual summary

**Why it works:** real text has an unequal distribution of characters. Huffman assigns short codes to frequent bytes and long codes to rare ones. The codes are **prefix-free** — no code is a prefix of another — guaranteed because all characters are placed at the **leaves** of the tree, never at internal nodes.

**Decoding:** traverse the tree bit by bit. `0` = go to left child, `1` = go to right child. Upon reaching a leaf, emit the byte and return to the root.

**Compressed file format (agreed decision):**

- Header: frequency table — number of entries + `(byte, freq)` pairs
- Separator between header and data
- Data: bits packed into bytes (manual BitWriter, no crates)

**Tree serialization:** save the frequency table in the header (not the tree itself). During decompression, the tree is rebuilt from scratch using the frequencies. This is the simplest approach for correct validation.

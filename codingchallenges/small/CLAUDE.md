# Huffman Compression Tool

## What is this project

A command-line tool in Rust that compresses and decompresses text files using the Huffman Encoding/Decoding algorithm. Based on the "Build Your Own Compression Tool" challenge by John Crickett.

The test file is `test.txt` — the complete Les Misérables book. **Do not open or read this file**, it is too large and will consume too many tokens. Use it only to run the binary and validate output values.

---

## Development approach

- **TDD**: always write tests before implementation. Only implement after tests exist and are failing.
- **No external dependencies**: the project uses only the Rust stdlib. Do not add crates.
- **Manual CLI**: uses `std::env::args` directly, no `clap` or similar.
- **Error handling with `Result`**: no `unwrap` in production code — errors propagated with `?` or handled with readable messages in `main`.

---

## CLI interface

```
ccli compress <input> -o <output>
ccli decompress <input> -o <output>
```

Examples:

```
ccli compress test.txt -o test.huff
ccli decompress test.huff -o result.txt
```

---

## Module structure

```
src/
├── main.rs        — entry point, CLI wiring, file reading
├── cli.rs         — argument parsing (parse_args)
├── frequency.rs   — byte frequency counting
├── tree.rs        — HuffNode, tree construction (TODO)
├── table.rs       — prefix-code table generation (TODO)
├── bits.rs        — BitWriter and BitReader (TODO)
├── encoder.rs     — compression: header + data (TODO)
└── decoder.rs     — decompression (TODO)
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

### Total tests: 17 passing, 0 failing

---

## Next step: Step 2 — Huffman Tree (`tree.rs`)

### What needs to be done

Build the Huffman tree from the `[u64; 256]` returned by `frequency::count`.

### Design decisions already agreed upon

**Node representation — recursive enum:**

```rust
enum HuffNode {
    Leaf { byte: u8, freq: u64 },
    Internal { freq: u64, left: Box<HuffNode>, right: Box<HuffNode> },
}
```

**Priority queue — stdlib `BinaryHeap` with `Reverse`:**

- `BinaryHeap` is a max-heap by default
- Wrapping with `std::cmp::Reverse` inverts the ordering to a min-heap
- Each item in the heap is `Reverse<(u64, HuffNode)>` where `u64` is the frequency

**Construction algorithm:**

1. For each byte with frequency > 0, create a `HuffNode::Leaf` and insert into the heap
2. While the heap has more than 1 element:
   - Remove the two nodes with the lowest frequency
   - Create a `HuffNode::Internal` with those two as children and frequency = sum
   - Reinsert into the heap
3. The last remaining element is the tree root

### How to validate

Use the OpenDSA table example as test data:

| Letter | Frequency |
|--------|-----------|
| C      | 32        |
| D      | 42        |
| E      | 120       |
| K      | 7         |
| L      | 42        |
| M      | 24        |
| U      | 37        |
| Z      | 2         |

The tree root must have frequency = sum of all = 306.
More frequent characters (E=120) must have lower depth than less frequent ones (Z=2, K=7).

### TDD approach

Write tests first in `tree.rs`, then implement. Suggested tests:

- Single byte input produces a leaf root
- Two bytes: internal root with two leaf children, frequency = sum
- Full OpenDSA example: root with freq=306
- Root frequency always equals the sum of all input frequencies
- Bytes with freq=0 do not generate nodes

---

## Huffman algorithm — conceptual summary

**Why it works:** real text has an unequal distribution of characters. Huffman assigns short codes to frequent bytes and long codes to rare ones. The codes are **prefix-free** — no code is a prefix of another — guaranteed because all characters are placed at the **leaves** of the tree, never at internal nodes.

**Decoding:** traverse the tree bit by bit. `0` = go to left child, `1` = go to right child. Upon reaching a leaf, emit the byte and return to the root.

**Compressed file format (agreed decision):**

- Header: frequency table — number of entries + `(byte, freq)` pairs
- Separator between header and data
- Data: bits packed into bytes (manual BitWriter, no crates)

**Tree serialization:** save the frequency table in the header (not the tree itself). During decompression, the tree is rebuilt from scratch using the frequencies. This is the simplest approach for correct validation.

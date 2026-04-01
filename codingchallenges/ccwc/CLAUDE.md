# ccwc

Rust implementation of the Unix `wc` tool, built as a coding challenge.

## Commands

```bash
cargo test              # run all tests (unit + integration)
cargo install --path .  # install binary to ~/.cargo/bin/
cargo build --release   # build optimized binary
```

## Architecture

```text
src/
  main.rs       — entry point: parses args, opens input, prints output
  cli.rs        — argument parsing (parse_args) and output formatting (format_counts)
  counter.rs    — pure counting via count_all, returns Counts { lines, words, bytes, chars }
tests/
  integration_test.rs  — black-box tests that run the compiled binary
```

**Separation of concerns:** `counter.rs` knows nothing about CLI — it only receives a `Read` and returns counts. This makes unit testing trivial via `std::io::Cursor`.

**Multiple flags:** `parse_args` accepts any combination of `-l`, `-w`, `-c`, `-m` in any order. `Args.flags` is a `Flags` struct with boolean fields. If no flag is given, defaults to lines + words + bytes. `compute_output` always calls `count_all` once and `format_counts` selects which columns to show in fixed order: lines → words → chars → bytes.

## Development approach

- **TDD**: write tests first, then implement.
- Unit tests live alongside the code in `#[cfg(test)]` modules.
- Integration tests in `tests/` run the real binary via `std::process::Command`.
- No external crates — stdlib only.

## Conventions

- Number output is right-aligned in a field of width 7: `{count:>7}`.
- Errors go to stderr + `process::exit(1)` (Unix philosophy).
- Counting uses buffered reads (8192-byte chunks) — never loads the full file into memory.

## Challenge steps

- [x] Step 1: `-c` byte count
- [x] Step 2: `-l` line count
- [x] Step 3: `-w` word count
- [x] Step 4: `-m` character count (UTF-8 aware)
- [x] Step 5: default mode (no flag) = `-l -w -c`
- [x] Step 6: stdin support when no file is given

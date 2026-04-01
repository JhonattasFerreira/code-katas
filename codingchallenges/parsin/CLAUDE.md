# JSON Parser

A simplified JSON parser built in Rust as a coding challenge.

## Commands

```bash
# Run all tests
~/.cargo/bin/cargo test

# Build
~/.cargo/bin/cargo build

# Run against a file
~/.cargo/bin/cargo run -- <path/to/file.json>
```

Exit code `0` = valid JSON, exit code `1` = invalid JSON.

## Project Structure

- `src/main.rs` — CLI entry point, reads the file and calls `run()`
- `src/lexer.rs` — transforms raw text into tokens
- `src/parser.rs` — validates the token sequence against JSON rules

## Conventions

- We follow TDD: tests are written before implementation
- The lexer has no knowledge of JSON validity — it only transforms text into tokens
- The parser is the sole responsible for deciding if the token sequence is valid
- The parser uses a **recursive descent** approach: `parse_object` and `parse_key_value` are separate functions that consume tokens from a shared peekable iterator
- Integration tests in `main.rs` read directly from `tests/step*/` files using `std::fs::read_to_string` and call `run()` — no subprocess spawning needed
- Error messages are kept simple; we only assert `is_err()` in tests, not the message content

## Steps

| Step | Description | Status |
|------|-------------|--------|
| 1 | Parse `{}` as valid and empty input as invalid | Done |
| 2 | Support string keys and string values: `{"key": "value"}` | Done |
| 3 | Support boolean, null and numeric values | Pending |
| 4 | Support nested objects and arrays | Pending |
| 5 | Add custom tests and validate against the full JSON test suite | Pending |

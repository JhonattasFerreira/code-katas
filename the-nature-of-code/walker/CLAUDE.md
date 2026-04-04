# Walker — The Nature of Code

Creative coding exercise from [The Nature of Code](https://natureofcode.com/) by Daniel Shiffman, implemented in Rust using macroquad.

## What it does

A circle (the walker) moves smoothly across the screen driven by 1D Perlin-like noise. The position is not random — it follows a continuous noise path, so the movement is organic and never teleports.

## Running

Always run in **release mode** — debug mode is too slow for real-time rendering:

```bash
~/.cargo/bin/cargo run --release
```

> This project runs inside a VSCode Flatpak. Use the full cargo path `~/.cargo/bin/cargo` for all commands. See `/rust-flatpak` for details.

## Project structure

- `main.rs` — window config and main loop
- `walker.rs` — Walker struct with noise-based movement

## Key decisions

**Noise library:** uses `noise = "0.9"` with `Fbm<Value>`, not bare `Perlin`. Standard Perlin always returns 0 at integer lattice points, causing the walker to snap back to center periodically. `Fbm<Value>` matches p5.js `noise()` behavior.

**Two noise offsets, one instance:** `tx` starts at `0.0` and `ty` starts at `10000.0`, both sampled from the same noise instance. The large offset guarantees x and y are decorrelated for millions of frames without needing two separate instances.

**Delta time:** the noise offset increments are multiplied by `get_frame_time() * 60.0` so movement speed is consistent regardless of framerate.

**Anti-aliasing:** MSAA 4x is enabled via `Conf { sample_count: 4, .. }`. The circle is drawn with `draw_poly` at 64 segments instead of `draw_circle` (which uses only 20 segments).

**Noise seed:** the walker uses seed `1`. If you add other noise-driven elements in the future, use different seeds to avoid correlated behavior.

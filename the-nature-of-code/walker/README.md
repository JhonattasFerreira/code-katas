# walker

A Perlin noise walker simulation built in Rust, based on an exercise from [The Nature of Code](https://natureofcode.com/) by Daniel Shiffman.

## Technologies

| Technology | Role |
| ---------- | ---- |
| [Rust](https://www.rust-lang.org/) | Language |
| [macroquad](https://macroquad.rs/) `0.4` | Windowing, rendering loop, and drawing primitives |
| [noise](https://crates.io/crates/noise) `0.9` | `Fbm<Value>` noise for smooth, continuous movement |

## Running

Always run in **release mode** — debug mode is too slow for real-time rendering:

```bash
~/.cargo/bin/cargo run --release
```

> This project runs inside a VSCode Flatpak. Use the full cargo path `~/.cargo/bin/cargo` for all commands.

---

## Architecture

```text
src/
  main.rs    — window config (MSAA 4x), main loop: clear → step → show → next_frame
  walker.rs  — Walker struct: noise offsets (tx, ty), step() and show() methods
```

**Two noise offsets, one instance:** `tx` starts at `0.0` and `ty` starts at `10000.0`, both sampled from the same `Fbm<Value>` instance. The large offset ensures x and y are decorrelated without needing two separate noise instances.

**`Fbm<Value>` over bare `Perlin`:** standard `Perlin` always returns `0.0` at integer lattice points, which causes the walker to snap back to center periodically. `Fbm<Value>` avoids this and matches the behavior of p5.js `noise()`.

**Delta time:** noise offsets are incremented by `get_frame_time() * 60.0 * 0.007` each frame, keeping movement speed consistent regardless of framerate.

**Anti-aliasing:** MSAA 4x is enabled via `Conf { sample_count: 4, .. }`. The circle is drawn with `draw_poly` at 64 segments instead of `draw_circle` (which uses only 20 segments) for a smoother edge.

## Development approach

- No external state — the `Walker` struct owns its noise instance and both offsets.
- Noise seed fixed at `1`; future noise-driven elements should use different seeds to avoid correlated behavior.
- Release mode is the only supported build target for this project.

## Challenge steps

- [x] Step 1: Basic random walker (uniform random direction each frame)
- [x] Step 2: Perlin noise walker (smooth, continuous movement via `Fbm<Value>`)

---

## Versão em Português Brasileiro

Uma simulação de walker com ruído de Perlin construída em Rust, baseada em um exercício de [The Nature of Code](https://natureofcode.com/) de Daniel Shiffman.

### Tecnologias

| Tecnologia | Função |
| ---------- | ------ |
| [Rust](https://www.rust-lang.org/) | Linguagem |
| [macroquad](https://macroquad.rs/) `0.4` | Janela, loop de renderização e primitivas de desenho |
| [noise](https://crates.io/crates/noise) `0.9` | Ruído `Fbm<Value>` para movimento suave e contínuo |

### Rodando

Sempre rode em modo **release** — o modo debug é lento demais para renderização em tempo real:

```bash
~/.cargo/bin/cargo run --release
```

> Este projeto roda dentro de um Flatpak do VSCode. Use o caminho completo do cargo `~/.cargo/bin/cargo` em todos os comandos.

---

### Arquitetura

```text
src/
  main.rs    — config da janela (MSAA 4x), loop principal: limpa → step → show → next_frame
  walker.rs  — struct Walker: offsets de ruído (tx, ty), métodos step() e show()
```

**Dois offsets de ruído, uma instância:** `tx` começa em `0.0` e `ty` em `10000.0`, ambos amostrados da mesma instância `Fbm<Value>`. O grande offset garante que x e y sejam descorrelacionados sem precisar de duas instâncias separadas.

**`Fbm<Value>` ao invés de `Perlin` puro:** o `Perlin` padrão retorna `0.0` nos pontos inteiros da malha, fazendo o walker voltar ao centro periodicamente. `Fbm<Value>` evita isso e replica o comportamento do `noise()` do p5.js.

**Delta time:** os offsets de ruído são incrementados por `get_frame_time() * 60.0 * 0.007` a cada frame, mantendo a velocidade de movimento consistente independente do framerate.

**Anti-aliasing:** MSAA 4x ativado via `Conf { sample_count: 4, .. }`. O círculo é desenhado com `draw_poly` em 64 segmentos ao invés de `draw_circle` (que usa apenas 20 segmentos) para bordas mais suaves.

### Abordagem de desenvolvimento

- Sem estado externo — a struct `Walker` possui a instância de ruído e ambos os offsets.
- Seed de ruído fixo em `1`; futuros elementos guiados por ruído devem usar seeds diferentes para evitar comportamento correlacionado.
- O modo release é o único alvo de build suportado para este projeto.

### Etapas do desafio

- [x] Etapa 1: Walker aleatório básico (direção aleatória uniforme a cada frame)
- [x] Etapa 2: Walker com ruído de Perlin (movimento suave e contínuo via `Fbm<Value>`)

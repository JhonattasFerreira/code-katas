# pongzin

A Pong clone built in Rust as a game dev learning vehicle — game loop, input, collision, basic physics, game state, and simple AI.

## Technologies

| Technology | Role |
| ---------- | ---- |
| [Rust](https://www.rust-lang.org/) | Language |
| [macroquad](https://docs.rs/macroquad) `0.4.15` | Windowing, rendering loop, and drawing primitives |

No sprites: every element is drawn as a geometric shape (white rectangles/squares) using macroquad's drawing primitives. No audio.

## Running

```bash
cargo run --release
```

## Running the tests

```bash
cargo test
```

Covers the pure logic: AABB collision detection, bounce-angle calculation, scoring/win rules, CPU AI deadzone, and menu button point-in-rectangle/marker-position geometry. This logic uses macroquad types (`Vec2`) but has no I/O dependency, so it's testable in isolation.

---

## Architecture

```text
src/
  main.rs          — window config, phase state machine (Menu / Playing), main loop
  window.rs         — window setup (resizable)
  input.rs           — keyboard reading → movement intent
  paddle.rs         — Paddle struct: position, movement, screen-edge clamping
  ball.rs             — Ball struct: constant-velocity movement, wall bounce
  collision.rs      — AABB collision detection, bounce-angle calculation from impact point
  ai.rs                  — CPU paddle chase logic, isolated from macroquad
  game_state.rs   — score, win condition, round reset
  menu.rs            — start button click/key detection, point-in-rectangle geometry, animated corner-marker position
  render.rs           — drawing: paddles, ball, score, center dashed line
  constants.rs      — centralized tuning values (speeds, sizes, durations)
  test_helpers.rs  — shared test fixtures
```

**Phase state machine:** `main.rs` orchestrates two phases, `AppPhase::Menu` and `AppPhase::Playing`. Each phase runs its own update → collision → render sequence per frame. There's no separate pause or game-over screen — match end is just the winner message, followed by an automatic return to the menu.

**Bounce angle from impact point:** ball-paddle collision doesn't do simple reflection. The rebound angle depends on where the ball hit the paddle — center produces a straight bounce, edges produce steeper angles — computed via interpolation in `collision.rs`.

**Ball speed ramp:** the ball's speed (`BALL_SPEED`) increases by `BALL_SPEED_INCREASE_FACTOR` on every paddle bounce, capped at `BALL_MAX_SPEED` (2x base speed).

**CPU AI deadzone:** the CPU paddle chases the ball's Y position, but ignores small offsets within `AI_DEADZONE` around its center to avoid jitter when the ball is nearly aligned. No other imperfection is modeled.

**Resizable window:** all elements scale and reposition proportionally on resize.

## Development approach

- Modules separated by responsibility from the start, even for a project this size — that's part of the learning goal
- TDD for pure logic: collision, bounce angle, scoring rules, AI deadzone, menu geometry — tests before implementation, red → green → refactor
- Code that depends directly on macroquad I/O (real keyboard/mouse reads, drawing, window config) has no automated tests — validated manually by playing
- Small, single-responsibility functions; descriptive names; comments only when the *why* isn't obvious from the code

## Challenge steps

- [x] Step 1: Setup and game loop
- [x] Step 2: Input handling
- [x] Step 3: Player paddle movement
- [x] Step 4: Ball and basic movement
- [x] Step 5: Ball-paddle collision (AABB)
- [x] Step 6: Bounce angle based on impact point
- [x] Step 7: Score and game state
- [x] Step 8: CPU paddle AI
- [x] Step 9: Visual polish

---

## Versão em Português Brasileiro

Um clone de Pong construído em Rust como veículo de aprendizado de game dev — game loop, input, colisão, física básica, estado de jogo e IA simples.

### Tecnologias

| Tecnologia | Função |
| ---------- | ------ |
| [Rust](https://www.rust-lang.org/) | Linguagem |
| [macroquad](https://docs.rs/macroquad) `0.4.15` | Janela, loop de renderização e primitivas de desenho |

Sem sprites: todos os elementos são desenhados como formas geométricas (retângulos/quadrados brancos) usando as primitivas de desenho do macroquad. Sem áudio.

### Rodando

```bash
cargo run --release
```

### Rodando os testes

```bash
cargo test
```

Cobre a lógica pura: detecção de colisão AABB, cálculo de ângulo de rebote, regras de pontuação/vitória, deadzone da IA da CPU, e geometria de ponto-em-retângulo/posição do marcador do botão do menu. Essa lógica usa tipos do macroquad (`Vec2`), mas não depende de I/O, então é testável isoladamente.

---

### Arquitetura

```text
src/
  main.rs          — config da janela, máquina de estados de fases (Menu / Playing), loop principal
  window.rs         — configuração da janela (redimensionável)
  input.rs           — leitura de teclado → intenção de movimento
  paddle.rs         — struct Paddle: posição, movimento, limite às bordas da tela
  ball.rs             — struct Ball: movimento com velocidade constante, bounce nas paredes
  collision.rs      — detecção de colisão AABB, cálculo de ângulo de rebote a partir do ponto de impacto
  ai.rs                  — lógica de perseguição da raquete da CPU, isolada do macroquad
  game_state.rs   — placar, condição de vitória, reset de rodada
  menu.rs            — detecção de clique/tecla de start, geometria de ponto-em-retângulo, posição do marcador animado no canto
  render.rs           — desenho: raquetes, bola, placar, linha central pontilhada
  constants.rs      — valores de tunagem centralizados (velocidades, tamanhos, durações)
  test_helpers.rs  — fixtures de teste compartilhadas
```

**Máquina de estados de fases:** `main.rs` orquestra duas fases, `AppPhase::Menu` e `AppPhase::Playing`. Cada fase roda sua própria sequência update → colisão → render a cada frame. Não há tela de pausa ou game over separada — o fim de partida é só a mensagem de vencedor, seguida do retorno automático ao menu.

**Ângulo de rebote a partir do ponto de impacto:** a colisão bola-raquete não faz reflexão simples. O ângulo de saída depende de onde a bola bateu na raquete — centro produz rebote reto, extremidades produzem ângulos mais inclinados — calculado por interpolação em `collision.rs`.

**Aumento de velocidade da bola:** a velocidade da bola (`BALL_SPEED`) aumenta por `BALL_SPEED_INCREASE_FACTOR` a cada rebote na raquete, até um teto de `BALL_MAX_SPEED` (2x a velocidade base).

**Deadzone da IA da CPU:** a raquete da CPU persegue a posição Y da bola, mas ignora pequenos desvios dentro de `AI_DEADZONE` ao redor do seu centro para evitar tremulação quando a bola está quase alinhada. Nenhuma outra imperfeição é modelada.

**Janela redimensionável:** todos os elementos escalam e reposicionam proporcionalmente ao redimensionar.

### Abordagem de desenvolvimento

- Módulos separados por responsabilidade desde o início, mesmo em um projeto desse tamanho — isso faz parte do objetivo de aprendizado
- TDD para lógica pura: colisão, ângulo de rebote, regras de pontuação, deadzone da IA, geometria do menu — testes antes da implementação, ciclo red → green → refactor
- Código que depende diretamente de I/O do macroquad (leitura real de teclado/mouse, desenho, configuração de janela) não tem testes automatizados — validado manualmente jogando
- Funções pequenas e de responsabilidade única; nomes descritivos; comentários só quando o *porquê* não é óbvio pelo código

### Etapas do desafio

- [x] Etapa 1: Setup e game loop
- [x] Etapa 2: Input handling
- [x] Etapa 3: Movimento da raquete do jogador
- [x] Etapa 4: Bola e movimento básico
- [x] Etapa 5: Colisão bola-raquete (AABB)
- [x] Etapa 6: Ângulo de rebote baseado no ponto de impacto
- [x] Etapa 7: Placar e estado de jogo
- [x] Etapa 8: IA da raquete da CPU
- [x] Etapa 9: Polimento visual

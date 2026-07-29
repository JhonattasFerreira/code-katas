# Pongzin

Projeto de aprendizado de game dev: um Pong em Rust, usando o desenvolvimento do
jogo como veículo para aprender conceitos (game loop, input, colisão, física
básica, estado de jogo, IA simples).

## Stack

- Linguagem: Rust
- Lib gráfica: [macroquad](https://docs.rs/macroquad)
- Sem sprites: todos os elementos são desenhados como formas geométricas
  (retângulos/quadrados brancos) usando as primitivas de desenho do macroquad
- Sem áudio

## Decisões de design já fechadas

- **Modo de jogo**: 1 jogador vs CPU
- **Controles do jogador**: setas do teclado (cima/baixo)
- **IA da CPU**: persegue a posição Y da bola, com uma deadzone
  (`AI_DEADZONE`) ao redor do centro da raquete para evitar tremulação quando
  a bola está quase alinhada (sem imperfeição proposital além disso)
- **Placar**: visível na tela; ao atingir N pontos (`POINTS_TO_WIN`), declara
  vencedor, exibe mensagem por `WINNER_MESSAGE_DURATION` segundos e reinicia
  a partida (volta para o menu inicial)
- **Física da bola**: velocidade inicial (`BALL_SPEED`) aumenta a cada rebote
  na raquete por `BALL_SPEED_INCREASE_FACTOR`, até um teto de
  `BALL_MAX_SPEED` (2x a velocidade base); ângulo de rebote na raquete
  depende do ponto de impacto (não é reflexão simples)
- **Janela**: redimensionável, elementos devem escalar/reposicionar
  proporcionalmente
- **Estética**: fundo preto, formas brancas, linha central pontilhada (estilo
  clássico do Pong original)
- **Tela de início**: existe uma tela de menu (`AppPhase::Menu`) antes da
  partida, com botão de start (clique do mouse ou Enter/Espaço) e uma
  animação de marcador percorrendo a borda do botão em sentido horário; não
  há tela de pausa nem "game over" separada — o fim de partida é só a
  mensagem de vencedor, seguida do retorno automático ao menu

## Arquitetura

Separar responsabilidades em módulos desde o início, mesmo sendo um projeto
pequeno — isso faz parte do aprendizado. Ideia geral (sujeita a ajuste durante
a implementação):

- Módulo de input (leitura de teclado → intenção de movimento)
- Módulo de entidades (`Paddle`, `Ball` como structs com estado próprio,
  usando `macroquad::math::Vec2` para posição, direção e tamanho)
- Módulo de física/colisão (cálculo de movimento, detecção de colisão AABB,
  cálculo de ângulo de rebote)
- Módulo de estado de jogo (placar, condição de vitória, reset de rodada)
- Módulo de IA (`ai.rs`): lógica de perseguição da CPU, isolada do macroquad
- Módulo de menu (`menu.rs`): detecção de clique/tecla de start, geometria de
  ponto-em-retângulo, posição do marcador animado na borda do botão
- Módulo de constantes (`constants.rs`): valores de tunagem centralizados
  (velocidades, tamanhos, durações, etc.)
- Loop principal (`main.rs`) orquestra uma máquina de estados de fases
  (`AppPhase::Menu` / `AppPhase::Playing`) e, dentro de cada fase: update →
  colisão → render

## Testes

Este projeto segue TDD para lógica pura: colisão, cálculo de ângulo de
rebote, regras de pontuação, deadzone da IA, geometria de ponto-em-retângulo
e posição do marcador na borda do botão do menu. Testes antes da
implementação, ciclo red → green → refactor. (Essa lógica usa tipos do
macroquad como `Vec2`, mas não depende de I/O — janela, input real ou
desenho — então continua testável isoladamente.)

Código que depende diretamente de I/O do macroquad (leitura real de
teclado/mouse, desenho na tela, configuração de janela) não precisa de
testes automatizados — validação é manual, jogando.

## Estilo de código

- Funções pequenas, uma responsabilidade cada
- Nomes descritivos, evitar "mágica"
- Comentários só quando o *porquê* não é óbvio pelo código

## Fora de escopo por enquanto

Ideias que podem virar fases futuras, mas não estão no roadmap atual:
modo 2 jogadores, dificuldade ajustável da IA (além da deadzone atual), sons,
sprites/arte, tela de pausa.

Ver `roadmap.md` para o plano de implementação por fases.

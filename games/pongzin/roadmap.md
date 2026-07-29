# Roadmap — Pongzin

Cada fase corresponde a um conceito de game dev, usando o Pong como veículo de
aprendizado. As fases são incrementais: cada uma deve terminar com algo
executável/vísivel na tela. Detalhamento técnico de cada fase fica para ser
discutido quando chegarmos nela.

Cada fase é tratada como uma sessão nova. Ao começar uma sessão, olhe a seção
"Status" abaixo pra saber onde paramos, e ao terminar uma fase, atualize o
status dela aqui antes de encerrar.

## Status

| Fase | Status |
| --- | --- |
| 1 — Setup e Game Loop | Concluído |
| 2 — Input Handling | Concluído |
| 3 — Movimento da Raquete do Jogador | Concluído |
| 4 — Bola e Movimento Básico | Concluído |
| 5 — Colisão Bola-Raquete (AABB) | Concluído |
| 6 — Ângulo de Rebote Baseado no Ponto de Impacto | Concluído |
| 7 — Placar e Estado de Jogo | Concluído |
| 8 — IA da Raquete da CPU | Concluído |
| 9 — Polimento Visual | Concluído |

Legenda: Pendente / Em andamento / Concluído

## Fase 1 — Setup e Game Loop

Conceito: o loop de jogo (update → render, rodando a cada frame).

- Configurar macroquad com janela redimensionável
- Loop principal básico
- Desenhar um retângulo branco estático na tela pra validar que o pipeline
  de desenho funciona

## Fase 2 — Input Handling

Conceito: capturar input do teclado e traduzir em intenção de movimento.

- Módulo de input isolado
- Ler teclas de seta (cima/baixo)
- Ainda sem mover nada — só validar que o input é capturado corretamente

## Fase 3 — Movimento da Raquete do Jogador

Conceito: entidades com estado (posição, velocidade) e movimento por frame.

- Struct `Paddle`
- Mover a raquete do jogador verticalmente com o input da Fase 2
- Limitar a raquete às bordas da tela
- Lidar com redimensionamento da janela (reposicionamento proporcional)

## Fase 4 — Bola e Movimento Básico

Conceito: outra entidade com velocidade constante e colisão com os limites do
mundo (paredes).

- Struct `Ball`
- Movimento com velocidade constante
- Bounce nas paredes (topo/fundo da tela)

## Fase 5 — Colisão Bola-Raquete (AABB)

Conceito: detecção de colisão entre retângulos (AABB) e reflexão de
velocidade.

- Detecção de colisão entre `Ball` e `Paddle`
- Reflexão simples da direção X ao colidir (passo intermediário de
  aprendizado, antes de refinar na Fase 6)
- Testes unitários (TDD) para a lógica de detecção de colisão, isolada do
  macroquad

## Fase 6 — Ângulo de Rebote Baseado no Ponto de Impacto

Conceito: mapear uma posição de impacto para um ângulo/vetor de saída
(interpolação).

- Refinar a colisão da Fase 5: o ângulo de saída da bola depende de onde ela
  bateu na raquete (centro = reto, extremidades = mais inclinado)
- Testes unitários (TDD) para o cálculo do ângulo, isolado do macroquad

## Fase 7 — Placar e Estado de Jogo

Conceito: gerenciamento de estado (fora do estado de posição/velocidade das
entidades).

- Detectar quando a bola passa de uma raquete (ponto pro adversário)
- Módulo de estado de jogo: placar, reset da bola após ponto
- Exibir placar na tela
- Condição de vitória ao atingir N pontos, com reinício da partida
- Testes unitários (TDD) para a regra de pontuação/vitória

## Fase 8 — IA da Raquete da CPU

Conceito: comportamento autônomo simples (perseguição).

- Raquete da CPU persegue a posição Y da bola, respeitando uma velocidade
  máxima (mesmo limite de movimento da Fase 3)

## Fase 9 — Polimento Visual

Conceito: ajustes finais de apresentação, sem alterar a lógica do jogo.

- Linha central pontilhada
- Revisão de cores/proporções na tela redimensionável
- Revisão dos módulos criados até aqui (pequenos refactors se fizer sentido)

## Ideias futuras (fora do roadmap atual)

- Modo 2 jogadores local
- Dificuldade ajustável da IA (atraso de reação, margem de erro)
- Sons
- Sprites/arte
- Menus, pausa, tela de game over dedicada

# Games — Maintainer Guide

## Adding a new game

When a new project is added under `games/`, two things must be done:

1. **Create the project README** at `games/<project>/README.md`
2. **Update the platform index** at `games/README.md`

---

## 1. Reading the project before writing

Before writing anything, explore the project:

- Read `games/<project>/CLAUDE.md` — it has the full context: what the game does, design decisions, module structure, and current state.
- Read `games/<project>/roadmap.md` if it exists — it lists implementation phases and their status.
- Skim `src/` to confirm the module list is accurate.

Do **not** guess. Everything needed to write the README is already in CLAUDE.md and roadmap.md.

---

## 2. Project README structure

Follow the exact structure used by `pongzin/README.md`. Every README must have **both an English section and a Brazilian Portuguese section** (PT-BR comes after a `---` divider).

### Required sections (in order)

```
# <project-name>

One-line description of the game and its purpose as a learning project.

## Technologies
Table: Technology | Role

## Running
How to run the game (`cargo run --release`, etc.).

## Running the tests
`cargo test` + brief note on what it covers.

---

## Architecture
src/ tree with one-line descriptions per file.
Prose explaining the key design decisions (state machine, physics, notable patterns).

## Development approach
Bullet list of conventions (TDD scope, code style, etc.).

## Challenge steps
Checkbox list of completed phases (pull from roadmap.md if present).

---

## Versão em Português Brasileiro
(mirror of all sections above, translated)
```

### Style rules

- Keep descriptions factual — pulled from the project's CLAUDE.md/roadmap.md, not invented.
- Architecture prose should explain *why*, not just *what*: mention the key design decisions visible in the code.
- Challenge steps are checkboxes; all completed ones are checked (`- [x]`).
- No emojis, no filler phrases.

---

## 3. Updating games/README.md

Add a row to **both** tables (English and PT-BR):

```markdown
| [<project>](<project>/) | <short description> | <language> | Done |
```

```markdown
| [<project>](<project>/) | <descrição curta> | <linguagem> | Concluído |
```

The row goes at the bottom of the existing list, in the order the games were completed.

# The Nature of Code — Maintainer Guide

## Adding a new project

When a new project is added under `the-nature-of-code/`, two things must be done:

1. **Create the project README** at `the-nature-of-code/<project>/README.md`
2. **Update the platform index** at `the-nature-of-code/README.md`

---

## 1. Reading the project before writing

Before writing anything, explore the project:

- Read `the-nature-of-code/<project>/CLAUDE.md` — it has the full context: what the simulation does, the key design decisions, and the module structure.
- Skim `src/` to confirm the module list is accurate.

Do **not** guess. Everything needed to write the README is already in CLAUDE.md.

---

## 2. Project README structure

Follow the exact structure used by `walker/README.md`. Every README must have **both an English section and a Brazilian Portuguese section** (PT-BR comes after a `---` divider).

Projects here are visual simulations — there is no CLI, no binary to install, and typically no automated tests. The README structure reflects that.

### Required sections (in order)

```
# <project-name>

One-line description + link to natureofcode.com + credit to Daniel Shiffman.

## Technologies
Table: Technology | Role

## Running
`~/.cargo/bin/cargo run --release` + note about release mode being required.

---

## Architecture
src/ tree with one-line descriptions per file.
Prose explaining the key design decisions (noise approach, rendering choices, notable patterns).

## Development approach
Bullet list of conventions used in this project.

## Challenge steps
Checkbox list of completed steps from the book/exercises.

---

## Versão em Português Brasileiro
(mirror of all sections above, translated)
```

### Style rules

- Keep descriptions factual — pulled from CLAUDE.md, not invented.
- Architecture prose should explain *why*, not just *what*: mention the key design decisions visible in the code.
- Challenge steps are checkboxes; all completed ones are checked (`- [x]`).
- No emojis, no filler phrases.
- Always note that release mode is required (debug is too slow for real-time rendering).
- Always note the Flatpak cargo path (`~/.cargo/bin/cargo`) since this runs inside VSCode Flatpak.

---

## 3. Updating the-nature-of-code/README.md

Add a row to **both** tables (English and PT-BR):

```markdown
| [<project>](<project>/) | <short description> | Rust | Done |
```

```markdown
| [<project>](<project>/) | <descrição curta> | Rust | Concluído |
```

The row goes at the bottom of the existing list, in the order the projects were completed.

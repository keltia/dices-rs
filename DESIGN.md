# dices-rs

## Design document (WIP)

### Core dice module

The `dice` module defines the rolling engine and parsing for dice expressions.

- `Dice` types:
  - `Regular(size)`: yields `1..=size`
  - `Open(size)`: rerolls when the roll is `size`
  - `Constant(size)`: fixed value
  - `Fate`: yields -1/0/1
  - `Bonus(isize)`: tracked bonus
- `DiceSet`: collection of dice, with `parse()` for inputs like `3D6 +1`
- `Rollable`: trait implemented by `Dice` and `DiceSet`

Parsing of dice expressions (including bonuses and open dice) is handled in `dice::parse`.

### Engine (CLI runtime)

`Engine` owns the command registry and drives the REPL (rustyline). It:

- loads builtin commands from `src/bin/dices/commands.yaml`
- merges aliases/macros from `$HOME/.config/dices/aliases` (or a CLI-provided path)
- exposes `list`, `aliases`, `macros`
- runs the REPL loop and executes compiled actions

### Commands

Commands come from `commands.yaml` and include:

- Builtins: `dice`, `open`
- Special commands: `list`, `exit`, `aliases`, `macros`

### Aliases and macros

The aliases file defines either:

- an alias to an existing command
- a macro (new command with arguments)

Builtin aliases/macros:

- `roll` -> `dice`
- `doom` -> `dice 2D6`

### Compiler

The `Compiler` resolves user input into an `Action`:

- parse the keyword
- resolve aliases/macros recursively (cycle detection + max depth)
- return one of: `Execute`, `List`, `Exit`, `Aliases`, `Macros`, or `Error`

### Execution flow

- build `Engine` with builtins
- load aliases/macros
- create a `Compiler` from the command map
- for each input line:
  - compile to `Action`
  - execute or handle list/exit/aliases/macros

### Architecture diagram

```text
User input
  |
  v
Engine (REPL + command registry)
  |
  v
Compiler (parse + resolve aliases/macros)
  |
  v
Action
  |
  +--> Execute -> Cmd (dice/open) -> dice::parse -> DiceSet -> roll -> Res
  |
  +--> List / Aliases / Macros / Exit
```

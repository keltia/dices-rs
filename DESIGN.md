# dices-rs

## Design document (WIP, Will Change Frequently, Danger Will Robinson, There Be Dragons)

### Core commands from the `dice` module.

Here we have the real commands that everything is reduced/compiled into.

- Dice
- Open

### Engine (other name for the CLI parser)

This contains the entries of merged builtin commands and aliases, this is where user input from `rustyline` is parsed
and executed.

    let e = Engine::new();

    e.list() -> list of all available commands

### Commands

There are different builtin commands:

- dice
- open
- list
- exit
- new

### Aliases

We load and parse the `aliases` file (from known location or command-line). This will give you a list of either
aliases (for known and builtin commands) or new commands (creating a new keyword with arguments).

For convenience, there are some aliases pre-defined:

- roll for dice
- doom for a special roll of 2D6
- help for list

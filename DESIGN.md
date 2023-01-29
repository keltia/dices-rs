# dices-rs

## Design document (WIP, Will Change Frequently, Danger Will Robinson, There Be Dragons)

### Core commands from the `dice` module.

Here we have the real commands that everything is reduced/compiled into.

- Dice
- Open

There is the parser for the arguments (e.g. "2D6") and the bonus handling.

### Engine (other name for the CLI parser)

This contains the entries of merged builtin commands and aliases, this is where user input from `rustyline` is parsed
and executed. This is where everything is handled.

    let e = Engine::new();

    e.list() -> list of all available commands

### Commands

There are different builtin commands:

- dice
- open
- list
- exit
- new

See the `commands.yaml`  file for all of them.

### Aliases

We load and parse the `aliases` file (from known location or command-line). This will give you a list of either
aliases (for known and builtin commands) or new commands (creating a new keyword with arguments).

For convenience, there are some aliases pre-defined:

- roll for dice
- doom for a special roll of 2D6
- help for list

## Compiler

The compiler does for each line:

- recursively parse to identify the potential command:
  - list/exit/etc. break immediately
  - alias can point to a macro or another alias
  - macros can be referring to aliases, builtin command of another macro
  - if a builtin command is identified, return an Execute action.

## Execution

- load alias file
- create an engine with builtin commands, load aliases
- engine create an instance of the compiler with all the commands
- for each input
  - compile the line and return an action
  - do whatever is needed for each possible action
  - exit at the end


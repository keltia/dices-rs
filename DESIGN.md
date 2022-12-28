# dices-rs

## Design document (WIP, Will Change Frequently, Danger Will Robinson, There Be Dragons)

### Core

Here we have the real commands that everything is reduced/compiled into.

- Dice
- Open
- New

### Engine

This contains the entries of merged builtin commands and aliases, this is where user input from `rustyline` is parsed
and executed.

    let e = Engine::new();

    e.list() -> list of all available commands
        

### Cmds

There are different commands: 


### Aliases

We load and parse the `aliases` file (from known location or command-line). This will give you a list of either
aliases (for known and builtin commands) or new commands (creating a new keyword with arguments).

For convenience, we also have some builtin aliases like `roll` for `dice`, etc.
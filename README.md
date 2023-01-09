# dices-rs

> **Dice simulator**

## Build Status

main:  [![main](https://github.com/keltia/dices-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/keltia/dices-rs/actions/workflows/rust.yml)
develop:  [![develop](https://github.com/keltia/dices-rs/actions/workflows/develop.yml/badge.svg)](https://github.com/keltia/dices-rs/actions/workflows/develop.yml)

## Description

Usual small Dice simulator for AD&D type of games.

Previous unreleased version was a straight port of the [Ruby] code in https://bitbucket.org/keltia/addfh-utils
(Using [Mercurial]), rewritten before in [Go] [here](https://github.com/keltia/dices-go) and now rewritten to
use [Rust].

It serves as well as a test-bed on my way of learning [Rust]. It has been rewritten several times (see the commits) as
I progress further in my learning of the language.

## Installation

    cargo install dices-rs

or get the source and build:

    git clone https://github.com/keltia/dices-rs
    cd dices-rs
    cargo test
    cargo bench
    cargo install --path .

The binary will be installed wherever it is defined on your machine and the library in `dice` itself will be compiled
and available. The library itself is very minimal and usable only through the CLI utility.

## Basic commands

Usage:

```text
Small CLI utility to roll dices.

Usage: dices [OPTIONS]

Options:
  -A, --alias-file <ALIAS_FILE>  Alias file
  -v, --verbose...               Verbose mode
  -V, --version                  Display utility full version
  -h, --help                     Print help information
```

Example:

```text
$ cargo run --   -A ./testdata/aliases
dices/0.9.4 by Ollivier Robert <roberto@keltia.net>
Small CLI utility to roll dices.

Available commands:
special macros = Macros
builtin dice = Builtin { name: "dice", cmd: Dice }
macro   move = Macro { name: "move", cmd: "dice 3D6 -9" }
alias   roll = Alias { name: "roll", cmd: "dice" }
alias   rulez = Alias { name: "rulez", cmd: "dice" }
macro   mouv = Macro { name: "mouv", cmd: "move +7" }
macro   doom = Macro { name: "doom", cmd: "dice 2D6" }
builtin open = Builtin { name: "open", cmd: Open }
special aliases = Aliases
alias   quit = Alias { name: "quit", cmd: "exit" }
special exit = Exit
special list = List

Dices>
```

If you specify the `-v` flag several times you increase the amount of debugging information displayed. See below for
the format of the `aliases` file.

The main commands the `dices` CLI support are:

- `dice`

  The regular dice everyone know and love. It can be any size, I could have limited to the usual 4, 6, 8, 10, 12, 20
  but I do not see why I should. You can specify multiple dices and even a bonus like in:

  dice 3D6 +2

- `open`

  This is a special dice, you can specify only a sized dice and if the roll is equal to its size, it will reroll again
  until the result is not the max.

  open D8

- `list`

  List all available commands including aliases.

- `aliases`

List only aliases.

- `macros`

List all macros.

- `exit`

  Should be obvious

## Configuring

The `dices` utility supports configuring new command or aliases through the `aliases` file, usually located
on `$HOME/.config/dices` on UNIX systems. Windows is also supported and use the same location for now.

```text
# define a new command
doom = "dice 2D6"
! this is an alias
rulez = dice
// and another one
roll = dice
# or even
quit = exit
# Movement roll
move = "dice 3D6 -9"
# new command with argument
mouv = "move +7"
```

As you can see, you can alias existing commands or create new ones (common usage I expect). You can even create
new command pointing to aliases or other new commands (see `mouv` above which points to `move +7`, etc.).

Some aliases are pre-defined at start to be useful:

- `roll` for `dice`
- `doom` for the special roll of `2D6`

## TODO

- Document, document and more documentation
- Tests and more tests
- Fold `aliases` and `macros` into a specialized `list` through a special closure
- ~~add CLI tests.~~
- ~~Allow alias to existing commands or other aliases~~

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for some simple rules.

I use Git Flow for this package so please use something similar or the usual github workflow.

1. Fork it ( https://github.com/keltia/dmarc-rs/fork )
2. Checkout the develop branch (`git checkout develop`)
3. Create your feature branch (`git checkout -b my-new-feature`)
4. Commit your changes (`git commit -am 'Add some feature'`)
5. Push to the branch (`git push origin my-new-feature`)
6. Create a new Pull Request

NOTE: Pull Request on the "main" branch will be closed without commit.

[Go]: https://golang.org/

[Mercurial]: https://mercurial-scm.org/

[Ruby]: https://ruby-lang.org/

[Rust]: https://rust-lang.org/

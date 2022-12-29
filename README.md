# dices-rs

> **Dice simulator**

## Build Status

Branch: main  [![main](https://github.com/keltia/dices-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/keltia/dices-rs/actions/workflows/rust.yml)

## Description

Usual small Dice simulator for AD&D type of games.

Previous unreleased version was a straight port of the [Ruby] code in https://bitbucket.org/keltia/addfh-utils
(Using Mercurial), rewritten before in [Go] [here](https://github.com/keltia/dices-go) and now rewritten to use [Rust].

## Installation

    cargo install dices-rs

or get the source and build:

    git clone https://github.com/keltia/dices-rs
    cd dices-rs
    cargo test
    cargo bench
    cargo install --path .

The binary will be installed wherever it is defined on your machine and the library in `dice` itself will be compiled
and available.

## Basic commands

The main commands the `dices` CLI support are:

```text
dices/0.8.1 by Ollivier Robert <roberto@keltia.net>
Small CLI utility to roll dices.

Dices>
```

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

- `exit`

  Should be obvious

## Configuring

The `dices` utility supports configuring new command or aliases through the `aliases` file, usually located
on `$HOME/.config/dices`
on UNIX systems. Windows is also supported and use the same location for now.

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

As you can see, you can alias existing commands (not other alias or commands yet!) or create new ones (common usage I
expect).

Some aliases are pre-defined at start to be useful:

- `roll` for `dice`
- `doom` for the special roll of `2D6`

[Go]: https://golang.org/

[Ruby]: https://ruby-lang.org/

[Rust]: https://rust-lang.org/

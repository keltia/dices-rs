# dices-rs

> **Dice simulator**

## Build Status

[![main](https://github.com/keltia/dices-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/keltia/dices-rs/actions/workflows/rust.yml)
[![develop](https://github.com/keltia/dices-rs/actions/workflows/develop.yml/badge.svg)](https://github.com/keltia/dices-rs/actions/workflows/develop.yml)
[![dependency status](https://deps.rs/repo/github/keltia/dices-rs/status.svg)](https://deps.rs/repo/github/keltia/dices-rs)
[![Docs](https://img.shields.io/docsrs/dices-rs)](https://docs.rs/dices-rs)
[![GitHub release](https://img.shields.io/github/release/keltia/dices-rs.svg)](https://github.com/keltia/dices-rs/releases/)
[![GitHub issues](https://img.shields.io/github/issues/keltia/dices-rs.svg)](https://github.com/keltia/dices-rs/issues)
[![dices-rs: 1.85+](https://img.shields.io/badge/Rust%20version-1.85%2B-lightgrey)][Rust 1.85]
[![SemVer](https://img.shields.io/badge/semver-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
[![License](https://img.shields.io/crates/l/mit)](https://opensource.org/licenses/MIT)

## Description

Usual small Dice simulator for AD&D type of games.

Previous unreleased versions were a straight port of the [Ruby] code in https://bitbucket.org/keltia/addfh-utils
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

Usage: dices [OPTIONS] [COMMANDS]...

Arguments:
  [COMMANDS]...  Commands to execute (non-interactive mode)

Options:
  -A, --alias-file <ALIAS_FILE>  Alias file
  -v, --verbose...               Verbose mode
  -V, --version                  Display utility full version
  -h, --help                     Print help
```

Example:

```text
$ dices
dices/0.21.0 by Ollivier Robert <roberto@keltia.net>
Small CLI utility to roll dices.

/Users/roberto/.config/dices/aliases loaded.

Available commands:
special aliases
builtin dice
macro   doom
special exit
special list
alias   llist
special macros
macro   mouv
macro   move
builtin open
alias   quit
alias   roll
alias   rulez

>>
```

If you specify the `-v` flag several times you increase the amount of debugging information displayed. See below for
the format of the `aliases` file.

It also supports non-interactive mode:

    $ echo "dice d20" | dices 

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

The default alias file is `$HOME/.config/dices/aliases` on UNIX systems, and `$LOCALAPPDATA/dices/aliases` on Windows.

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
- ~~Specifying the commands as a yaml file at compile time with `include_str!`~~
- ~~add CLI tests.~~
- ~~Allow alias to existing commands or other aliases~~

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for some simple rules.

I use Git Flow for this package so please use something similar or the usual github workflow.

1. Fork it ( https://github.com/keltia/dices-rs/fork )
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

[Rust 1.85]: https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/

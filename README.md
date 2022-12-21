# dices-rs

> **Dice simulator**

## Build Status

Branch: main  [![main](https://github.com/keltia/dices-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/keltia/dices-rs/actions/workflows/rust.yml)

## Description

Usual small Dice simulator for AD&D type of games.

Previous unreleased version was a straight port of the [Ruby] code in https://bitbucket.org/keltia/addfh-utils (Using Mercurial),
rewritten before in [Go] [here](https://github.com/keltia/dices-go) and now rewritten to use [Rust].

## Installation

    cargo install dices-rs

or get the source and build:

    git clone https://github.com/keltia/dices-rs
    cd dices-rs
    cargo test
    cargo bench
    cargo install --path .

The binary will be installed wherever it is defined on your machine and the library in `dice` itself will be compiled and available.

[Go]: https://golang.org/
[Ruby]: https://ruby-lang.org/
[Rust]: https://rust-lang.org/

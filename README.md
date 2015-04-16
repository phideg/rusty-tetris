# Rusty Tetris [![Build Status](https://travis-ci.org/PistonDevelopers/rusty-tetris.svg?branch=master)](https://travis-ci.org/PistonDevelopers/rusty-tetris)


A Tetris clone written in Rust.

![screenshot](rustytetris.png?raw=true)


The fall speed increases every 10 tetrominoes.

## Keys:
- E / Q or Up => rotate
- A / D or Left / Right => move
- S or Down => drop
- F1 => restart after losing


## How to build & run

You need the latest Rust and Cargo installed (see [here](http://www.rust-lang.org/install.html)).

1. `git clone ...` the repository
2. cd into the `rusty-tetris` directory
3. Type `cargo build`
4. Type `cargo run`

## Dependencies:

The project uses the [piston game engine](https://github.com/PistonDevelopers/piston)

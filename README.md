# Rusty Tetris [![Build Status](https://travis-ci.org/PistonDevelopers/rusty-tetris.svg?branch=master)](https://travis-ci.org/PistonDevelopers/rusty-tetris)


A Tetris clone written in Rust.

![screenshot](rustytetris.png?raw=true)


The fall speed increases every 10 tetrominoes.

## Keys:
- E / Q or Up => rotate
- A / D or Left / Right => move
- S or Down => drop
- F1 => restart after losing

## Command line options:
By default the game starts with a resolution of 600x800. With the `-m` option a minified version gets rendered which should also work on smaller screens. 

## How to build & run

prerequisites:
- Rust 1.0 and Cargo (see [here](http://www.rust-lang.org/install.html))
- freetype (see [here](https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started#installing-dependencies))

1. `git clone ...` the repository
2. cd into the `rusty-tetris` directory
3. Type `cargo build`
4. Type `cargo run`

## Dependencies:

The project uses the [piston game engine](https://github.com/PistonDevelopers/piston) with the glutin backend.


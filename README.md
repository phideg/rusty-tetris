# Rusty Tetris [![Build Status](https://travis-ci.org/PistonDevelopers/rusty-tetris.svg?branch=master)](https://travis-ci.org/PistonDevelopers/rusty-tetris)


A Tetris clone written in Rust.

![screenshot](rustytetris.png?raw=true)


The fall speed increases every 10 tetrominoes.

## Keys:
- E / Q or Up => rotate
- A / D or Left / Right => move
- S or Down => fast move
- space => drop immediately
- F1 => restart the game at any time

## Command line options
- `-m` By default the game starts with a resolution of 600x800. With the `-m` option a minified version gets rendered which should also work on smaller screens. 
- `-o` Switches off the background music
- `-i` specifies the number of initial lines whose cells should be randomly filled

## How to build & run

Prerequisites:
- Rust 1.0 and Cargo (see [here](http://www.rust-lang.org/install.html))
- install [External Dependencies](#external-dependencies)

Build & run:

1. `git clone ...` the repository
2. cd into the `rusty-tetris` directory
3. `cargo run --release`

## Dependencies

The project uses the [piston game engine](https://github.com/PistonDevelopers/piston) with the glutin backend.

## External Dependencies

The piston game engine currently depends on a few external non-Rust libraries
- SDL and SDL2_mixer (see [rust-sdl2 library README](https://github.com/AngryLawyer/rust-sdl2#requirements))
- freetype (see [here](https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started#installing-dependencies))

## Assets
The background music
- gravitationalWaves by airtone (c) copyright 2016 Licensed under a Creative Commons Attribution Noncommercial  (3.0) license. http://dig.ccmixter.org/files/airtone/55021 

## Remark
The initial [Rusty Tetris](https://github.com/bachm/rusty-tetris) version was implemented by [bachm](https://github.com/bachm).

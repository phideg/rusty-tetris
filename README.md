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
- Rust 1.0 or newer (best way to install is [rustup](https://www.rustup.rs/))
- install [External Dependencies](#external-dependencies)

Build & run:

1. `git clone ...` the repository
2. cd into the `rusty-tetris` directory
3. `cargo run --release`

Windows:

if you use the msvc based version of Rust you have to take care to install the msvc development libs of SDL2 and SDL_mixer.

1. download [SDL2-devel-2.x.x-VC.zip](https://www.libsdl.org/download-2.0.php)
2. download [SDL2_mixer-devel-2.x.x-VC.zip](https://www.libsdl.org/projects/SDL_mixer/)
3. copy the *.lib files in your Rust lib directory typically `%HOMEPATH%\.multirust\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib`
4. finally you need to copy the *.dll files next to rusty-tetris executable. If you run rusty-tetris with cargo you place them also in the projects root directory.


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

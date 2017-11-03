extern crate piston_window;
extern crate music;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate rand;
extern crate clap;

use piston_window::*;
use clap::{App, Arg};

mod tetromino;
mod active;
mod tetris;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    // gravitationalWaves by airtone (c)
    // copyright 2016 Licensed under a Creative Commons Attribution Noncommercial  (3.0) license.
    // http://dig.ccmixter.org/files/airtone/55021
    Waves,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Sound { }

fn main() {
    let matches = App::new("rusty-tetris")
        .about("Simple Tetris clone written in Rust")
        .version("0.0.4")
        .arg(
            Arg::with_name("initial_stack_size")
                .short("i")
                .long("initial_stack_size")
                .help("Deteremines the number of lines to be filled randomly")
                .multiple(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("music_off")
                .short("o")
                .long("music_off")
                .help("Turns off the music")
                .multiple(false),
        )
        .arg(
            Arg::with_name("mini")
                .short("m")
                .long("mini")
                .help("Minified rendering for screens < 600x800")
                .multiple(false),
        )
        .get_matches();
    let mut initial_stack_size: usize = 0;
    if let Some(ref stack_size_str) = matches.value_of("initial_stack_size") {
        initial_stack_size = stack_size_str.parse::<usize>().unwrap();
    }
    let music_off = matches.is_present("music_off");
    let mini = matches.is_present("mini");
    let (width, height) = (tetris::WINDOW_WIDTH, tetris::WINDOW_HEIGHT);
    let (width, height) = if mini {
        (width / 2, height / 2)
    } else {
        (width, height)
    };
    let mut window: PistonWindow =
        WindowSettings::new("Rusty Tetris", [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let basic_block = Texture::from_path(
        &mut window.factory,
        &(assets.join("block.png")),
        Flip::None,
        &TextureSettings::new(),
    ).unwrap_or_else(|e| panic!("Failed to load assets: {}", e));
    let mut game = tetris::Tetris::new(
        if mini { 0.5 } else { 1.0 },
        &basic_block,
        initial_stack_size,
    );

    music::start::<Music, Sound, _>(16, || {
        music::bind_music_file(
            Music::Waves,
            &(assets.join("airtone-gravitationalWaves.ogg")),
        );
        if !music_off {
            music::set_volume(0.2);
            music::play_music(&Music::Waves, music::Repeat::Forever);
        }
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, gl| {
                clear([1.0; 4], gl);
                game.render(&c, gl);
            });

            if let Some(uargs) = e.update_args() {
                game.update(&uargs);
            }

            if let Some(Button::Keyboard(key)) = e.press_args() {
                game.key_press(&key);
            }

            if let Some(Button::Keyboard(key)) = e.release_args() {
                game.key_release(&key);
            }
        }
    })
}

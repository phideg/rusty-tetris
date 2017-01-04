#[macro_use]
extern crate piston_window;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate rand;
extern crate clap;

use piston_window::*;
use clap::{App, Arg};

mod tetromino;
mod active;
mod tetris;

fn main() {
    let matches = App::new("rusty-tetris")
        .about("Simple Tetris clone written in Rust")
        .version("0.0.3")
        .arg(Arg::with_name("mini")
            .short("m")
            .help("Use this option for screen resolutions < 600x800")
            .multiple(false))
        .get_matches();

    let mini = matches.is_present("mini");
    let (width, height) = (tetris::WINDOW_WIDTH, tetris::WINDOW_HEIGHT);
    let (width, height) = if mini {
        (width / 2, height / 2)
    } else {
        (width, height)
    };
    let mut window: PistonWindow = WindowSettings::new("Rusty Tetris", [width, height])
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let basic_block = Texture::from_path(&mut window.factory,
                                         &(assets.join("block.png")),
                                         Flip::None,
                                         &TextureSettings::new())
        .unwrap_or_else(|e| panic!("Failed to load assets: {}", e));
    let mut game = tetris::Tetris::new(if mini { 0.5 } else { 1.0 }, &basic_block);

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
}

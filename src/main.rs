use clap::Parser;
use piston_window::*;

mod active;
mod tetris;
mod tetromino;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    // gravitationalWaves by airtone (c)
    // copyright 2016 Licensed under a Creative Commons Attribution Noncommercial  (3.0) license.
    // http://dig.ccmixter.org/files/airtone/55021
    Waves,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Sound {}

#[derive(Parser)]
#[command(version, about, long_about = Some("A simple Tetris clone written in Rust"))]
struct CliArgs {
    /// Deteremines the number of lines to be filled randomly
    #[arg(short = 'i', long, default_value_t = 0)]
    initial_stack_size: usize,

    /// Turns off the music
    #[arg(short = 'o', long)]
    music_off: bool,

    /// Minified rendering for screens < 600x800
    #[arg(short = 'm', long)]
    mini: bool,
}

fn main() {
    let CliArgs {
        initial_stack_size,
        music_off,
        mini,
    } = CliArgs::parse();
    let (width, height) = (tetris::WINDOW_WIDTH, tetris::WINDOW_HEIGHT);
    let (width, height) = if mini {
        (width / 2, height / 2)
    } else {
        (width, height)
    };
    let mut window: PistonWindow = WindowSettings::new("Rusty Tetris", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let basic_block = Texture::from_path(
        &mut window.create_texture_context(),
        assets.join("block.png"),
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap_or_else(|e| panic!("Failed to load assets: {}", e));
    let mut game = tetris::Tetris::new(
        if mini { 0.5 } else { 1.0 },
        basic_block,
        initial_stack_size,
    );

    music::start::<Music, Sound, _>(16, || {
        music::bind_music_file(Music::Waves, assets.join("airtone-gravitationalWaves.ogg"));
        if !music_off {
            music::set_volume(0.2);
            music::play_music(&Music::Waves, music::Repeat::Forever);
        }
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, gl, _| {
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

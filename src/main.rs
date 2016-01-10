extern crate rand;
extern crate piston_window;
extern crate gfx_device_gl;

use std::path::Path;
use piston_window::*;

mod tetromino;
mod active;
mod tetris;

fn main() { 
    let mini = false;
    let (width, height) = (400, 800);
    let (width, height) = if mini { (width / 2, height / 2) } else { (width, height) };
    let window: PistonWindow = 
        WindowSettings::new("Rusty Tetris", [width, height])
        .exit_on_esc(true)        
        .build()
        .unwrap();
    
    let basic_block = Texture::from_path(
        &mut *window.factory.borrow_mut(),
        &(Path::new("./bin/assets/block.png")),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    let mut game = tetris::Tetris::new(if mini { 0.5 } else { 1.0 }, &basic_block);
    
    for e in window {
        e.draw_2d(|c, gl| {
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

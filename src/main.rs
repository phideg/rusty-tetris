#![feature(negate_unsigned)]
extern crate rand;
extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::rc::Rc;
use std::cell::RefCell;
use opengl_graphics::{ GlGraphics, OpenGL };
use sdl2_window::Sdl2Window;
use piston::window::{ WindowSettings, Size };
use piston::event::*;

mod tetromino;
mod active;
mod tetris;

fn main() {
    let mini = false;
    let (width, height) = (400, 800);
    let (width, height) = if mini { (width / 2, height / 2) } else { (width, height) };
    let opengl = OpenGL::_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings::new(
            "Rusty Tetris".to_string(),
            Size { width: width, height: height })
            .exit_on_esc(true)
            .fullscreen(false)
    );

    let mut game = tetris::Tetris::new(if mini { 0.5 } else { 1.0 });
    let ref mut gl = GlGraphics::new(opengl);
    let window = Rc::new(RefCell::new(window));
    for e in window.events() {
        use piston::input::Button;

        if let Some(args) = e.render_args() {
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                graphics::clear([1.0; 4], gl);
                game.render(&c, gl);
            });
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_press(&key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            game.key_release(&key);
        }

        if let Some(uargs) = e.update_args() {
            game.update(&uargs);
        }

    }
}

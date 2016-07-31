use std::default::Default;
use gfx_device_gl::Resources;
//use gfx_core::Resources;
use piston_window::*;

use active::ActiveTetromino;
use tetromino::Color;
use tetris::State::*;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
static TILE_SIZE: f64 = 40.0;

pub const UPDATE_TIME: f64 = 0.15;


#[derive(PartialEq, Copy, Clone)]
enum State {
    Playing,
    Dropping,
    Defeated
}

#[derive(Debug, Copy, Clone)]
enum KeyStateType {
    Released,
    Pressed,
    PressedLongTime
}

#[derive(Debug)]
struct KeyState {
    state_type : KeyStateType,
    press_count : i32
}

impl KeyState {
    fn new() -> KeyState {
        KeyState {
            state_type: KeyStateType::Released,
            press_count: 0
        }
    }

    fn update_on_press(&mut self) {
        if let KeyStateType::Released = self.state_type {
            self.state_type = KeyStateType::Pressed;
            self.press_count += 1;
        }
    }

    fn update_on_release(&mut self) {
        match self.state_type {
            KeyStateType::Pressed => {
                self.state_type = KeyStateType::Released;
            },
            KeyStateType::PressedLongTime => {
                self.reset();
            },
            _ => {}
        }
    }

    fn update_by_time(&mut self) {
        match (self.state_type, self.press_count) {
            (KeyStateType::Pressed, 1) => {
                self.state_type = KeyStateType::PressedLongTime;
            },
            (KeyStateType::PressedLongTime, _) => {},
            _ => {
                self.reset();
            }
        }
    }

    fn is_active(&self) -> bool {
        self.press_count > 0
    }

    fn reset(&mut self) {
        self.state_type = KeyStateType::Released;
        self.press_count = 0;
    }
}

pub struct ControlState {
    rotate_right : KeyState,
    rotate_left : KeyState,
    move_left : KeyState,
    move_right : KeyState
}

pub struct Tetris<'a> {
    gravity_accumulator: f64,
    gravity_factor: f64,
    tetromino_count: usize,
    active_tetromino: ActiveTetromino,
    board: [[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT],
    state: State,
    control_state : ControlState,
    time: f64,
    block: &'a Texture<Resources>,
    paused: bool,
    scale: f64,
}

impl<'a> Tetris<'a> {
    pub fn new(scale: f64, texture: &'a Texture<Resources>) -> Tetris {
        Tetris {
            gravity_accumulator: 0.0,
            gravity_factor: 0.5,
            tetromino_count: 0,
            active_tetromino: ActiveTetromino::new(),
            board: [[Default::default(); BOARD_WIDTH]; BOARD_HEIGHT],
            state: Playing,
            control_state: ControlState {
                rotate_right: KeyState::new(),
                rotate_left: KeyState::new(),
                move_left: KeyState::new(),
                move_right: KeyState::new()
            },
            time: UPDATE_TIME,
            block: texture,
            paused: false,
            scale: scale,
        }
    }

    fn gravity(&mut self, amount: f64) {
        self.gravity_accumulator += amount * self.gravity_factor;
        if self.gravity_accumulator >= 0.35 {
            self.gravity_accumulator = 0.0;
            if ! self.active_tetromino.try_move_down(&self.board) {
                for &(x,y) in self.active_tetromino.as_points().iter() {
                    if y < self.board.len() && x < self.board[y].len() {
                        self.board[y][x] = Some(self.active_tetromino.get_color());
                    } else {
                        self.state = Defeated;
                    }
                }
                if self.state == Playing || self.state == Dropping {
                    self.state = Playing;
                    let mut board: [[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT] = [[None; BOARD_WIDTH]; BOARD_HEIGHT];
                    for (new,old) in board.iter_mut().rev().zip(self.board.iter().rev().filter(|row| row.iter().any(|color| color.is_none()))) {
                        *new = (*old).clone();
                    }
                    self.board = board;
                    self.active_tetromino = ActiveTetromino::new();
                    self.tetromino_count += 1;
                    if self.tetromino_count >= 10 {
                        self.tetromino_count = 0;
                        self.gravity_factor *= 1.1;
                    }
                }
            }
        }
    }

    fn play_again(&mut self) {
        self.state = Playing;
        self.gravity_accumulator = 0.0;
        self.tetromino_count = 0;
        self.gravity_factor = 0.5;
        self.board = [[Default::default(); BOARD_WIDTH]; BOARD_HEIGHT];
        self.active_tetromino = ActiveTetromino::new();
    }

    pub fn render(&mut self, c: &Context, g: &mut G2d) {
        let c = c.zoom(self.scale);
        fn pos(n: usize) -> f64 { n as f64 * TILE_SIZE }
        for y in 0usize..BOARD_HEIGHT {
            for x in 0usize..BOARD_WIDTH {
                self.board[y][x].as_ref()
                    .map(|e| Image::new_color(e.as_rgba())
                                  .draw(self.block, &Default::default(),
                                        c.trans(pos(x), pos(y)).transform, g));
            }
        }
        for &(x,y) in self.active_tetromino.as_points().iter() {
            Image::new_color(self.active_tetromino.get_color().as_rgba())
                 .draw(self.block, &Default::default(), c.trans(pos(x), pos(y)).transform, g);
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if self.paused { return }

        self.time += args.dt;

        if self.time > UPDATE_TIME {
            if self.control_state.rotate_right.is_active() {
                for _ in 0..self.control_state.rotate_right.press_count {
                    self.active_tetromino.try_rotate_right(&self.board);
                }

                self.control_state.rotate_right.update_by_time();
            }

            if self.control_state.rotate_left.is_active() {
                for _ in 0..self.control_state.rotate_left.press_count {
                    self.active_tetromino.try_rotate_left(&self.board);
                }
                self.control_state.rotate_left.update_by_time();
            }

            if self.control_state.move_left.is_active() {
                for _ in 0..self.control_state.move_left.press_count {
                    self.active_tetromino.try_move_left(&self.board);
                }
                self.control_state.move_left.update_by_time();
            }

            if self.control_state.move_right.is_active() {
                for _ in 0..self.control_state.move_right.press_count {
                    self.active_tetromino.try_move_right(&self.board);
                }
                self.control_state.move_right.update_by_time();
            }

            self.time -= UPDATE_TIME;
        }

        match self.state {
            Playing     => self.gravity(args.dt),
            Dropping    => self.gravity(0.12 + args.dt),
            _ => {}
        }
    }

    pub fn key_press(&mut self, key: &Key) {
        match (self.state, key) {
            (Defeated, &Key::F1)
                => self.play_again(),
            (Defeated, _) 
                => {},
            (Playing,  &Key::P)
                => self.paused = !self.paused,
            (_,  &Key::E) if !self.paused
                => self.control_state.rotate_right.update_on_press(),
            (_,  &Key::Up)    | (_, &Key::Q) if !self.paused
                => self.control_state.rotate_left.update_on_press(),
            (_,  &Key::Left)  | (_, &Key::A) if !self.paused
                => self.control_state.move_left.update_on_press(),
            (_,  &Key::Right) | (_, &Key::D) if !self.paused
                => self.control_state.move_right.update_on_press(),
            (_,  &Key::Down)  | (_, &Key::S) if !self.paused
                => self.state = Dropping,
            _ => {}
        }
    }

    pub fn key_release(&mut self, key: &Key) {
        match (self.state, key) {
            (Dropping,  &Key::Down)  | (Dropping, &Key::S) if !self.paused
                => self.state = Playing,
            (_,  &Key::E) if !self.paused
                => self.control_state.rotate_right.update_on_release(),
            (_,  &Key::Up)    | (_, &Key::Q) if !self.paused
                => self.control_state.rotate_left.update_on_release(),
            (_,  &Key::Left)  | (_, &Key::A) if !self.paused
                => self.control_state.move_left.update_on_release(),
            (_,  &Key::Right) | (_, &Key::D) if !self.paused
                => self.control_state.move_right.update_on_release(),
            _ => {}
        }
    }
}

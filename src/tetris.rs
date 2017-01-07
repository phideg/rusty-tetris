use std::default::Default;
use rand::{thread_rng, Rng};
use gfx_device_gl::Resources;
// use gfx_core::Resources;
use piston_window::*;

use tetromino::{Tetromino, Color, Rotation};
use active::ActiveTetromino;
use tetris::State::*;

pub const TILE_SIZE: f64 = 40.0;
pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
pub const SIDEBAR_WIDTH: usize = 5;
pub const WINDOW_WIDTH: u32 = (BOARD_WIDTH + SIDEBAR_WIDTH) as u32 * TILE_SIZE as u32;
pub const WINDOW_HEIGHT: u32 = BOARD_HEIGHT as u32 * TILE_SIZE as u32;

pub const UPDATE_TIME: f64 = 0.15;


#[derive(PartialEq, Copy, Clone)]
enum State {
    Playing,
    Dropping,
    Defeated,
}

#[derive(Debug, Copy, Clone)]
enum KeyStateType {
    Released,
    Pressed,
    PressedLongTime,
}

#[derive(Debug)]
struct KeyState {
    state_type: KeyStateType,
    press_count: i32,
}

impl KeyState {
    fn new() -> KeyState {
        KeyState {
            state_type: KeyStateType::Released,
            press_count: 0,
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
            }
            KeyStateType::PressedLongTime => {
                self.reset();
            }
            _ => {}
        }
    }

    fn update_by_time(&mut self) {
        match (self.state_type, self.press_count) {
            (KeyStateType::Pressed, 1) => {
                self.state_type = KeyStateType::PressedLongTime;
            }
            (KeyStateType::PressedLongTime, _) => {}
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
    rotate_right: KeyState,
    rotate_left: KeyState,
    move_left: KeyState,
    move_right: KeyState,
}

pub struct Board {
    board: [[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT],
    next_shape: &'static Tetromino,
    active_tetromino: ActiveTetromino,
    line_count: usize,
    tetromino_count: usize,
    control_state: ControlState,
    gravity_accumulator: f64,
    gravity_factor: f64,
    offset: usize,
    state: State,
}

impl Board {
    fn new(offset: usize, initial_stack_size: usize) -> Board {
        Board {
            tetromino_count: 0,
            line_count: 0,
            active_tetromino: ActiveTetromino::new(Tetromino::get_random_shape()),
            next_shape: Tetromino::get_random_shape(),
            board: Tetris::create_board(initial_stack_size),
            gravity_accumulator: 0.0,
            gravity_factor: 0.8,
            control_state: ControlState {
                rotate_right: KeyState::new(),
                rotate_left: KeyState::new(),
                move_left: KeyState::new(),
                move_right: KeyState::new(),
            },
            offset: offset,
            state: Playing,
        }
    }

    fn print_digit(&mut self, digit: usize, x_offset: usize) {
        match digit {
            0 => {
                self.board[1][0 + x_offset] = Some(Color::Grey);
                self.board[1][1 + x_offset] = Some(Color::Grey);
                self.board[1][2 + x_offset] = Some(Color::Grey);
                self.board[2][0 + x_offset] = Some(Color::Grey);
                self.board[2][2 + x_offset] = Some(Color::Grey);
                self.board[3][0 + x_offset] = Some(Color::Grey);
                self.board[3][2 + x_offset] = Some(Color::Grey);
                self.board[4][0 + x_offset] = Some(Color::Grey);
                self.board[4][2 + x_offset] = Some(Color::Grey);
                self.board[5][0 + x_offset] = Some(Color::Grey);
                self.board[5][1 + x_offset] = Some(Color::Grey);
                self.board[5][2 + x_offset] = Some(Color::Grey);

            }
            1 => {
                self.board[1][2 + x_offset] = Some(Color::Grey);
                self.board[2][1 + x_offset] = Some(Color::Grey);
                self.board[2][2 + x_offset] = Some(Color::Grey);
                self.board[3][2 + x_offset] = Some(Color::Grey);
                self.board[4][2 + x_offset] = Some(Color::Grey);
                self.board[5][2 + x_offset] = Some(Color::Grey);
            }
            2 => {
                self.board[1][0 + x_offset] = Some(Color::Grey);
                self.board[1][1 + x_offset] = Some(Color::Grey);
                self.board[1][2 + x_offset] = Some(Color::Grey);
                self.board[2][2 + x_offset] = Some(Color::Grey);
                self.board[3][1 + x_offset] = Some(Color::Grey);
                self.board[4][0 + x_offset] = Some(Color::Grey);
                self.board[5][0 + x_offset] = Some(Color::Grey);
                self.board[5][1 + x_offset] = Some(Color::Grey);
                self.board[5][2 + x_offset] = Some(Color::Grey);
            }
            3 => {
                self.board[1][0 + x_offset] = Some(Color::Grey);
                self.board[1][1 + x_offset] = Some(Color::Grey);
                self.board[1][2 + x_offset] = Some(Color::Grey);
                self.board[2][2 + x_offset] = Some(Color::Grey);
                self.board[3][1 + x_offset] = Some(Color::Grey);
                self.board[4][2 + x_offset] = Some(Color::Grey);
                self.board[5][2 + x_offset] = Some(Color::Grey);
                self.board[5][1 + x_offset] = Some(Color::Grey);
                self.board[5][0 + x_offset] = Some(Color::Grey);
            }
            4 => {
                self.board[1][0 + x_offset] = Some(Color::Grey);
                self.board[2][0 + x_offset] = Some(Color::Grey);
                self.board[3][0 + x_offset] = Some(Color::Grey);
                self.board[3][1 + x_offset] = Some(Color::Grey);
                self.board[3][2 + x_offset] = Some(Color::Grey);
                self.board[4][1 + x_offset] = Some(Color::Grey);
                self.board[5][1 + x_offset] = Some(Color::Grey);
            }
            5 => {
                self.board[1][0 + x_offset] = Some(Color::Grey);
                self.board[1][1 + x_offset] = Some(Color::Grey);
                self.board[1][2 + x_offset] = Some(Color::Grey);
                self.board[2][0 + x_offset] = Some(Color::Grey);
                self.board[3][0 + x_offset] = Some(Color::Grey);
                self.board[3][1 + x_offset] = Some(Color::Grey);
                self.board[3][2 + x_offset] = Some(Color::Grey);
                self.board[4][2 + x_offset] = Some(Color::Grey);
                self.board[5][1 + x_offset] = Some(Color::Grey);
                self.board[5][0 + x_offset] = Some(Color::Grey);
            }
            6 => {
                self.board[1][1 + x_offset] = Some(Color::Grey);
                self.board[1][2 + x_offset] = Some(Color::Grey);
                self.board[2][0 + x_offset] = Some(Color::Grey);
                self.board[3][0 + x_offset] = Some(Color::Grey);
                self.board[3][1 + x_offset] = Some(Color::Grey);
                self.board[4][0 + x_offset] = Some(Color::Grey);
                self.board[4][2 + x_offset] = Some(Color::Grey);
                self.board[5][0 + x_offset] = Some(Color::Grey);
                self.board[5][1 + x_offset] = Some(Color::Grey);
                self.board[5][2 + x_offset] = Some(Color::Grey);
            }
            7 => {
                self.board[1][0 + x_offset] = Some(Color::Grey);
                self.board[1][1 + x_offset] = Some(Color::Grey);
                self.board[1][2 + x_offset] = Some(Color::Grey);
                self.board[2][2 + x_offset] = Some(Color::Grey);
                self.board[3][1 + x_offset] = Some(Color::Grey);
                self.board[4][1 + x_offset] = Some(Color::Grey);
                self.board[5][1 + x_offset] = Some(Color::Grey);
            }
            8 => {
                self.board[1][1 + x_offset] = Some(Color::Grey);
                self.board[2][0 + x_offset] = Some(Color::Grey);
                self.board[2][2 + x_offset] = Some(Color::Grey);
                self.board[3][1 + x_offset] = Some(Color::Grey);
                self.board[4][0 + x_offset] = Some(Color::Grey);
                self.board[4][2 + x_offset] = Some(Color::Grey);
                self.board[5][1 + x_offset] = Some(Color::Grey);
            }
            9 => {
                self.board[1][0 + x_offset] = Some(Color::Grey);
                self.board[1][1 + x_offset] = Some(Color::Grey);
                self.board[1][2 + x_offset] = Some(Color::Grey);
                self.board[2][0 + x_offset] = Some(Color::Grey);
                self.board[2][2 + x_offset] = Some(Color::Grey);
                self.board[3][0 + x_offset] = Some(Color::Grey);
                self.board[3][1 + x_offset] = Some(Color::Grey);
                self.board[3][2 + x_offset] = Some(Color::Grey);
                self.board[4][2 + x_offset] = Some(Color::Grey);
                self.board[5][0 + x_offset] = Some(Color::Grey);
                self.board[5][1 + x_offset] = Some(Color::Grey);
                self.board[5][2 + x_offset] = Some(Color::Grey);
            }      
            _ => {}
        }
    }

    fn show_result(&mut self) {
        self.board = [[Default::default(); BOARD_WIDTH]; BOARD_HEIGHT];
        let first_digit = self.line_count % 10;
        self.print_digit(first_digit, 5);
        let second_digit = (self.line_count - first_digit) / 10;
        self.print_digit(second_digit, 1);
    }

    fn gravity(&mut self, amount: f64) {
        self.gravity_accumulator += amount * self.gravity_factor;
        if self.gravity_accumulator >= 0.35 {
            self.gravity_accumulator = 0.0;
            if !self.active_tetromino.try_move_down(&self.board) {
                for &(x, y) in self.active_tetromino.as_points().iter() {
                    if y < self.board.len() && x < self.board[y].len() {
                        self.board[y][x] = Some(self.active_tetromino.get_color());
                    } else {
                        self.state = Defeated;
                    }
                }
                if self.state == Playing || self.state == Dropping {
                    self.state = Playing;
                    let mut board: [[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT] =
                        [[None; BOARD_WIDTH]; BOARD_HEIGHT];
                    let mut full_line_count = BOARD_HEIGHT;
                    for (new, old) in board.iter_mut().rev().zip(self.board
                        .iter()
                        .rev()
                        .filter(|row| row.iter().any(|color| color.is_none()))) {
                        *new = (*old).clone();
                        full_line_count -= 1;
                    }
                    self.board = board;
                    self.line_count += full_line_count;
                    self.active_tetromino = ActiveTetromino::new(self.next_shape);
                    self.next_shape = Tetromino::get_random_shape();
                    self.tetromino_count += 1;
                    if self.tetromino_count >= 10 {
                        self.tetromino_count = 0;
                        self.gravity_factor *= 1.1;
                    }
                }
            }
        }
    }

    fn drop_fully(&mut self) {
        while self.active_tetromino.try_move_down(&self.board) {}
    }

    fn update(&mut self, dt: f64) -> State {
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

        match self.state {
            Playing => self.gravity(dt),
            Dropping => self.gravity(0.12 + dt),
            Defeated => self.show_result(),
        }
        self.state
    }
}


pub struct Tetris<'a> {
    duel_mode: bool,
    boards: Vec<Board>,
    initial_stack_size: usize,
    state: State,
    time: f64,
    block: &'a Texture<Resources>,
    paused: bool,
    scale: f64,
}

impl<'a> Tetris<'a> {
    pub fn new(scale: f64,
               texture: &'a Texture<Resources>,
               initial_stack_size: usize,
               duel_mode: bool)
               -> Tetris {
        let stack_size = if initial_stack_size < BOARD_HEIGHT {
            initial_stack_size
        } else {
            BOARD_HEIGHT - 1
        };
        Tetris {
            duel_mode: duel_mode,
            boards: if duel_mode {
                vec![Board::new(0, stack_size), Board::new(BOARD_WIDTH + SIDEBAR_WIDTH, stack_size)]
            } else {
                vec![Board::new(0, stack_size)]
            },
            initial_stack_size: stack_size,
            state: Playing,
            time: UPDATE_TIME,
            block: texture,
            paused: false,
            scale: scale,
        }
    }

    pub fn create_board(initial_stack_size: usize) -> [[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT] {
        let mut board = [[Default::default(); BOARD_WIDTH]; BOARD_HEIGHT];
        if initial_stack_size > 0 {
            for y in 0usize..initial_stack_size {
                // set random cells within a row
                for x in (0usize..BOARD_WIDTH).filter(|_| thread_rng().gen()) {
                    board[(BOARD_HEIGHT - 1) - y][x] = Some(Color::Grey);
                }
            }
        }
        board
    }

    fn play_again(&mut self) {
        self.state = Playing;
        self.boards = if self.duel_mode {
            vec![Board::new(0, self.initial_stack_size),
                 Board::new(BOARD_WIDTH + SIDEBAR_WIDTH, self.initial_stack_size)]
        } else {
            vec![Board::new(0, self.initial_stack_size)]
        };
    }

    pub fn render(&mut self, c: &Context, g: &mut G2d) {
        let c = c.zoom(self.scale);
        fn pos(n: usize) -> f64 {
            n as f64 * TILE_SIZE
        }
        // render the boards
        for board in &self.boards {
            for y in 0usize..BOARD_HEIGHT {
                for x in 0usize..BOARD_WIDTH {
                    board.board[y][x]
                        .as_ref()
                        .map(|e| {
                            Image::new_color(e.as_rgba()).draw(self.block,
                                                               &Default::default(),
                                                               c.trans(pos(x + board.offset),
                                                                          pos(y))
                                                                   .transform,
                                                               g)
                        });
                }
            }
            if self.state != Defeated {
                for &(x, y) in board.active_tetromino.as_points().iter() {
                    Image::new_color(board.active_tetromino
                            .get_color()
                            .as_rgba())
                        .draw(self.block,
                              &Default::default(),
                              c.trans(pos(x + board.offset), pos(y)).transform,
                              g);
                }
            }
            // render the side bar
            rectangle(Color::Grey.as_rgba(),
                      [0.0, 0.0, pos(SIDEBAR_WIDTH), pos(BOARD_HEIGHT)], // rectangle
                      c.trans(pos(BOARD_WIDTH + board.offset), 0.0).transform,
                      g);
            for &(x, y) in board.next_shape.points(Rotation::R0).iter() {
                Image::new_color(board.next_shape
                        .get_color()
                        .as_rgba())
                    .draw(self.block,
                          &Default::default(),
                          c.trans(pos(BOARD_WIDTH + board.offset) + pos(x + 1), pos(y)).transform,
                          g);
            }
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if self.paused {
            return;
        }
        self.time += args.dt;
        if self.time > UPDATE_TIME {
            for board in &mut self.boards {
                if board.update(args.dt) == Defeated {
                    self.state = Defeated;
                }
            }
            self.time -= UPDATE_TIME;
        }
    }

    pub fn key_press(&mut self, key: &Key) {
        let board_index = self.boards.len() - 1;
        match (self.state, key, self.paused) {
            // general keys
            (Defeated, _, _) => {
                if key == &Key::F1 {
                    self.play_again()
                }
            }
            (Playing, &Key::P, _) => self.paused = !self.paused,
            (_, &Key::F1, _) => self.play_again(),
            // keys of player one
            (_, &Key::Space, false) => self.boards[0].drop_fully(),
            (_, &Key::Down, false) => self.boards[0].state = Dropping,
            (_, &Key::Up, false) => self.boards[0].control_state.rotate_left.update_on_press(),
            (_, &Key::Left, false) => self.boards[0].control_state.move_left.update_on_press(),
            (_, &Key::Right, false) => self.boards[0].control_state.move_right.update_on_press(),
            // keys of player two
            (_, &Key::F, false) => self.boards[board_index].drop_fully(),
            (_, &Key::S, false) => self.boards[board_index].state = Dropping,
            (_, &Key::Q, false) => {
                self.boards[board_index].control_state.rotate_left.update_on_press()
            }
            (_, &Key::A, false) => {
                self.boards[board_index].control_state.move_left.update_on_press()
            }
            (_, &Key::D, false) => {
                self.boards[board_index].control_state.move_right.update_on_press()
            }
            _ => {}
        }
    }

    pub fn key_release(&mut self, key: &Key) {
        if self.paused {
            return;
        }
        let board_index = self.boards.len() - 1;
        match key {
            // player one
            &Key::Down => self.boards[0].state = Playing,
            &Key::Up => self.boards[0].control_state.rotate_left.update_on_release(),
            &Key::Left => self.boards[0].control_state.move_left.update_on_release(),
            &Key::Right => self.boards[0].control_state.move_right.update_on_release(),
            // player two
            &Key::S => self.boards[board_index].state = Playing,
            &Key::Q => self.boards[board_index].control_state.rotate_left.update_on_release(),
            &Key::A => self.boards[board_index].control_state.move_left.update_on_release(),
            &Key::D => self.boards[board_index].control_state.move_right.update_on_release(),
            _ => {}
        }
    }
}
use std::fmt;
use std::collections::HashMap;

struct Cli_interface { }

pub trait GameEvents {
    fn introduction(&self);
    fn show_grid(&self, grid: &Grid);
    fn player_turn(&self, p1_turn: bool) -> Result<usize, ()>;
    fn animate_chip(&self);
    fn invalid_move(&self);
    fn game_over(&self, winner: String);
}

#[derive(PartialEq)]
enum State {
    Done,
    Running
}

pub struct Game {
    grid: Grid,
    p1: String,
    p2: String,
    with_ai: bool,
    state: State,
    winner: String,
}

impl Game {
    pub(crate) fn new(row_size: usize, col_size: usize, with_ai: bool, p1_name: String, p2_name: String) -> Game {
        let grid = Grid::new(row_size, col_size);
        let mut game = Game{
            grid,
            p1: p1_name,
            p2: p2_name,
            with_ai: false,
            state: State::Running,
            winner: "".to_string()
        };
        if with_ai {
            game.p2 = "Computer".to_string();
            game.with_ai = true;
        }
        game
    }

    pub fn start_game<H: GameEvents>(&mut self, handler: H) {
        handler.introduction();
        let mut p1_turn = true;
        while self.state == State::Running {
            handler.show_grid(&self.grid);
            let sel_col = handler.player_turn(p1_turn);
            if sel_col.is_ok() {
                let col_num = sel_col.unwrap();
                if self.grid.insert_chip(col_num, p1_turn).is_err() {
                    handler.invalid_move();
                    continue;
                }
            }
            else{
                continue;
            }
            let result = self.check_win();
            if result.is_some() {
                handler.show_grid(&self.grid);
                let winner = result.unwrap();
                match winner {
                    Chip::P1 => {
                        self.winner = self.p1.clone();
                        handler.game_over(self.winner.clone());
                    }
                    Chip::P2 => {
                        self.winner = self.p2.clone();
                        handler.game_over(self.winner.clone());
                    }
                    Chip::Empty => {
                        println!("error");
                    }
                }
                self.state = State::Done;
                self.post_game();
            }
            p1_turn = !p1_turn;
        }
    }
    fn post_game(&self) {

    }

    fn check_tile(&self, target: Chip, r: i32, c: i32) -> bool{
        if !(r >=0 && r < self.grid.cols[0].items.len() as i32) {
            return false;
        }
        if !(c >= 0 && c < self.grid.cols.len() as i32) {
            return false;
        }
        if target == self.grid.cols[c as usize].items[r as usize] {
            return true;
        }
        return false;
    }

    fn check_win(&self) -> Option<Chip>{
        for c in 0..self.grid.cols.len() as i32 {
            for r in 0..self.grid.cols[0].items.len() as i32 {
                let target = self.grid.cols[c as usize].items[r as usize];
                if target == Chip::Empty {
                    // dont match against empty spaces
                    continue;
                }
                for k in 0..4 {
                    if self.check_tile(target, r, c-1) && self.check_tile(target, r, c-2) &&
                        self.check_tile(target, r, c-3) {
                        return Some(target);
                    }
                    if self.check_tile(target, r-1, c) && self.check_tile(target, r-2, c) &&
                        self.check_tile(target, r-3, c) {
                        return Some(target);
                    }
                    if self.check_tile(target, r-1, c-1) && self.check_tile(target, r-2, c-2) &&
                        self.check_tile(target, r-3, c-3) {
                        return Some(target);
                    }
                    if self.check_tile(target, r+1, c-1) && self.check_tile(target, r+2, c-2) &&
                        self.check_tile(target, r+3, c-3) {
                        return Some(target);
                    }
                    if self.check_tile(target, r+1, c+1) && self.check_tile(target, r+2, c+2) &&
                        self.check_tile(target, r+3, c+3) {
                        return Some(target);
                    }
                    if self.check_tile(target, r-1, c+1) && self.check_tile(target, r-2, c+2) &&
                        self.check_tile(target, r-3, c+3) {
                        return Some(target);
                    }
                }
            }
        }
        return None;
    }

    fn ai_move() {

    }
}

pub struct Grid {
    cols: Vec<Col>
}

impl Grid {
    pub(crate) fn new(row_size: usize, col_size: usize) -> Grid {
        let mut grid = Grid{ cols: vec![] };
        for _ in 0..row_size {
            let col = Col::new(col_size);
            grid.cols.push(col);
        }
        grid
    }

    pub fn insert_chip(&mut self, col: usize, is_p1: bool) -> Result<(), ()> {
        for x in self.cols[col].items.iter_mut().rev() {
            match x {
                Chip::Empty => {
                    if is_p1 {
                        *x = Chip::P1;
                    }
                    else{
                        *x = Chip::P2;
                    }
                    return Ok(());
                }
                _ => {}
            }
        }
        return Err(());
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.cols[0].items.len() {
            for c in 0..self.cols.len() {
                let chip = &self.cols[c].items[r];
                match *chip {
                    Chip::Empty => write!(f, "_"),
                    Chip::P1 => write!(f, "R"),
                    Chip::P2 => write!(f, "Y")
                };
                write!(f, " ");
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

struct Col {
    items: Vec<Chip>
}

impl Col {
    fn new(size: usize) -> Col {
        let mut col = Col{ items: vec![] };
        col.items = Vec::new();
        for _ in 0..size {
            col.items.push(Chip::Empty);
        }
        col
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Chip {
    Empty,
    P1,
    P2
}
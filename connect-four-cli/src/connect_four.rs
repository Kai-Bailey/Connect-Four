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
                    1 => {
                        self.winner = self.p1.clone();
                        handler.game_over(self.winner.clone());
                    }
                    2 => {
                        self.winner = self.p2.clone();
                        handler.game_over(self.winner.clone());
                    }
                    _ => {
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

    fn check_tile(&self, target: u32, r: i32, c: i32) -> bool{
        if !(r >=0 && r < self.grid.rows[0].items.len() as i32) {
            return false;
        }
        if !(c >= 0 && c < self.grid.rows.len() as i32) {
            return false;
        }
        if target == self.grid.rows[c as usize].items[r as usize] {
            return true;
        }
        return false;
    }

    fn check_win(&self) -> Option<u32>{
        for c in 0..self.grid.rows.len() as i32 {
            for r in 0..self.grid.rows[0].items.len() as i32 {
                let target = self.grid.rows[c as usize].items[r as usize];
                if target == 0 {
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

    fn ai_move(&self) {
//        let choice;
        let state = &self.grid.clone();

    }

    fn ai_check_state(state: Grid) -> (i64, i64){
        let mut winVal: i64 = 0;
        let mut chainVal: i64 = 0;
        let mut temp_r: i64 = 0;
        let mut temp_b: i64 = 0;
        let mut temp_br: i64 = 0;
        let mut temp_tr: i64 = 0;

        for i in 0..6 {
            for j in 0..7 {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;

                for k in 0..4 {
                    if j + k < 7 {
                        temp_r += state.rows[j + k].items[i] as i64;
                    }
                    if i + k < 6 {
                        temp_b += state.rows[j].items[i+k] as i64;
                    }
                    if i + k < 6 && j + k < 7 {
                        temp_br += state.rows[j+k].items[i+k] as i64;
                    }
                    if i - k >= 0 && j + k < 7 {
                        temp_tr += state.rows[j + k].items[i - k] as i64;
                    }
                }
                chainVal += temp_r * temp_r + temp_r;
                chainVal += temp_b * temp_b * temp_b;
                chainVal += temp_br * temp_br * temp_br;
                chainVal += temp_tr * temp_tr * temp_tr;

                if i64::abs(temp_r) == 4 {
                    winVal = temp_r;
                }
                else if i64::abs(temp_b) == 4 {
                    winVal = temp_b;
                }
                else if i64::abs(temp_br) == 4 {
                    winVal = temp_br;
                }
                else if i64::abs(temp_tr) == 4 {
                    winVal = temp_tr;
                }
            }
        }
        return (winVal, chainVal);
    }
}

#[derive(Clone)]
pub struct Grid {
    rows: Vec<Row>
}

//impl Clone for Grid {
//    fn clone(&self) -> Self {
//        Grid { rows: vec![] }
//    }
//}

impl Grid {
    pub(crate) fn new(row_size: usize, col_size: usize) -> Grid {
        let mut grid = Grid{ rows: vec![] };
        for _ in 0..row_size {
            let row = Row::new(row_size);
            grid.rows.push(row);
        }
        grid
    }

    pub fn insert_chip(&mut self, col: usize, is_p1: bool) -> Result<(), ()> {
        for r in (0..self.rows.len()).rev() {
            match self.rows[r].items[col] {
                0 => {
                    if is_p1 {
                        self.rows[r].items[col] = 1;
                    }
                    else{
                        self.rows[r].items[col] = 2;
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
        for r in 0..self.rows.len() {
            for c in 0..self.rows[0].items.len() {
                let chip = &self.rows[r].items[c];
                match *chip {
                    0 => write!(f, "_"),
                    1 => write!(f, "R"),
                    2 => write!(f, "Y"),
                    _ => Err(std::fmt::Error)
                };
                write!(f, " ");
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Row {
    items: Vec<u32>
}

impl Row {
    fn new(size: usize) -> Row {
        let mut row = Row{ items: vec![] };
        row.items = Vec::new();
        for _ in 0..size {
            row.items.push(0);
        }
        row
    }
}
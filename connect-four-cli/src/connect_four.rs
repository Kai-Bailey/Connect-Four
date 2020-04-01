use std::fmt;
use std::collections::{VecDeque};
use std::cmp::{max, min};
use rand::Rng;

pub trait GameEvents {
    fn introduction(&self);
    fn show_grid(&self, grid: &Grid);
    fn player_turn_message(&self, p1_turn: bool);
    fn player_turn(&self, col_size: usize) -> Result<usize, ()>;
    fn selected_column(&self, player: String, col: usize);
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
        let col_size = self.grid.rows[0].items.len();
        while self.state == State::Running {
            handler.show_grid(&self.grid);
            handler.player_turn_message(p1_turn);

            if !p1_turn && self.with_ai {
                let col_num = self.ai_move(-1);
                if self.grid.insert_chip(col_num, p1_turn).is_err() {
                    continue;
                }
                handler.selected_column(self.p1.clone(), col_num);
                p1_turn = !p1_turn;
                continue;
            }

            let sel_col = handler.player_turn(col_size);
            if sel_col.is_ok() {
                let col_num = sel_col.unwrap();
                if self.grid.insert_chip(col_num, p1_turn).is_err() {
                    handler.invalid_move();
                    continue;
                }
                if p1_turn {
                    handler.selected_column(self.p1.clone(), col_num);
                }
                else{
                    handler.selected_column(self.p2.clone(), col_num);
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

    fn check_tile(&self, target: i64, r: i32, c: i32) -> bool{
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

    fn check_win(&self) -> Option<i64>{
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

    fn ai_move(&self, ai_move_val: i64) -> usize{
        let state = &self.grid.clone();
        let choice_val = self.ai_max_state(&state, 0, -100000000007, 100000000007, ai_move_val);
        let choice = choice_val.1;
//        let val = choice_val.0;
        return choice as usize;
    }

    fn ai_check_state(state: &Grid) -> (i64, i64){
        let mut win_val: i64 = 0;
        let mut chain_val: i64 = 0;
        let mut temp_r: i64 = 0;
        let mut temp_b: i64 = 0;
        let mut temp_br: i64 = 0;
        let mut temp_tr: i64 = 0;

        let num_rows = state.rows.len();
        let num_cols = state.rows[0].items.len();

        for i in 0..num_rows {
            for j in 0..num_cols {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;

                for k in 0..4 {
                    if j + k < num_cols {
                        temp_r += state.rows[i].items[j+k] as i64;
                    }
                    if i + k < num_rows {
                        temp_b += state.rows[i+k].items[j] as i64;
                    }
                    if i + k < num_rows && j + k < num_cols {
                        temp_br += state.rows[i+k].items[j+k] as i64;
                    }
                    if i as i64 - k as i64 >= 0 && j + k < num_cols {
                        temp_tr += state.rows[i-k].items[j+k] as i64;
                    }
                }
                chain_val += temp_r * temp_r + temp_r;
                chain_val += temp_b * temp_b * temp_b;
                chain_val += temp_br * temp_br * temp_br;
                chain_val += temp_tr * temp_tr * temp_tr;

                if i64::abs(temp_r) == 4 {
                    win_val = temp_r;
                }
                else if i64::abs(temp_b) == 4 {
                    win_val = temp_b;
                }
                else if i64::abs(temp_br) == 4 {
                    win_val = temp_br;
                }
                else if i64::abs(temp_tr) == 4 {
                    win_val = temp_tr;
                }
            }
        }
        return (win_val, chain_val);
    }

    fn ai_value(&self, state: &Grid, depth: u32, alpha: i64, beta: i64, ai_move_val: i64) -> (i64, i64) {
        let val = Game::ai_check_state(&state);
        if depth >= 4 {
            let mut ret_value = 0;
            let win_val = val.0;
            let chain_val = val.1 * ai_move_val;
            ret_value = chain_val;

            if win_val == 4 * ai_move_val {
                ret_value = 999999;
            }
            else if win_val == 4 * ai_move_val * -1 {
                ret_value = 999999 * -1;
            }
            ret_value -= (depth * depth) as i64;

            return (ret_value, -1);
        }
        let win = val.0;
        if win == 4 * ai_move_val {
            return ((999999 - depth * depth) as i64, -1);
        }
        if win == 4 * ai_move_val * -1 {
            return (999999 * -1 - ((depth * depth) as i64), -1);
        }

        if depth % 2 == 0 {
            return self.ai_min_state(state, depth + 1, alpha, beta, ai_move_val);
        }
        return self.ai_max_state(state, depth + 1, alpha, beta, ai_move_val);
    }

    fn ai_max_state(&self, state: &Grid, depth: u32, alpha: i64, beta: i64, ai_move_val: i64) -> (i64, i64){
        let mut v:i64 = -100000000007;
        let mut _move: i64 = -1;
        let mut temp_val: (i64, i64) = (0,0);
        let mut temp_state:Grid;
        let mut move_queue: VecDeque<usize> = VecDeque::new();
        let mut alpha = alpha;

        for j in 0..self.grid.rows[0].items.len() {
            let temp_state_opt = self.ai_fill_map(state, j, ai_move_val);
            if temp_state_opt.is_some() {
                temp_state = temp_state_opt.unwrap();
                temp_val = self.ai_value(&temp_state, depth, alpha, beta, ai_move_val);
                if temp_val.0 > v {
                    v = temp_val.0;
                    _move = j as i64;
                    move_queue.clear();
                    move_queue.push_back(j);
                } else if temp_val.0 == v {
                    move_queue.push_back(j);
                }

                if v > beta {
                    _move = Game::choose(move_queue) as i64;
                    return (v, _move as i64);
                }
                alpha = max(alpha, v);
            }
        }
        _move = Game::choose(move_queue) as i64;

        return (v, _move as i64);
    }

    fn choose(choice: VecDeque<usize>) -> usize{
        let mut rng = rand::thread_rng();
        let rand_idx = rng.gen_range(0, choice.len());
        return choice[rand_idx as usize];
    }

    fn ai_min_state(&self, state: &Grid, depth: u32, alpha: i64, beta: i64, ai_move_val: i64) -> (i64, i64) {
        let mut v:i64 = 100000000007;
        let mut _move: i64 = -1;
        let mut temp_val: (i64, i64) = (0,0);
        let mut temp_state:Grid;
        let mut move_queue: VecDeque<usize> = VecDeque::new();
        let mut beta = beta;

        for j in 0..self.grid.rows[0].items.len() {
            let temp_state_opt = self.ai_fill_map(state, j, ai_move_val * -1);
            if temp_state_opt.is_some() {
                temp_state = temp_state_opt.unwrap();
                temp_val = self.ai_value(&temp_state, depth, alpha, beta, ai_move_val);
                if temp_val.0 < v {
                    v = temp_val.0;
                    _move = j as i64;
                    move_queue.clear();
                    move_queue.push_back(j);
                } else if temp_val.0 == v {
                    move_queue.push_back(j);
                }

                if v < alpha {
                    _move = Game::choose(move_queue) as i64;
                    return (v, _move as i64);
                }
                beta = min(beta, v);
            }
        }
        _move = Game::choose(move_queue) as i64;

        return (v, _move as i64);
    }

    fn ai_fill_map(&self, state: &Grid, column: usize, value: i64) -> Option<Grid>{
        let mut temp_map = state.clone();
        if temp_map.rows[0].items[column] != 0 || column < 0 || column >= state.rows[0].items.len() {
            return None;
        }
        let mut done = false;
        let mut row = 0;
        for i in 0..self.grid.rows.len()-1 {
            if temp_map.rows[i+1].items[column] != 0 {
                done = true;
                row = i;
                break;
            }
        }
        if !done {
            row = 5;
        }
        temp_map.rows[row].items[column] = value;
        return Some(temp_map);
    }
}

#[derive(Clone)]
pub struct Grid {
    rows: Vec<Row>
}

impl Grid {
    pub(crate) fn new(row_size: usize, col_size: usize) -> Grid {
        let mut grid = Grid{ rows: vec![] };
        for _ in 0..row_size {
            let row = Row::new(col_size);
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
    items: Vec<i64>
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
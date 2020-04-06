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

#[derive(Clone, PartialEq)]
pub enum State {
    Done,
    Running,
    NonStarted,
}

#[derive(Clone)]
pub struct Game {
    pub grid: Grid,
    pub p1: String,
    pub p2: String,
    pub with_ai: bool,
    pub state: State,
    pub winner: String,
    pub p_move: i64
}

impl Game {
    pub fn new(row_size: usize, col_size: usize, with_ai: bool, p1_name: String, p2_name: String) -> Game {
        let grid = Grid::new(row_size, col_size);
        let mut game = Game{
            grid,
            p1: p1_name,
            p2: p2_name,
            with_ai: false,
            state: State::Running,
            winner: "".to_string(),
            p_move: 0
        };
        if with_ai {
            game.p2 = "Computer".to_string();
            game.with_ai = true;
        }
        game
    }

    pub fn start_game(&mut self) {
        self.state = State::Running;
    }

    pub fn start_game_cli<H: GameEvents>(&mut self, handler: H) {
        handler.introduction();
        let mut p1_turn = true;
        let col_size = self.grid.rows[0].items.len();
        while self.state == State::Running {
            handler.show_grid(&self.grid);
            handler.player_turn_message(p1_turn);

            if !p1_turn && self.with_ai {
                let col_num = self.ai_move(-1);
                if self.grid.insert_chip(col_num, self.p_move).is_err() {
                    continue;
                }
                handler.selected_column(self.p1.clone(), col_num);
                p1_turn = !p1_turn;
                continue;
            }

            let sel_col = handler.player_turn(col_size);
            if sel_col.is_ok() {
                let col_num = sel_col.unwrap();
                let insert_result = self.grid.insert_chip(col_num, self.p_move);
                if insert_result.is_err() {
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

    fn player_move_translate(&self) -> i64{
        if (self.p_move % 2) == 0 {
            return 1;
        }
        return -1;
    }

    pub fn make_move(&mut self, col_num: usize) -> Result<(usize, usize), ()>{
        let grid_val = self.player_move_translate();

        let insert_result = self.grid.insert_chip(col_num, grid_val);
        if insert_result.is_err() {
            return Err(());
        }

        self.p_move += 1;

        let result = self.check_win();
        if result.is_some() {
            let winner = result.unwrap();
            if winner > 0{
                self.winner = self.p1.clone();
            }
            else if winner < 0 {
                self.winner = self.p2.clone();
            }
            else if winner == 0{
                println!("error");
                self.winner = "Draw".to_string();
            }
            self.state = State::Done;
            self.post_game();
        }

        return Ok((insert_result.unwrap(), (self.p_move - 1) as usize));
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

    fn check_win(&self) -> Option<i64> {
        let mut temp_r = 0;
        let mut temp_b = 0;
        let mut temp_br = 0;
        let mut temp_tr = 0;

        for i in 0..self.grid.rows.len() {
            for j in 0..self.grid.rows[0].items.len() {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;

                for k in 0..4 {
                    if j + k < 7 {
                        temp_r += self.grid.rows[i].items[j + k];
                    }
                    //from (i,j) to bottom
                    if i + k < 6 {
                        temp_b += self.grid.rows[i + k].items[j];
                    }

                    //from (i,j) to bottom-right
                    if i + k < 6 && j + k < 7 {
                        temp_br += self.grid.rows[i + k].items[j + k];
                    }

                    //from (i,j) to top-right
                    if i as i64 - k as i64 >= 0 && j + k < 7 {
                        temp_tr += self.grid.rows[i - k].items[j + k];
                    }
                }
                if i64::abs(temp_r) == 4 {
                    return Some(temp_r);
                } else if i64::abs(temp_b) == 4 {
                    return Some(temp_b);
                } else if i64::abs(temp_br) == 4 {
                    return Some(temp_br);
                } else if i64::abs(temp_tr) == 4 {
                    return Some(temp_tr);
                }
            }
        }

        if self.p_move == 42 {
            match self.state {
                State::Done => {
                    return Some(0);
                }
                _ => {}
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
    pub rows: Vec<Row>
}

impl Grid {
    pub fn new(row_size: usize, col_size: usize) -> Grid {
        let mut grid = Grid{ rows: vec![] };
        for _ in 0..row_size {
            let row = Row::new(col_size);
            grid.rows.push(row);
        }
        grid
    }

    pub fn insert_chip(&mut self, col: usize, grid_val: i64) -> Result<(usize), ()> {
        for r in (0..self.rows.len()).rev() {
            match self.rows[r].items[col] {
                0 => {
                    self.rows[r].items[col] = grid_val;
                    return Ok((r));
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
                    -1 => write!(f, "Y"),
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
pub struct Row {
    pub items: Vec<i64>
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
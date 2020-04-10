use rand::Rng;
use std::cmp::{max, min};
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum ChipType {
    T,
    O,
}

pub trait GameEvents {
    fn introduction(&self);
    fn show_grid(&self, grid: &DummyGrid);
    fn player_turn_message(&self, p1_turn: bool);
    fn player_turn(&self, col_size: usize) -> Result<(ChipType, usize), ()>;
    fn selected_column(&self, player: String, chip_type: ChipType, col: usize);
    fn animate_chip(&self);
    fn invalid_move(&self);
    fn game_over(&self, winner: String);
}

#[derive(Clone, PartialEq)]
pub enum State {
    Done,
    Running,

    #[allow(dead_code)] // Used by web
    Busy,

    #[allow(dead_code)] // Used by web
    NonStarted,
}

#[derive(Clone)]
pub struct Game {
    pub grid: Grid,
    pub dummy_grid: DummyGrid,
    pub p1: String,
    pub p2: String,
    pub with_ai: bool,
    pub state: State,
    pub winner: String,
    pub p_move: i64,
}

impl Game {
    pub fn new(
        row_size: usize,
        col_size: usize,
        with_ai: bool,
        p1_name: String,
        p2_name: String,
    ) -> Game {
        let grid = Grid::new(row_size, col_size);
        let dummy_grid = DummyGrid::new(row_size, col_size);
        let mut game = Game {
            grid,
            dummy_grid,
            p1: p1_name,
            p2: p2_name,
            with_ai: false,
            state: State::Running,
            winner: "".to_string(),
            p_move: 0,
        };
        if with_ai {
            game.p2 = "Computer".to_string();
            game.with_ai = true;
        }
        game
    }

    #[allow(dead_code)] // Used by web
    pub fn start_game(&mut self) {
        self.state = State::Running;
    }

    pub fn start_game_cli<H: GameEvents>(&mut self, handler: H) {
        handler.introduction();
        let mut p1_turn = true;
        let col_size = self.grid.num_cols;
        while self.state == State::Running {
            handler.show_grid(&self.dummy_grid);
            handler.player_turn_message(p1_turn);
            if !p1_turn && self.with_ai {
                let (chip_type, col_num) = self.ai_move_val(-1);
                if self.grid.insert_chip(col_num, self.player_move_translate()).is_err() {
                    continue;
                }
                self.dummy_grid.insert_chip(col_num, self.player_move_dummy_translate(chip_type)).unwrap();
                self.p_move += 1;
                handler.selected_column(self.p1.clone(), chip_type, col_num);
                p1_turn = !p1_turn;
            } else {
                let sel_col = handler.player_turn(col_size);
                if sel_col.is_ok() {
                    let (chip_type, col_num) = sel_col.unwrap();
                    let grid_val = self.player_move_translate();
                    let insert_result = self.grid.insert_chip(col_num, grid_val);
                    if insert_result.is_err() {
                        handler.invalid_move();
                        continue;
                    }
                    self.dummy_grid.insert_chip(col_num, self.player_move_dummy_translate(chip_type)).unwrap();
                    self.p_move += 1;
                    if p1_turn {
                        handler.selected_column(self.p1.clone(), chip_type, col_num);
                    } else {
                        handler.selected_column(self.p2.clone(), chip_type, col_num);
                    }
                } else {
                    continue;
                }
                p1_turn = !p1_turn;
            }
            let result = self.check_win();
            if result.is_some() {
                handler.show_grid(&self.dummy_grid);
                let winner = result.unwrap();
                if winner >= 1 {
                    self.winner = self.p1.clone();
                    handler.game_over(self.winner.clone());
                } else if winner <= -1 {
                    self.winner = self.p2.clone();
                    handler.game_over(self.winner.clone());
                } else if winner == 0 {
                    self.winner = "Draw".to_string();
                    println!("Draw");
                }
                self.state = State::Done;
                self.post_game();
            }
        }
    }

    fn post_game(&self) {}

    pub fn player_move_translate(&self) -> i32 {
        if (self.p_move % 2) == 0 {
            return 1;
        }
        return -1;
    }

    pub fn player_move_dummy_translate(self, chip_type: ChipType) -> i32 {
        match chip_type {
            ChipType::T => 1,
            ChipType::O => -1,
        }
    }

    #[allow(dead_code)] // Used by web
    pub fn make_move(&mut self, chip_type: ChipType, col_num: usize) -> Result<(usize, usize), ()> {
        panic!("TODO");

        let grid_val = self.player_move_translate();

        let insert_result = self.grid.insert_chip(col_num, grid_val);
        if insert_result.is_err() {
            return Err(());
        }
        self.dummy_grid.insert_chip(col_num, self.player_move_dummy_translate(chip_type)).unwrap();

        self.p_move += 1;

        let result = self.check_win();
        if result.is_some() {
            let winner = result.unwrap();
            if winner > 0 {
                self.winner = self.p1.clone();
            } else if winner < 0 {
                self.winner = self.p2.clone();
            } else if winner == 0 {
                self.winner = "Draw".to_string();
            }
            self.state = State::Done;
            self.post_game();
        }

        return Ok((insert_result.unwrap(), (self.p_move - 1) as usize));
    }

    fn check_win(&self) -> Option<i64> {
        let mut temp_r1: [i32; 4];
        let mut temp_b1: [i32; 4];
        let mut temp_br1: [i32; 4];
        let mut temp_br2: [i32; 4];

        for i in 0..self.dummy_grid.num_rows {
            for j in 0..self.dummy_grid.num_cols {
                temp_r1[0] = 0; temp_r1[1] = 0; temp_r1[2] = 0; temp_r1[3] = 0;
                temp_b1[0] = 0; temp_b1[1] = 0; temp_b1[2] = 0; temp_b1[3] = 0;
                temp_br1[0] = 0; temp_br1[1] = 0; temp_br1[2] = 0; temp_br1[3] = 0;
                temp_br2[0] = 0; temp_br2[1] = 0; temp_br2[2] = 0; temp_br2[3] = 0;

                for k in 0..4 {
                    // From (i,j) to right
                    if j + k < 7 {
                        temp_r1[k] = self.dummy_grid.get(i, j + k);
                    }
                    // From (i,j) to bottom
                    if i + k < 6 {
                        temp_b += self.grid.get(i + k, j) as i64;
                    }

                    // From (i,j) to bottom-right
                    if i + k < 6 && j + k < 7 {
                        temp_br += self.grid.get(i + k, j + k) as i64;
                    }

                    // From (i,j) to top-right
                    if i as i64 - k as i64 >= 0 && j + k < 7 {
                        temp_tr += self.grid.get(i - k, j + k) as i64;
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

        // Draw
        if self.p_move == (self.grid.num_rows * self.grid.num_cols) as i64 {
            match self.state {
                State::Done => {}
                _ => {
                    return Some(0);
                }
            }
        }

        return None;
    }

    #[allow(dead_code)] // Used by web
    pub fn ai_make_move(&mut self) -> Result<(usize, usize, usize), ()> {
        let mut col_num = self.ai_move_val(-1);
        let grid_val = self.player_move_translate();

        let mut insert_result = self.grid.insert_chip(col_num, grid_val);

        // Fall back to random agent
        while insert_result.is_err() {
            let mut rng = rand::thread_rng();
            col_num = rng.gen_range(0, self.grid.num_cols);
            insert_result = self.grid.insert_chip(col_num, grid_val);
        }

        self.p_move += 1;

        let result = self.check_win();
        if result.is_some() {
            let winner = result.unwrap();
            if winner > 0 {
                self.winner = self.p1.clone();
            } else if winner < 0 {
                self.winner = self.p2.clone();
            } else if winner == 0 {
                self.winner = "Draw".to_string();
            }
            self.state = State::Done;
            self.post_game();
        }

        return Ok((insert_result.unwrap(), (self.p_move - 1) as usize, col_num));
    }

    fn ai_move_val(&self, ai_move_val: i64) -> (ChipType, usize) {
        let state = &self.grid.clone();
        let choice_val = self.ai_max_state(&state, 0, -100000000007, 100000000007, ai_move_val);
        let choice = choice_val.1;
        return choice as usize;
    }

    fn ai_check_state(state: &Grid) -> (i64, i64) {
        let mut win_val: i64 = 0;
        let mut chain_val: i64 = 0;
        let mut temp_r: i64;
        let mut temp_b: i64;
        let mut temp_br: i64;
        let mut temp_tr: i64;

        let num_rows = state.num_rows;
        let num_cols = state.num_cols;

        for i in 0..num_rows {
            for j in 0..num_cols {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;

                for k in 0..4 {
                    if j + k < num_cols {
                        temp_r += state.get(i, j + k) as i64;
                    }
                    if i + k < num_rows {
                        temp_b += state.get(i + k, j) as i64;
                    }
                    if i + k < num_rows && j + k < num_cols {
                        temp_br += state.get(i + k, j + k) as i64;
                    }
                    if i as i64 - k as i64 >= 0 && j + k < num_cols {
                        temp_tr += state.get(i - k, j + k) as i64;
                    }
                }

                chain_val += temp_r * temp_r * temp_r;
                chain_val += temp_b * temp_b * temp_b;
                chain_val += temp_br * temp_br * temp_br;
                chain_val += temp_tr * temp_tr * temp_tr;

                if i64::abs(temp_r) == 4 {
                    win_val = temp_r;
                } else if i64::abs(temp_b) == 4 {
                    win_val = temp_b;
                } else if i64::abs(temp_br) == 4 {
                    win_val = temp_br;
                } else if i64::abs(temp_tr) == 4 {
                    win_val = temp_tr;
                }
            }
        }

        return (win_val, chain_val);
    }

    fn ai_value(
        &self,
        state: &Grid,
        depth: u32,
        alpha: i64,
        beta: i64,
        ai_move_val: i64,
    ) -> (i64, i64) {
        let val = Game::ai_check_state(&state);
        if depth >= 4 {
            let mut ret_value;
            let win_val = val.0;
            let chain_val = val.1 * ai_move_val;
            ret_value = chain_val;

            if win_val == 4 * ai_move_val {
                ret_value = 999999;
            } else if win_val == 4 * ai_move_val * -1 {
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

    fn ai_max_state(
        &self,
        state: &Grid,
        depth: u32,
        alpha: i64,
        beta: i64,
        ai_move_val: i64,
    ) -> (i64, i64) {
        let mut v: i64 = -100000000007;
        let mut _move: i64 = -1;
        let mut temp_val: (i64, i64);
        let mut temp_state: Grid;
        let mut move_queue: Vec<usize> = Vec::new();
        let mut alpha = alpha;

        for j in 0..self.grid.num_cols {
            let temp_state_opt = self.ai_fill_map(state, j, ai_move_val);
            if temp_state_opt.is_some() {
                temp_state = temp_state_opt.unwrap();
                temp_val = self.ai_value(&temp_state, depth, alpha, beta, ai_move_val);

                if temp_val.0 > v {
                    v = temp_val.0;
                    _move = j as i64;
                    move_queue.clear();
                    move_queue.push(j);
                } else if temp_val.0 == v {
                    move_queue.push(j);
                }

                if v > beta {
                    _move = Game::choose(move_queue) as i64;
                    return (v, _move as i64);
                }
                alpha = max(alpha, v);
            }
        }

        if move_queue.len() == 0 {
            (v, -1)
        } else {
            _move = Game::choose(move_queue) as i64;
            (v, _move as i64)
        }
    }

    fn choose(choice: Vec<usize>) -> usize {
        let mut rng = rand::thread_rng();
        let rand_idx = rng.gen_range(0, choice.len());
        return choice[rand_idx as usize];
    }

    fn ai_min_state(
        &self,
        state: &Grid,
        depth: u32,
        alpha: i64,
        beta: i64,
        ai_move_val: i64,
    ) -> (i64, i64) {
        let mut v: i64 = 100000000007;
        let mut _move: i64 = -1;
        let mut temp_val: (i64, i64);
        let mut temp_state: Grid;
        let mut move_queue: Vec<usize> = Vec::new();
        let mut beta = beta;

        for j in 0..self.grid.num_cols {
            let temp_state_opt = self.ai_fill_map(state, j, ai_move_val * -1);
            if temp_state_opt.is_some() {
                temp_state = temp_state_opt.unwrap();
                temp_val = self.ai_value(&temp_state, depth, alpha, beta, ai_move_val);

                if temp_val.0 < v {
                    v = temp_val.0;
                    _move = j as i64;
                    move_queue.clear();
                    move_queue.push(j);
                } else if temp_val.0 == v {
                    move_queue.push(j);
                }

                if v < alpha {
                    _move = Game::choose(move_queue) as i64;
                    return (v, _move as i64);
                }
                beta = min(beta, v);
            }
        }

        if move_queue.len() == 0 {
            (v, -1)
        } else {
            _move = Game::choose(move_queue) as i64;
            return (v, _move as i64);
        }
    }

    fn ai_fill_map(&self, state: &Grid, column: usize, value: i64) -> Option<Grid> {
        let mut temp_map = state.clone();
        if temp_map.get(0, column) != 0 || /* column < 0 || */ column >= self.grid.num_cols {
            return None;
        }
        let mut done = false;
        let mut row = 0;
        for i in 0..self.grid.num_rows - 1 {
            if temp_map.get(i + 1, column) != 0 {
                done = true;
                row = i;
                break;
            }
        }
        if !done {
            row = self.grid.num_rows - 1;
        }
        temp_map.set(row, column, value as i32);
        return Some(temp_map);
    }
}

#[derive(Clone)]
pub struct Grid {
    pub items: [i32; 80],
    pub num_rows: usize,
    pub num_cols: usize,
}

impl Grid {
    pub fn new(row_size: usize, col_size: usize) -> Self {
        let mut grid = Grid {
            items: [0; 80],
            num_rows: row_size,
            num_cols: col_size,
        };
        for x in 0..(row_size * col_size) {
            grid.items[x] = 0;
        }
        grid
    }

    pub fn insert_chip(&mut self, col: usize, grid_val: i32) -> Result<usize, ()> {
        for r in (0..self.num_rows).rev() {
            match self.get(r, col) {
                0 => {
                    self.set(r, col, grid_val as i32);
                    return Ok(r);
                }
                _ => {}
            }
        }
        return Err(());
    }
    pub fn get(&self, row: usize, col: usize) -> i32 {
        self.items[col * self.num_rows + (self.num_rows - 1 - row)]
    }
    pub fn set(&mut self, row: usize, col: usize, val: i32) {
        self.items[col * self.num_rows + (self.num_rows - 1 - row)] = val;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.num_rows {
            for c in 0..self.num_cols {
                let chip = self.get(r, c);
                match chip {
                    0 => write!(f, "_"),
                    1 => write!(f, "R"),
                    -1 => write!(f, "Y"),
                    _ => Err(std::fmt::Error),
                }?;
                write!(f, " ")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct DummyGrid {
    pub items: [i32; 80],
    pub num_rows: usize,
    pub num_cols: usize,
}

impl DummyGrid {
    pub fn new(row_size: usize, col_size: usize) -> Self {
        let mut grid = DummyGrid {
            items: [0; 80],
            num_rows: row_size,
            num_cols: col_size,
        };
        for x in 0..(row_size * col_size) {
            grid.items[x] = 0;
        }
        grid
    }

    pub fn insert_chip(&mut self, col: usize, grid_val: i32) -> Result<usize, ()> {
        for r in (0..self.num_rows).rev() {
            match self.get(r, col) {
                0 => {
                    self.set(r, col, grid_val as i32);
                    return Ok(r);
                }
                _ => {}
            }
        }
        return Err(());
    }

    pub fn get(&self, row: usize, col: usize) -> i32 {
        self.items[col * self.num_rows + (self.num_rows - 1 - row)]
    }

    pub fn set(&mut self, row: usize, col: usize, val: i32) {
        self.items[col * self.num_rows + (self.num_rows - 1 - row)] = val;
    }
}

impl fmt::Display for DummyGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.num_rows {
            for c in 0..self.num_cols {
                let chip = self.get(r, c);
                match chip {
                    0 => write!(f, "_"),
                    1 => write!(f, "T"),
                    -1 => write!(f, "O"),
                    _ => Err(std::fmt::Error),
                }?;
                write!(f, " ")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

// TODO: Is this still used?
#[derive(Clone)]
pub struct Row {
    pub items: Vec<i64>,
}

impl Row {
    fn new(size: usize) -> Row {
        let mut row = Row { items: vec![] };
        row.items = Vec::new();
        for _ in 0..size {
            row.items.push(0);
        }
        row
    }
}

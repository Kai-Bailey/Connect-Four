mod connect_four;
mod toot_otto;

use crate::connect_four::{
    Game as ConnectFourGame, GameEvents as ConnectFourGameEvents, Grid as ConnectFourGrid,
};
use crate::toot_otto::{
    ChipType as TootOttoChipType, DummyGrid as TootOttoGrid, Game as TootOttoGame,
    GameEvents as TootOttoGameEvents,
};
use std::io;

fn main() {
    println!("Welcome to our game Command Line Interface.");
    println!("Please choose between the following: ");
    println!("1) Connect-4");
    println!("2) Toot and Otto");

    let sel = retrieve_user_input();
    let is_connect_four;

    match sel {
        Ok(x) => match x.as_str() {
            "1" => is_connect_four = true,
            "2" => is_connect_four = false,
            _ => {
                println!("Invalid input");
                return;
            }
        },
        Err(_) => {
            println!("Invalid input");
            return;
        }
    }

    if is_connect_four {
        start_connect_four();
    } else {
        start_toot_and_otto();
    }
}

fn retrieve_user_input() -> Result<String, ()> {
    let mut command = String::new();
    let command_vector;
    print!("> ");
    io::Write::flush(&mut io::stdout()).expect("Error while Flushing Buffer");
    match io::stdin().read_line(&mut command) {
        Ok(_) => {
            command_vector = command.split_whitespace().collect::<Vec<&str>>();
        }
        Err(_) => {
            println!("Error reading input!");
            return Err(());
        }
    }
    if command_vector.len() == 0 {
        println!("Error: No selection made.");
        return Err(());
    } else {
        return Ok(command_vector[0].to_string());
    }
}

struct ConnectFourCliInterface {}

impl ConnectFourGameEvents for ConnectFourCliInterface {
    fn introduction(&self) {}

    fn show_grid(&self, grid: &ConnectFourGrid) {
        for i in 0..grid.num_cols {
            print!("{} ", i);
        }
        println!("");

        println!("{}", grid);
    }

    fn player_turn_message(&self, p1_turn: bool) {
        if p1_turn {
            println!("Player 1's turn");
        } else {
            println!("Player 2's turn");
        }
    }

    fn player_turn(&self, col_size: usize) -> Result<usize, ()> {
        println!("Please select a column (0-{})", col_size - 1);
        let col = retrieve_user_input();
        if col.is_ok() {
            return Ok(col.unwrap().parse().unwrap());
        }
        return Err(());
    }

    fn selected_column(&self, player: String, col: usize) {
        println!("{} Selected Column {}", player, col)
    }

    fn animate_chip(&self) {}

    fn invalid_move(&self) {
        println!("Column is full. Please try again with different column");
    }

    fn game_over(&self, winner: String) {
        println!("{} has won! Congratulations!", winner);
    }
}

struct TootOttoCliInterface {}

impl TootOttoGameEvents for TootOttoCliInterface {
    fn introduction(&self) {}

    fn show_grid(&self, grid: &TootOttoGrid) {
        for i in 0..grid.num_cols {
            print!("{} ", i);
        }
        println!("");

        println!("{}", grid);
    }

    fn player_turn_message(&self, p1_turn: bool) {
        if p1_turn {
            println!("Player 1's turn");
        } else {
            println!("Player 2's turn");
        }
    }

    fn player_turn(&self, col_size: usize) -> Result<(TootOttoChipType, usize), ()> {
        let chip_type;

        println!("Please select a chip type (T or O)");
        let chip = retrieve_user_input();
        if chip.is_ok() {
            let chip_str = chip.unwrap();
            if chip_str == "T" {
                chip_type = TootOttoChipType::T;
            } else if chip_str == "O" {
                chip_type = TootOttoChipType::O;
            } else {
                println!("Invalid input");
                return Err(());
            }
        } else {
            return Err(());
        }

        println!("Please select a column (0-{})", col_size - 1);
        let col = retrieve_user_input();
        if col.is_ok() {
            return Ok((chip_type, col.unwrap().parse().unwrap()));
        }
        return Err(());
    }

    fn selected_column(&self, player: String, chip_type: TootOttoChipType, col: usize) {
        let chip_str;
        match chip_type {
            TootOttoChipType::T => {
                chip_str = "T";
            }
            TootOttoChipType::O => {
                chip_str = "O";
            }
        };
        println!(
            "{} Selected Chip Type {} and Column {}",
            player, chip_str, col
        );
    }

    fn animate_chip(&self) {}

    fn invalid_move(&self) {
        println!("Column is full. Please try again with different column");
    }

    fn game_over(&self, winner: String) {
        println!("{} has won! Congratulations!", winner);
    }
}

fn start_connect_four() {
    println!("Select a game board size for Connect-4: ");
    // Size variations include 5×4, 6×5, 8×7, 9×7, 10×7, 8×8,
    println!("1) 6 x 7");
    println!("2) 5 x 4");
    println!("3) 6 x 5");
    println!("4) 8 x 7");
    println!("5) 9 x 7");
    println!("6) 10 x 7");
    println!("7) 8 x 8");

    let sel = retrieve_user_input();
    let num_rows;
    let num_cols;
    match sel {
        Ok(x) => match x.as_str() {
            "1" => {
                num_rows = 6;
                num_cols = 7;
            }
            "2" => {
                num_rows = 5;
                num_cols = 4;
            }
            "3" => {
                num_rows = 6;
                num_cols = 5;
            }
            "4" => {
                num_rows = 8;
                num_cols = 7;
            }
            "5" => {
                num_rows = 9;
                num_cols = 7;
            }
            "6" => {
                num_rows = 10;
                num_cols = 7;
            }
            "7" => {
                num_rows = 8;
                num_cols = 8;
            }

            _ => {
                println!("Invalid input");
                return;
            }
        },
        Err(_) => {
            println!("Invalid input");
            return;
        }
    }

    println!("Do you want to play against an AI or another human?");
    println!("1) Human");
    println!("2) AI");

    let sel = retrieve_user_input();
    let mut game;

    match sel {
        Ok(x) => match x.as_str() {
            "1" => {
                game = ConnectFourGame::new(
                    num_rows,
                    num_cols,
                    false,
                    "P1".to_string(),
                    "P2".to_string(),
                );
            }
            "2" => {
                game = ConnectFourGame::new(
                    num_rows,
                    num_cols,
                    true,
                    "Player".to_string(),
                    "Computer".to_string(),
                );
            }
            _ => {
                println!("Invalid input");
                return;
            }
        },
        Err(_) => {
            println!("Invalid input");
            return;
        }
    }

    let handler: ConnectFourCliInterface = ConnectFourCliInterface {};
    game.start_game_cli(handler);
}

fn start_toot_and_otto() {
    println!("Select a game board size for Toot-Otto: ");
    // Size variations include 5×4, 6×5, 8×7, 9×7, 10×7, 8×8,
    println!("1) 6 x 7");
    println!("2) 5 x 4");
    println!("3) 6 x 5");
    println!("4) 8 x 7");
    println!("5) 9 x 7");
    println!("6) 10 x 7");
    println!("7) 8 x 8");

    let sel = retrieve_user_input();
    let num_rows;
    let num_cols;
    match sel {
        Ok(x) => match x.as_str() {
            "1" => {
                num_rows = 6;
                num_cols = 7;
            }
            "2" => {
                num_rows = 5;
                num_cols = 4;
            }
            "3" => {
                num_rows = 6;
                num_cols = 5;
            }
            "4" => {
                num_rows = 8;
                num_cols = 7;
            }
            "5" => {
                num_rows = 9;
                num_cols = 7;
            }
            "6" => {
                num_rows = 10;
                num_cols = 7;
            }
            "7" => {
                num_rows = 8;
                num_cols = 8;
            }

            _ => {
                println!("Invalid input");
                return;
            }
        },
        Err(_) => {
            println!("Invalid input");
            return;
        }
    }

    println!("Do you want to play against an AI or another human?");
    println!("1) Human");
    println!("2) AI");

    let sel = retrieve_user_input();
    let mut game;

    match sel {
        Ok(x) => match x.as_str() {
            "1" => {
                game = TootOttoGame::new(
                    num_rows,
                    num_cols,
                    false,
                    "P1".to_string(),
                    "P2".to_string(),
                );
            }
            "2" => {
                game = TootOttoGame::new(
                    num_rows,
                    num_cols,
                    true,
                    "Player".to_string(),
                    "Computer".to_string(),
                );
            }
            _ => {
                println!("Invalid input");
                return;
            }
        },
        Err(_) => {
            println!("Invalid input");
            return;
        }
    }

    let handler: TootOttoCliInterface = TootOttoCliInterface {};
    game.start_game_cli(handler);
}

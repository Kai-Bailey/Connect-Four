mod connect_four;

use std::io;
use connect_four::Grid;
use crate::connect_four::{GameEvents, Game};
use std::io::Write;

fn main() {
    println!("Welcome to our game Command Line Interface.");
    println!("Please choose between the following: ");
    println!("1) Connect-4");
    println!("2) Toot and Otto");

    let sel = retrieve_user_input();
    let mut is_connect_four = true;
    match sel {
        Ok(x) => {
            match x.as_str() {
                "1" => { is_connect_four = true },
                "2" => { is_connect_four = false },
                _ => {
                    println!("Invalid input");
                    return;
                }
            }
        }
        Err(_) => {
            println!("Invalid input");
            return;
        }
    }

    if is_connect_four {
        start_connect_four();
    }
    else {
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
            return Err(())
        }
    }
    if command_vector.len() == 0 {
        println!("Error: No selection made.");
        return Err(())
    }else{
        return Ok(command_vector[0].to_string())
    }
}

struct CliInterface {

}

impl GameEvents for CliInterface {
    fn introduction(&self) {

    }
    fn show_grid(&self, grid: &Grid) {
        for i in 0..grid.num_cols {
            print!("{} ", i);
        }
        println!("");

        println!("{}", grid);
    }

    fn player_turn_message(&self, p1_turn: bool) {
        if p1_turn {
            println!("Player 1's turn");
        }
        else{
            println!("Player 2's turn");
        }
    }

    fn player_turn(&self, col_size: usize) -> Result<usize, ()> {
        println!("Please select a column (0-{})", col_size-1);
        let col = retrieve_user_input();
        if col.is_ok() {
            return Ok(col.unwrap().parse().unwrap());
        }
        return Err(());
    }

    fn selected_column(&self, player: String, col: usize) {
        println!("{} Selected Column {}", player, col)
    }

    fn animate_chip(&self) {

    }
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
    let mut num_rows = 6;
    let mut num_cols = 7;
    match sel {
        Ok(x) => {
            match x.as_str() {
                "1" => { num_rows = 6; num_cols = 7; },
                "2" => { num_rows = 5; num_cols = 4; },
                "3" => { num_rows = 6; num_cols = 5; },
                "4" => { num_rows = 8; num_cols = 7; },
                "5" => { num_rows = 9; num_cols = 7; },
                "6" => { num_rows = 10; num_cols = 7; },
                "7" => { num_rows = 8; num_cols = 8; },

                _ => {
                    println!("Invalid input");
                    return;
                }
            }
        }
        Err(_) => {
            println!("Invalid input");
            return;
        }
    }

    println!("Do you want to play against an AI or another human?");
    println!("1) Human");
    println!("2) AI");
    let sel = retrieve_user_input();
    let mut is_connect_four = true;
    let mut game;
    match sel {
        Ok(x) => {
            match x.as_str() {
                "1" => {
                    game = Game::new(num_rows, num_cols, false, "Bob".to_string(), "Jim".to_string());
                },
                "2" => {
                    game = Game::new(num_rows, num_cols, true, "Bob".to_string(), "Jim".to_string());
                },
                _ => {
                    println!("Invalid input");
                    return;
                }
            }
        }
        Err(_) => {
            println!("Invalid input");
            return;
        }
    }

    let handler: CliInterface = CliInterface{};
    game.start_game_cli(handler);
}

fn start_toot_and_otto() {

}
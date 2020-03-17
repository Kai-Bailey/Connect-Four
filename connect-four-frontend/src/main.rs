#![recursion_limit="999999"]

#[macro_use] extern crate stdweb;

mod connect_4_app;
mod connect_4_human;
mod score_board_ctrl;

fn main() {
    connect_4_app::main();
    connect_4_human::main();
    score_board_ctrl::main();
}

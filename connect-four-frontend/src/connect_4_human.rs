use serde_json::json;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct Connect4HumanModel {
    link: ComponentLink<Self>,
    gameStarted: bool,
    player1Name: String,
    player2Name: String,
    game: Rc<RefCell<Game>>,
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
}

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d, FillRule};

use stdweb::web::event::{ClickEvent, ResizeEvent};

use connect_four_cli::connect_four::{Game, Grid, State};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use stdweb::web::html_element::{CanvasElement, SelectElement};

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub enum Msg {
    gotPlayer1Name(String),
    gotPlayer2Name(String),
    startGame,
    clicked(Option<usize>),
    PostGameSuccess,
    PostGameFailed,
}

fn draw_board(game: Rc<RefCell<Game>>) {
    let canvas: CanvasElement = document()
        .query_selector("#gameboard_c4_human")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    context.save();
    context.set_fill_style_color("#00bfff");
    context.begin_path();
    for y in 0..game.borrow().grid.num_rows {
        for x in 0..game.borrow().grid.num_cols {
            context.arc(
                (75 * x + 100) as f64,
                (75 * y + 50) as f64,
                25.0,
                0.0,
                2.0 * PI,
                false,
            );
            context.rect((75 * x + 150) as f64, (75 * y) as f64, -100.0, 100.0);
        }
    }
    context.fill(FillRule::NonZero);
    context.restore();
}

fn draw(grid: &Grid, num_rows: usize, num_cols: usize) {
    for y in 0..num_rows {
        for x in 0..num_cols {
            let mut fg_color = "transparent";
            if grid.get(y, x) >= 1 {
                fg_color = "#ff4136";
            } else if grid.get(y, x) <= -1 {
                fg_color = "#ffff00";
            }
            draw_circle(
                75.0 * x as f64 + 100.0,
                75.0 * y as f64 + 50.0,
                25.0,
                fg_color.to_string(),
                "black".to_string(),
            );
        }
    }
}

fn draw_circle(x: f64, y: f64, r: f64, fill: String, stroke: String) {
    let canvas: CanvasElement = document()
        .query_selector("#gameboard_c4_human")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    context.save();
    context.set_fill_style_color(fill.as_str());
    context.set_stroke_style_color(stroke.as_str());
    context.begin_path();
    context.arc(x, y, r, 0.0, 2.0 * PI, false);
    context.fill(FillRule::NonZero);
    context.restore();
}

fn print_win(winner: String) {
    let canvas: CanvasElement = document()
        .query_selector("#gameboard_c4_human")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    let mut msg = "".to_string();
    if winner == "Draw" {
        msg.push_str("It's a draw");
    } else {
        msg.push_str(winner.as_str());
        msg.push_str(" wins");
    }
    msg.push_str(" - Click on board to save and reset");
    context.save();
    context.set_font("14pt sans-serif");
    context.set_fill_style_color("#111");
    context.fill_text(msg.as_str(), 130.0, 20.0, None);
    context.restore();
}

fn animate(
    column: i64,
    move_val: i64,
    to_row: i64,
    cur_pos: i64,
    grid: Grid,
    game: Rc<RefCell<Game>>,
) {
    let cur_pos = cur_pos;
    let mut fg_color = "transparent";
    if move_val % 2 == 0 {
        fg_color = "#ff4136";
    } else if move_val % 2 == 1 {
        fg_color = "#ffff00";
    }

    if to_row * 75 >= cur_pos {
        clear_canvas();
        draw(
            &grid.clone(),
            game.borrow().grid.num_rows,
            game.borrow().grid.num_cols,
        );
        draw_circle(
            (75 * column + 100) as f64,
            (cur_pos + 50) as f64,
            25.0,
            fg_color.to_string(),
            "black".to_string(),
        );
        draw_board(game.clone());
        window().request_animation_frame(move |_| {
            animate(
                column,
                move_val,
                to_row,
                cur_pos + 25.0 as i64,
                grid.clone(),
                game,
            )
        });
    } else {
        check_for_win(game.clone());
    }
}

fn check_for_win(game: Rc<RefCell<Game>>) {
    // check if game ended after move
    let state = game.clone().borrow_mut().state.clone();
    match state {
        State::Done => {
            // draw finished
            print_win(game.clone().borrow_mut().winner.clone());
        }
        _ => {}
    }
}

fn clear_canvas() {
    let canvas: CanvasElement = document()
        .query_selector("#gameboard_c4_human")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
}

impl Connect4HumanModel {
    fn is_started(&self) -> bool {
        let state = self.game.clone().borrow().state.clone();
        return match state {
            State::NonStarted => false,
            _ => true,
        };
    }
}

impl Component for Connect4HumanModel {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let game = Rc::new(RefCell::new(Game {
            grid: Grid::new(6, 7),
            p1: "".to_string(),
            p2: "".to_string(),
            with_ai: false,
            state: State::NonStarted,
            winner: "".to_string(),
            p_move: 0,
            max_ai_depth: 4,
        }));
        Connect4HumanModel {
            link,
            gameStarted: false,
            player1Name: "".to_string(),
            player2Name: "".to_string(),
            game: game.clone(),
            fetch_service: FetchService::new(),
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::gotPlayer1Name(newName) => {
                self.player1Name = newName;
            }
            Msg::gotPlayer2Name(newName) => {
                self.player2Name = newName;
            }
            Msg::startGame => {
                self.gameStarted = true;

                // board size
                let sel_box: SelectElement = document()
                    .query_selector("#board_size_dropdown")
                    .unwrap()
                    .unwrap()
                    .try_into()
                    .unwrap();

                let boardSize = match sel_box.value().unwrap().as_str() {
                    "6_7" => (6, 7),
                    "5_4" => (5, 4),
                    "6_5" => (6, 5),
                    "8_7" => (8, 7),
                    "9_7" => (9, 7),
                    "10_7" => (10, 7),
                    _ => (6, 7),
                };

                self.game.replace(Game::new(
                    boardSize.0,
                    boardSize.1,
                    false,
                    self.player1Name.clone(),
                    self.player2Name.clone(),
                    4,
                ));
                draw_board(self.game.clone());
                self.game.borrow_mut().start_game();
            }
            Msg::clicked(col) => {
                let state = self.game.borrow().state.clone();
                match state {
                    State::Done => {
                        clear_canvas();
                        self.game.clone().borrow_mut().state = State::NonStarted;
                        self.post_win();
                    }
                    State::Running => {
                        if col.is_some()
                            /* && col.unwrap() >= 0 */
                            && col.unwrap() < self.game.borrow().grid.num_cols
                        {
                            let prev_grid = self.game.borrow().grid.clone();
                            let insert_result =
                                self.game.borrow_mut().make_move(col.unwrap() as usize);
                            if insert_result.is_ok() {
                                animate(
                                    col.unwrap() as i64,
                                    insert_result.unwrap().1 as i64,
                                    insert_result.unwrap().0 as i64,
                                    0,
                                    prev_grid,
                                    self.game.clone(),
                                );
                            }
                        }
                    }
                    _ => {}
                }
                check_for_win(self.game.clone());
            }
            Msg::PostGameSuccess => {
                js! {alert("Game was successfully saved.")}
            }
            Msg::PostGameFailed => {
                js! {alert("Failed to save game...")}
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn mounted(&mut self) -> ShouldRender {
        let canvas: CanvasElement = document()
            .query_selector("#gameboard_c4_human")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);

        window().add_event_listener(enclose!((canvas) move |_: ResizeEvent| {
            canvas.set_width(canvas.offset_width() as u32);
            canvas.set_height(canvas.offset_height() as u32);
        }));

        let rect = canvas.get_bounding_client_rect();

        let game_clone = self.game.clone();
        let link = self.link.clone();

        #[allow(unused_variables)]
        canvas.add_event_listener(enclose!((context) move |event: ClickEvent| {
            let x_click = event.client_x() - rect.get_left() as i32;
            let num_cols = game_clone.clone().borrow().grid.num_cols;
            for col in 0..num_cols {
                let x_col = 75 * col as i32 + 100;
                if (x_click - x_col) * (x_click - x_col) <= 25 * 25 {
                    link.send_message(Msg::clicked(Some(col as usize)));
                    return;
                }
            }
            link.send_message(Msg::clicked(None));
        }));

        false
    }

    fn view(&self) -> VNode {
        let title;
        if self.is_started() {
            title = "Human VS Human Connect 4";
        } else {
            title = "Enter Player Names";
        }

        html! {
            <div id="main" ng-controller="humanController">
                <div class="w3-container" id="services" style="margin-top:75px">
                    <h5 class="w3-xxxlarge w3-text-red"><b>{title}</b></h5>
                    <hr style="width:50px;border:5px solid red" class="w3-round"/>
                </div>
                <div class="col-md-offset-4 col-md-8">
                    {
                        if self.is_started() {
                            html! {
                                <div>
                                    <h4>{"New Game: "}{&self.player1Name}{" VS "}{&self.player2Name}</h4>
                                    <small>{"Disc Colors: "}{&self.player1Name}{" - Red and "}{&self.player2Name}{" - Yellow"}</small>
                                </div>
                            }
                        } else {
                            html!{
                                <div class="col-md-offset-3 col-md-8">
                                    <input id="textbox1" style="margin: 5px" type="text" placeholder="Player 1's Name" oninput=self.link.callback(|e: InputData| Msg::gotPlayer1Name(e.value)) />
                                    <input id="textbox2" style="margin: 5px" type="text" placeholder="Player 2's Name" oninput=self.link.callback(|e: InputData| Msg::gotPlayer2Name(e.value)) />
                                    <select id="board_size_dropdown" style="margin: 5px">
                                        <option selected=true disabled=false value="6_7">{"6 x 7"}</option>
                                        <option selected=false disabled=false value="5_4">{"5 x 4"}</option>
                                        <option selected=false disabled=false value="6_5">{"6 x 5"}</option>
                                        <option selected=false disabled=false value="8_7">{"8 x 7"}</option>
                                        <option selected=false disabled=false value="9_7">{"9 x 7"}</option>
                                        <option selected=false disabled=false value="10_7">{"10 x 7"}</option>
                                    </select>
                                    <button style="margin: 5px" onclick=self.link.callback(|_| Msg::startGame)>{ "Start Game" }</button>
                                </div>
                            }
                        }
                    }
                    <canvas id="gameboard_c4_human" height="760" width="640"></canvas>
                </div>
            </div>
        }
    }
}

impl Connect4HumanModel {
    fn post_win(&mut self) {
        let json_sg = json!({
            "gameType": "Connect4 with Human",
            "Player1Name": self.player1Name,
            "Player2Name": self.player2Name,
            "WinnerName": self.game.clone().borrow_mut().winner.clone(),
        });
        let callback = self.link.callback(|response: Response<Result<String, _>>| {
            if response.status().is_success() {
                Msg::PostGameSuccess
            } else {
                Msg::PostGameFailed
            }
        });
        let post_request = Request::post("http://localhost:8000/games")
            .header("Content-Type", "application/json")
            .body(Json(&json_sg))
            .unwrap();
        let task = self.fetch_service.fetch(post_request, callback);
        self.fetch_task = Some(task.unwrap());
    }
}

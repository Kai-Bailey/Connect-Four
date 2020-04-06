use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct Connect4HumanModel {
    link: ComponentLink<Self>,
    gameStarted: bool,
    player1Name: String,
    player2Name: String,
    game: Rc<RefCell<Game>>,
}

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d, FillRule};

use stdweb::web::event::{ClickEvent, ResizeEvent};

use stdweb::web::html_element::CanvasElement;
use connect_four_cli::connect_four::{Game, Grid, State};
use std::f64::consts::PI;
use std::cell::RefCell;
use std::rc::Rc;

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
}

fn draw_board() {
    js! {// draw the mask
        // http://stackoverflow.com/questions/6271419/how-to-fill-the-opposite-shape-on-canvas
        // -->  http://stackoverflow.com/a/11770000/917957
        var canvas = document.getElementsByTagName("canvas")[0];
        var context = canvas.getContext("2d");
        context.save();
        context.fillStyle = "#00bfff";
        context.beginPath();
        var x, y;
        for (y = 0; y < 6; y++) {
            for (x = 0; x < 7; x++) {
                context.arc(75 * x + 100, 75 * y + 50, 25, 0, 2 * Math.PI);
                context.rect(75 * x + 150, 75 * y, -100, 100);
            }
        }
        context.fill();
        context.restore();
    }
}

fn draw(grid: &Grid) {
    for y in 0..6 {
        for x in 0..7 {
            let mut fg_color = "transparent";
            if grid.rows[y].items[x] >= 1 {
                fg_color = "#ff4136";
            }
            else if grid.rows[y].items[x] <= -1 {
                fg_color = "#ffff00";
            }
            draw_circle(75.0 * x as f64 + 100.0, 75.0 * y as f64 + 50.0, 25.0,
                        fg_color.to_string(), "black".to_string());
        }
    }
}

fn draw_circle(x: f64, y: f64, r: f64, fill: String, stroke: String) {
    let canvas: CanvasElement = document()
        .query_selector("#gameboard")
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
        .query_selector("#gameboard")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    let mut msg = "".to_string();
    if winner == "Draw" {
        msg.push_str("It's a draw");
    }
    else{
        msg.push_str(winner.as_str());
        msg.push_str(" wins");
    }
    msg.push_str(" - Click on game board to reset");
    context.save();
    context.set_font("14pt sans-serif");
    context.set_fill_style_color("#111");
    context.fill_text(msg.as_str(), 130.0, 20.0, None);
    context.restore();
}

fn animate(column: i64, move_val: i64, to_row: i64, cur_pos: i64, grid: Grid, game: Rc<RefCell<Game>>) {
    let mut cur_pos = cur_pos;
    let mut fg_color = "transparent";
    if move_val % 2 == 0 {
        fg_color = "#ff4136";
    }
    else if move_val % 2 == 1 {
        fg_color = "#ffff00";
    }

    if to_row * 75 >= cur_pos {
        clear_canvas();
        draw(&grid.clone());
        draw_circle((75 * column + 100) as f64, (cur_pos + 50) as f64, 25.0,
                    fg_color.to_string(), "black".to_string());
        draw_board();
        window().request_animation_frame(move |_| {
            animate(column, move_val, to_row, cur_pos + 25.0 as i64, grid.clone(), game)
        });
    }
    else{
        check_for_win(game.clone());
    }
}

fn check_for_win(game:Rc<RefCell<Game>>){
    // check if game ended after move
    let state = game.clone().borrow_mut().state.clone();
    match state {
        State::Done => {
            // draw finished
            print_win(game.clone().borrow_mut().winner.clone());
        },
        State::Running => {

        },
        State::NonStarted => {

        }
    }
}

fn clear_canvas() {
    let canvas: CanvasElement = document()
        .query_selector("#gameboard")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
}

impl Connect4HumanModel {
    fn is_started(&self) -> bool{
        let state = self.game.clone().borrow().state.clone();
        return match state {
            State::NonStarted => false,
            _ => true
        }
    }
}

impl Component for Connect4HumanModel {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let game = Rc::new(RefCell::new(Game{
            grid: Grid::new(6, 7),
            p1: "".to_string(),
            p2: "".to_string(),
            with_ai: false,
            state: State::NonStarted,
            winner: "".to_string(),
            p_move: 0
        }));
        Connect4HumanModel {
            link,
            gameStarted: false,
            player1Name: "".to_string(),
            player2Name: "".to_string(),
            game: game.clone(),
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
                self.game.replace(Game::new(6, 7, false, self.player1Name.clone(), self.player2Name.clone()));
                draw_board();
                self.game.borrow_mut().start_game();
            }
            Msg::clicked(col) => {
                let state = self.game.borrow().state.clone();
                match state {
                    State::Done => {
                        clear_canvas();
                        self.game.clone().borrow_mut().state = State::NonStarted;
                    },
                    State::Running => {
                        if col.is_some() {
                            let prev_grid = self.game.borrow().grid.clone();
                            let insert_result = self.game.borrow_mut().make_move(col.unwrap() as usize);
                            if insert_result.is_ok(){
                                animate(col.unwrap() as i64,
                                        insert_result.unwrap().1 as i64,
                                        insert_result.unwrap().0 as i64,
                                        0,
                                        prev_grid,
                                        self.game.clone());
                            }
                        }
                    },
                    State::NonStarted => {

                    }
                }
                check_for_win(self.game.clone());
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn mounted(&mut self) -> ShouldRender {
        let canvas: CanvasElement = document()
            .query_selector("#gameboard")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);

        window().add_event_listener(enclose!( (canvas) move |_: ResizeEvent| {
            canvas.set_width(canvas.offset_width() as u32);
            canvas.set_height(canvas.offset_height() as u32);
        }));

        let rect = canvas.get_bounding_client_rect();

        let game_clone = self.game.clone();
        let link = self.link.clone();

        canvas.add_event_listener(enclose!( (context) move |event: ClickEvent| {
            let x_click = event.client_x() - rect.get_left() as i32;
            let y_click = event.client_y() - rect.get_top() as i32;
            for col in 0..7 {
                let x_col = 75 * col + 100;
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
               { if self.is_started() {
                    html! {
                    <div>
                        <h4>{"New Game: "} {&self.player1Name} {" VS "} {&self.player2Name}</h4>
                        <small>{"Disc Colors: "} {&self.player1Name} {" - Red    and    "} {&self.player2Name} {" - Yellow"}</small>
                     </div>
                    }
               } else {
                html!{
                    <div class="col-md-offset-3 col-md-8">
                        <input id="textbox1" type="text" placeholder="Player 1's Name" oninput=self.link.callback(|e: InputData| Msg::gotPlayer1Name(e.value))/>
                        <input id="textbox2" type="text" placeholder="Player 2's Name" oninput=self.link.callback(|e: InputData| Msg::gotPlayer2Name(e.value))/>
                        <button onclick=self.link.callback(|_| Msg::startGame)>{ "Start Game" }</button>
                    </div>
                  }
                 }
               }
               <canvas id="gameboard" height="480" width="640"></canvas>
            </div>
         </div>
        }
    }
}
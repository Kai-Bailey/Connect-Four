use yew::{prelude::*, virtual_dom::VNode, Properties};
use connect_four_cli::connect_four;

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
use connect_four_cli::connect_four::{Game, Grid, State, GameEvents};
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
    clickedCol(usize),
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
    //this.context.stroke();
    context.fill(FillRule::NonZero);
    context.restore();
}

/*
this.win = function (player) {
        this.paused = true;
        this.won = true;
        this.rejectClick = false;
        var msg = null;
        //var winner = null;
        if (player > 0) {
            msg = $scope.newGame.Player1Name + " wins";
            $scope.newGame.WinnerName = $scope.newGame.Player1Name;
           //winner = $scope.newGame.Player1Name;

        } else if (player < 0) {
            msg = $scope.newGame.Player2Name + " wins";
            $scope.newGame.WinnerName = $scope.newGame.Player2Name;
            //winner = $scope.newGame.Player2Name;
        } else {
            msg = "It's a draw";
            $scope.newGame.WinnerName = 'Draw';
           // winner = 'Draw';
        }
        //alert($scope.newGame.WinnerName+" wins");
        msg += " - Click on game board to reset";
        this.context.save();
        this.context.font = '14pt sans-serif';
        this.context.fillStyle = "#111";
        this.context.fillText(msg, 130, 20);
        this.context.restore();

        postService.save($scope.newGame, function(){

            console.log("succesfully saved");
        });

        this.canvas = document.getElementsByTagName("canvas")[0];
	        this.canvas.addEventListener('click', function (e) {
	            location.reload();
	        });

        button.disabled = false;

        console.info(msg);

    };
*/
fn print_win(winner: String, is_draw: bool) {
    let canvas: CanvasElement = document()
        .query_selector("#gameboard")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    let mut msg = "".to_string();
    if is_draw {
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

struct GuiInterface {
    selected_col: Option<usize>,
    game: Rc<RefCell<Game>>
}

impl GuiInterface {
    fn select_col(&mut self, col: usize) {
        self.selected_col = Some(col);
    }
}

impl GameEvents for GuiInterface {
    fn introduction(&self) {

    }
    fn show_grid(&self, grid: &Grid) {
        draw(grid);
    }

    fn player_turn_message(&self, p1_turn: bool) {
//        if p1_turn {
//            println!("Player 1's turn");
//        }
//        else{
//            println!("Player 2's turn");
//        }
    }

    fn player_turn(&self, col_size: usize) -> Result<usize, ()> {
        while self.selected_col.is_none() {}
        return Ok(self.selected_col.unwrap());
    }

    fn selected_column(&self, player: String, col: usize) {
//        println!("{} Selected Column {}", player, col)
    }

    fn animate_chip(&self) {

    }
    fn invalid_move(&self) {

    }
    fn game_over(&self, winner: String) {
        println!("{} has won! Congratulations!", winner);
    }
}

//enum GameState {
//    NotStarted,
//    P1Turn,
//    P2Turn,
//
//}
fn click_handler(col: usize, game:Rc<RefCell<Game>>) {
    let state = game.clone().borrow_mut().state.clone();
    match state {
        State::Done => {

        },
        State::Running => {
            game.clone().borrow_mut().make_move(col as usize);
            draw(&game.borrow_mut().grid);
        },
        State::NonStarted => {

        }
    }

    // check if game ended after move
    let state = game.clone().borrow_mut().state.clone();
    match state {
        State::Done => {
            // draw finished
            print_win(game.clone().borrow_mut().winner.clone(), false);
        },
        State::Running => {

        },
        State::NonStarted => {

        }
    }

}
impl Connect4HumanModel {

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
            p1_turn: false,
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
            Msg::clickedCol(x) => {
//                let result = self.game.make_move(x);
//                if result.is_ok() {
//                    self.draw();
//                }
//                js! {
//                     alert("hello");
//                }
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

        canvas.add_event_listener(enclose!( (context, game_clone) move |event: ClickEvent| {
            let game = game_clone.clone();
            let x_click = event.client_x() - rect.get_left() as i32;
            let y_click = event.client_y() - rect.get_top() as i32;
//            let callback = self.link.send_back(Msg::clickedCol);
            for col in 0..7 {
                let x_col = 75 * col + 100;
                if (x_click - x_col) * (x_click - x_col) <= 25 * 25 {
                    // add draw stuff here as well
                    click_handler(col as usize, game.clone());

//                    js! {
//                        alert( @{col} );
//                    };
                }
            }
        }));

        false
    }

    fn view(&self) -> VNode {
        let title;
        if self.gameStarted {
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
               { if self.gameStarted {
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
    /*
this.draw = function () {
        var x, y;
        var fg_color;
        for (y = 0; y < 6; y++) {
            for (x = 0; x < 7; x++) {
                fg_color = "transparent";
                if (this.map[y][x] >= 1) {
                    fg_color = "#ff4136";
                } else if (this.map[y][x] <= -1) {
                    fg_color = "#ffff00";
                }
                this.drawCircle(75 * x + 100, 75 * y + 50, 25, fg_color, "black");
            }
        }
    };
    */
}

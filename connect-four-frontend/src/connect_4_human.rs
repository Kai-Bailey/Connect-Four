use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct Connect4HumanModel {
    link: ComponentLink<Self>,
    gameStarted: bool,
    player1Name: String,
    player2Name: String,
}

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use stdweb::web::event::{ClickEvent, ResizeEvent};

use stdweb::web::html_element::CanvasElement;

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

impl Component for Connect4HumanModel {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Connect4HumanModel {
            link,
            gameStarted: false,
            player1Name: "".to_string(),
            player2Name: "".to_string(),
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
                draw_board();
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

        canvas.add_event_listener(enclose!( (context) move |event: ClickEvent| {
            let x_click = event.client_x() - rect.get_left() as i32;
            let y_click = event.client_y() - rect.get_top() as i32;

            for col in 0..7 {
                let x_col = 75 * col + 100;
                if (x_click - x_col) * (x_click - x_col) <= 25 * 25 {
                    js! {
                        alert( @{col} );
                    };
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
}

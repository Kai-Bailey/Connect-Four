#![recursion_limit = "1024"]

#[macro_use]
extern crate stdweb;
extern crate connect_four_cli;

mod connect_4_human;
mod connect_4_computer;
mod how_to_connect_4;
mod how_to_toot;
mod scoreboard;
mod scores;
mod welcome;

use yew::prelude::*;

use yew_router::{prelude::*, Switch};

use crate::{
    connect_4_human::Connect4HumanModel, connect_4_computer::Connect4ComputerModel,
    how_to_connect_4::HowToConnect4Model, how_to_toot::HowToTootModel,
    scoreboard::ScoreBoardModel, scores::ScoresModel, welcome::WelcomeModel,
};
use yew::virtual_dom::VNode;
use yew_router::switch::Permissive;

use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    yew::initialize();
    web_logger::init();
    js! {
        window.wasmReady();
    }
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub gameType: String,
    pub gameNumber: String,
    pub Player1Name: String,
    pub Player2Name: String,
    pub WinnerName: String,
    #[serde(with = "ts_milliseconds")]
    pub GameDate: DateTime<Utc>,
}

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <div>
                <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold" id="mySidenav">
                   <div class="w3-container">
                       <h3 class="w3-padding-64">{"Play Connect4 / TOOT-OTTO"}</h3>
                   </div>
                    <RouterAnchor<AppRoute> route=AppRoute::HowToConnect4> {"How to Connect 4"} </RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::Connect4Computer> {"Player Connect4 With Computer"} </RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::Connect4Human> {"Player Connect4 With Another Human"} </RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::HowToToot> {"How to Toot"} </RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::ScoreBoard> {"View Game History"} </RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::Scores> {"Scores"} </RouterAnchor<AppRoute>>
                    </nav>
                <div class="w3-main" style="margin-left:390px;margin-right:40px">
                    <Router<AppRoute>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Welcome => html!{<WelcomeModel />},
                                AppRoute::HowToConnect4 => html!{<HowToConnect4Model />},
                                AppRoute::Connect4Computer => html!{<Connect4ComputerModel />},
                                AppRoute::Connect4Human => html!{<Connect4HumanModel />},
                                AppRoute::HowToToot => html!{<HowToTootModel />},
                                AppRoute::ScoreBoard => html!{<ScoreBoardModel />},
                                AppRoute::Scores => html!{<ScoresModel />},
                                AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                                AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                            }
                        })
                        redirect = Router::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route)))
                        })
                    />
                </div>
            </div>
        }
    }
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/ScoreBoard"]
    ScoreBoard,
    #[to = "/HowToConnect4"]
    HowToConnect4,
    #[to = "/Connect4Computer"]
    Connect4Computer,
    #[to = "/Connect4Human"]
    Connect4Human,
    #[to = "/HowToToot"]
    HowToToot,
    #[to = "/Score"]
    Scores,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/"]
    Welcome,
}

#[derive(Debug, Switch, PartialEq, Clone, Copy)]
#[to = "/"]
pub struct ARoute;

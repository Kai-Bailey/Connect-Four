use crate::Game;
use chrono::prelude::*;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::Task;
use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct ScoreBoardModel {
    link: ComponentLink<Self>,
    games: Option<Vec<Game>>,
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub enum Msg {
    FetchGamesComplete(Vec<Game>),
    FetchGamesFailed,
}

impl Component for ScoreBoardModel {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ScoreBoardModel {
            link,
            games: None,
            fetch_service: FetchService::new(),
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchGamesComplete(body) => {
                self.games = Some(body);
            }
            Msg::FetchGamesFailed => {
                js! {alert("Failed to load data...")}
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn mounted(&mut self) -> ShouldRender {
        let get_request = Request::get("http://localhost:8000/games")
            .body(Nothing)
            .unwrap();
        let callback = self
            .link
            .callback(|response: Response<Json<Result<Vec<Game>, _>>>| {
                if let (meta, Json(Ok(body))) = response.into_parts() {
                    if meta.status.is_success() {
                        return Msg::FetchGamesComplete(body);
                    }
                }
                Msg::FetchGamesFailed
            });

        let task = self.fetch_service.fetch(get_request, callback);
        self.fetch_task = Some(task.unwrap());
        true
    }

    fn view(&self) -> VNode {
        html! {
            <div class="w3-container" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round" />
                <div id="game-stream">
                    <table>
                        <tr>
                            <th>{"Game Type"}</th>
                            <th>{"Player1"}</th>
                            <th>{"Player2"}</th>
                            <th>{"Winner"}</th>
                            <th>{"When Played"}</th>
                        </tr>
                        {
                            match &self.games {
                                None => html!{{"Loading..."}},
                                Some(games) => {
                                    games.into_iter().map(|game| self.view_row(game)).collect::<Html>()
                                }
                            }
                        }
                    </table>
                </div>
            </div>
        }
    }
}

impl ScoreBoardModel {
    fn view_row(&self, game: &Game) -> Html {
        html! {
            <tr>
                <td>{game.gameType.clone()}</td>
                <td>{game.Player1Name.clone()}</td>
                <td>{game.Player2Name.clone()}</td>
                <td>{game.WinnerName.clone()}</td>
                <td>{game.GameDate.format("%b %e %Y %H:%M")}</td>
            </tr>
        }
    }
}

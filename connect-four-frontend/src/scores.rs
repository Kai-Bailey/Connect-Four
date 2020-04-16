use crate::{Game, PlayerWins};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct ScoresModel {
    link: ComponentLink<Self>,
    games: Option<Vec<Game>>,
    gamesWonPerPlayer: Option<Vec<PlayerWins>>,
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
    fetch_task2: Option<FetchTask>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub enum Msg {
    FetchGamesComplete(Vec<Game>),
    FetchGamesFailed,
    FetchPlayerWinsComplete(Vec<PlayerWins>),
    FetchPlayerWinsFailed,
}

impl Component for ScoresModel {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ScoresModel {
            link,
            games: None,
            gamesWonPerPlayer: None,
            fetch_service: FetchService::new(),
            fetch_task: None,
            fetch_task2: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchGamesComplete(body) => {
                self.games = Some(body);
            }
            Msg::FetchGamesFailed => {
                js! {alert("Failed to load game data...")}
            }
            Msg::FetchPlayerWinsComplete(body) => {
                self.gamesWonPerPlayer = Some(body);
            }
            Msg::FetchPlayerWinsFailed => {
                js! {alert("Failed to load wins data...")}
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

        let get_request_wins = Request::get("http://localhost:8000/wins")
            .body(Nothing)
            .unwrap();
        let callback_wins =
            self.link
                .callback(|response: Response<Json<Result<Vec<PlayerWins>, _>>>| {
                    if let (meta, Json(Ok(body))) = response.into_parts() {
                        if meta.status.is_success() {
                            return Msg::FetchPlayerWinsComplete(body);
                        }
                    }
                    Msg::FetchPlayerWinsFailed
                });

        let task2 = self.fetch_service.fetch(get_request_wins, callback_wins);
        self.fetch_task2 = Some(task2.unwrap());
        true
    }

    fn view(&self) -> VNode {
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Score Board"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round" />
                <div>
                    <h4>{"Games Won by Computer"}</h4>
                </div>
                <table>
                    <tr>
                        <th>{"Total Games Played"}</th>
                        <th>{"Games Against Computer"}</th>
                        <th>{"Games Computer Won"}</th>
                    </tr>
                    {
                        match &self.games {
                            None => html!{{"Loading..."}},
                            Some(games) => {
                                html!{
                                    <tr>
                                        <td>{games.len()}</td>
                                        <td>{games.into_iter().filter(|game| game.Player2Name == "Computer").count()}</td>
                                        <td>{games.into_iter().filter(|game| game.WinnerName == "Computer").count()}</td>
                                    </tr>
                                }
                            }
                        }
                    }
                </table>
                <div>
                    <h4>{"Details of Games Won by Computer"}</h4>
                </div>
                <table>
                    <tr>
                        <th>{"Game Type"}</th>
                        <th>{"Winner"}</th>
                        <th>{"Played Against"}</th>
                        <th>{"When Played"}</th>
                    </tr>
                    {
                        match &self.games {
                            None => html!{{"Loading..."}},
                            Some(games) => {
                                games.into_iter().filter_map(|game|
                                    if(game.WinnerName == "Computer"){
                                        return Some(self.view_row_computer_won(game));
                                    } else {
                                        return None;
                                    }).collect::<Html>()
                            }
                        }
                    }
                </table>
                <div>
                    <h4>{"Number of Wins For Each Player"}</h4>
                </div>
                <div id="game-stream">
                    <table>
                        <tr>
                            <th>{"Player Name"}</th>
                            <th>{"No. of Wins"}</th>
                        </tr>
                        {
                            match &self.gamesWonPerPlayer {
                                None => html!{{"Loading..."}},
                                Some(gamesWon) => {
                                    gamesWon.into_iter().map(|player| self.view_row_player_wins(player)).collect::<Html>()
                                }
                            }
                        }
                    </table>
                </div>
            </div>
        }
    }
}

impl ScoresModel {
    fn view_row_computer_won(&self, game: &Game) -> Html {
        html! {
            <tr>
                <td>{game.gameType.clone()}</td>
                <td>{game.WinnerName.clone()}</td>
                <td>{game.Player1Name.clone()}</td>
                <td>{game.GameDate.format("%b %e %Y %H:%M")}</td>
            </tr>
        }
    }

    fn view_row_player_wins(&self, player: &PlayerWins) -> Html {
        html! {
            <tr>
                <td>{player._id.clone()}</td>
                <td>{player.count.clone()}</td>
            </tr>
        }
    }
}

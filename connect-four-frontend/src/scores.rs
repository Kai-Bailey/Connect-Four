use crate::Game;
use chrono::prelude::*;
use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct PlayerWins {
    PlayerName: String,
    Wins: String,
}

pub struct ScoresModel {
    link: ComponentLink<Self>,
    games: Option<Vec<Game>>,
    gamesWonPerPlayer: Option<Vec<PlayerWins>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub enum Msg {}

impl Component for ScoresModel {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ScoresModel {
            link,
            games: None,
            gamesWonPerPlayer: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn mounted(&mut self) -> ShouldRender {
        self.games = Some(vec![
            Game {
                gameType: "human".to_string(),
                gameNumber: "5".to_string(),
                Player1Name: "Dinula".to_string(),
                Player2Name: "Kai".to_string(),
                WinnerName: "Kai".to_string(),
                GameDate: Utc.ymd(2019, 3, 17).and_hms(16, 43, 0),
            },
            Game {
                gameType: "Computer".to_string(),
                gameNumber: "5".to_string(),
                Player1Name: "Dinula".to_string(),
                Player2Name: "Computer".to_string(),
                WinnerName: "Computer".to_string(),
                GameDate: Utc.ymd(2018, 3, 17).and_hms(14, 3, 3),
            },
            Game {
                gameType: "human".to_string(),
                gameNumber: "4".to_string(),
                Player1Name: "Hugo".to_string(),
                Player2Name: "Kai".to_string(),
                WinnerName: "Kai".to_string(),
                GameDate: Utc.ymd(2017, 3, 17).and_hms(6, 41, 23),
            },
        ]);

        self.gamesWonPerPlayer = Some(vec![
            PlayerWins {
                PlayerName: "Kai".to_string(),
                Wins: "5000".to_string(),
            },
            PlayerWins {
                PlayerName: "Dinula".to_string(),
                Wins: "0".to_string(),
            },
            PlayerWins {
                PlayerName: "Hugo".to_string(),
                Wins: "0".to_string(),
            },
            PlayerWins {
                PlayerName: "Computer".to_string(),
                Wins: "20".to_string(),
            },
        ]);

        true
    }

    fn view(&self) -> VNode {
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Score Board"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
                <div>
                    <h4>{"Games Won by Computer"}</h4>
                </div>
                <table>
                    <tr>
                        <th>{"Total Games Played"}</th>
                        <th>{"Games Against Computer"}</th>
                        <th>{"Games Computer Won"}</th>
                    </tr>
                    {match &self.games {
                        None => html!{{"Loading..."}},
                        Some(games) => {
                            html!{
                            <tr>
                                <td>{games.len()}</td>
                                <td>{games.into_iter().filter( |game| game.Player2Name == "Computer").count()}</td>
                                <td>{games.into_iter().filter( |game| game.WinnerName == "Computer").count()}</td>
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
                        <th>{"Sl. No."}</th>
                        <th>{"Game Type"}</th>
                        <th>{"Winner"}</th>
                        <th>{"Played Against"}</th>
                        <th>{"When Played"}</th>
                    </tr>
                    {match &self.games {
                        None => html!{{"Loading..."}},
                        Some(games) => {
                            games.into_iter().filter_map(|game|
                                if(game.WinnerName == "Computer"){
                                    return Some(self.view_row_computer_won(game));
                                } else {
                                    return None;
                                }
                        ).collect::<Html>()
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
                        {match &self.gamesWonPerPlayer {
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
                <td>{game.gameNumber.clone()}</td>
                <td>{game.gameType.clone()}</td>
                <td>{game.Player1Name.clone()}</td>
                <td>{game.WinnerName.clone()}</td>
                <td>{game.GameDate.format("%b %e %Y %H:%M")}</td>
            </tr>
        }
    }

    fn view_row_player_wins(&self, player: &PlayerWins) -> Html {
        html! {
            <tr>
                <td>{player.PlayerName.clone()}</td>
                <td>{player.Wins.clone()}</td>
            </tr>
        }
    }
}

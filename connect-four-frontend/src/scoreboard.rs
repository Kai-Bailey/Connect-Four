use crate::Game;
use chrono::prelude::*;
use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct ScoreBoardModel {
    link: ComponentLink<Self>,
    games: Option<Vec<Game>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub enum Msg {}

impl Component for ScoreBoardModel {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ScoreBoardModel { link, games: None }
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
        true
    }

    fn view(&self) -> VNode {
        html! {
            <div class="w3-container" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>

                <div id="game-stream">
                    <table>
                        <tr>
                            <th>{"Game-ID"}</th>
                            <th>{"Game Type"}</th>
                            <th>{"Player1"}</th>
                            <th>{"Player2"}</th>
                            <th>{"Winner"}</th>
                            <th>{"When Played"}</th>
                        </tr>
                        {match &self.games {
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
                <td>{game.gameNumber.clone()}</td>
                <td>{game.gameType.clone()}</td>
                <td>{game.Player1Name.clone()}</td>
                <td>{game.Player2Name.clone()}</td>
                <td>{game.WinnerName.clone()}</td>
                <td>{game.GameDate.format("%b %e %Y %H:%M")}</td>
            </tr>
        }
    }
}

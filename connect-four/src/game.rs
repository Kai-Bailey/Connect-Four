use serde::{Serialize, Deserialize};
use mongodb::bson;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    #[serde(rename = "_id")]
    pub id: Option<bson::oid::ObjectId>,
    gameType: String,
    gameNumber: String,
    Player1Name: String,
    Player2Name: String,
    WinnerName: String,
    GameDate: bson::UtcDateTime,
}

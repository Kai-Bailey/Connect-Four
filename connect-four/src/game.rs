use serde::{Serialize, Deserialize};
use mongodb::bson;

// follow this for deserializng datetime - https://serde.rs/custom-date-format.html

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    #[serde(rename = "_id")]
    pub id: Option<bson::oid::ObjectId>,
    pub gameType: String,
    pub gameNumber: String,
    pub Player1Name: String,
    pub Player2Name: String,
    pub WinnerName: String,
    pub GameDate: bson::UtcDateTime,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InsertableGame {
    pub gameType: String,
    pub gameNumber: String,
    pub Player1Name: String,
    pub Player2Name: String,
    pub WinnerName: String,
    pub GameDate: bson::UtcDateTime,
}


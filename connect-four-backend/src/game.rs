use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use mongodb::bson;
use serde::{Deserialize, Serialize};

// follow this for deserializng datetime - https://serde.rs/custom-date-format.html

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    #[serde(rename = "_id")]
    pub id: Option<bson::oid::ObjectId>,
    pub gameType: String,
    pub Player1Name: String,
    pub Player2Name: String,
    pub WinnerName: String,
    #[serde(with = "ts_milliseconds")]
    pub GameDate: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InsertableGame {
    pub gameType: String,
    pub Player1Name: String,
    pub Player2Name: String,
    pub WinnerName: String,
    #[serde(with = "ts_milliseconds")]
    pub GameDate: DateTime<Utc>,
}

// Serialiazable game, designed to match conect_four_backend except
// except without the date since you cannot get the current date in
// webassembly.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SerializableGame {
    pub gameType: String,
    pub Player1Name: String,
    pub Player2Name: String,
    pub WinnerName: String,
}

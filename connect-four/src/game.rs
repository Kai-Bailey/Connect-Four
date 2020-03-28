use serde::{Serialize, Deserialize};
use mongodb::bson;
use chrono::{DateTime, Utc};

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
//    #[serde(with = "utc_date_format")]
//    pub GameDate: DateTime<Utc>,
    pub GameDate: bson::UtcDateTime,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InsertableGame {
    pub gameType: String,
    pub gameNumber: String,
    pub Player1Name: String,
    pub Player2Name: String,
    pub WinnerName: String,
//    #[serde(with = "utc_date_format")]
//    pub GameDate: DateTime<Utc>,
    pub GameDate: bson::UtcDateTime
}

mod utc_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";
//    "2020-03-24T17:59:10.412Z"

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

use crate::game::{Game, InsertableGame, SerializableGame};
use crate::mongo_connection::Conn;
use crate::r2d2_mongodb::mongodb::db::ThreadedDatabase;
use chrono::Utc;
use mongodb::{bson, coll::results::DeleteResult, doc, error::Error, oid::ObjectId};

pub fn get_all_games_handler(connection: &Conn) -> Result<Vec<Game>, Error> {
    let cursor = connection.collection("games").find(None, None).unwrap();

    cursor
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(err) => {
                    println!("get_all_games_handler -> {:?}", err);
                    Err(Error::DefaultError(String::from("")))
                }
            },
            Err(err) => {
                println!("get_all_games_handler -> {:?}", err);
                Err(err)
            }
        })
        .collect::<Result<Vec<Game>, Error>>()
}

pub fn get_game_with_id_handler(id: ObjectId, conn: &Conn) -> Result<Option<Game>, Error> {
    match conn
        .collection("games")
        .find_one(Some(doc! {"_id": id}), None)
    {
        Ok(r) => match r {
            Some(r_doc) => match bson::from_bson(bson::Bson::Document(r_doc)) {
                Ok(r_model) => Ok(Some(r_model)),
                Err(_) => Err(Error::DefaultError(String::from("Failed to reverse BSON"))),
            },
            None => Ok(None),
        },
        Err(err) => Err(err),
    }
}

pub fn get_wins_by_player(conn: &Conn) -> Result<Vec<bson::Document>, Error> {
    match conn.collection("games").aggregate(
        vec![doc! {
            "$group" => {
                "_id" => "$WinnerName",
                "count": { "$sum": 1 }
            }
        }],
        None,
    ) {
        Ok(doc) => Ok(doc.into_iter().map(|doc| doc.unwrap()).collect::<Vec<_>>()),
        Err(err) => Err(err),
    }
}

pub fn update_game_with_id_handler(
    id: bson::oid::ObjectId,
    game: Game,
    connection: &Conn,
) -> Result<Game, Error> {
    let mut game = game.clone();
    game.id = Some(id.clone());
    match bson::to_bson(&game) {
        Ok(model_bson) => match model_bson {
            bson::Bson::Document(doc) => {
                match connection
                    .collection("games")
                    .replace_one(doc! {"_id": id}, doc, None)
                {
                    Ok(_) => Ok(game),
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::DefaultError(String::from(
                "Error: Document not created",
            ))),
        },
        Err(_) => Err(Error::DefaultError(String::from("Failed to generate BSON"))),
    }
}

pub fn insert_game_handler(game: SerializableGame, connection: &Conn) -> Result<ObjectId, Error> {
    let new_game = InsertableGame {
        gameType: game.gameType,
        Player1Name: game.Player1Name,
        Player2Name: game.Player2Name,
        WinnerName: game.WinnerName,
        GameDate: Utc::now(),
    };
    match bson::to_bson(&new_game) {
        Ok(model_bson) => match model_bson {
            bson::Bson::Document(model_doc) => {
                match connection.collection("games").insert_one(model_doc, None) {
                    Ok(res) => match res.inserted_id {
                        Some(res) => match bson::from_bson(res) {
                            Ok(res) => Ok(res),
                            Err(_) => Err(Error::DefaultError(String::from("Failed to read BSON"))),
                        },
                        None => Err(Error::DefaultError(String::from("None"))),
                    },
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::DefaultError(String::from(
                "Failed to create Document",
            ))),
        },
        Err(_) => Err(Error::DefaultError(String::from("Failed to create BSON"))),
    }
}

pub fn delete_game_handler(id: ObjectId, conn: &Conn) -> Result<DeleteResult, Error> {
    conn.collection("games").delete_one(doc! {"_id": id}, None)
}

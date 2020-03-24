use crate::mongo_connection::Conn;
use crate::game::Game;
use mongodb::{bson, oid::ObjectId, coll::results::DeleteResult, doc, error::Error};
use crate::r2d2_mongodb::mongodb::db::ThreadedDatabase;

pub fn get_all_games_handler(connection: &Conn) -> Result<Vec<Game>, Error> {
    let cursor = connection.collection("games").find(None, None).unwrap();

    cursor
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(err) => Err(Error::DefaultError(String::from(""))),
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<Game>, Error>>()
}

pub fn get_game_with_id_handler(id: ObjectId, conn: &Conn) -> Result<Option<Game>, Error> {
    match conn.collection("games").find_one(Some(doc! {"_id": id}), None)
        {
            Ok(r) => match r {
                Some(r_doc) => match bson::from_bson(bson::Bson::Document(r_doc)) {
                    Ok(r_model) => Ok(Some(r_model)),
                    Err(_) => Err(Error::DefaultError(String::from("Failed to reverse BSON",))),
                },
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
}

pub fn update_game_with_id_handler(id: bson::oid::ObjectId, game: Game, connection: &Conn) {
    let mut game = game.clone();
    game.id = Some(id.clone());
    match bson::to_bson(&game) {
        Ok(model_bson) => match model_bson {
            bson::Bson::Document(doc) => {
                connection
                    .collection("games")
                    .replace_one(doc! {"_id": id}, doc, None);
            }
            _ => {},
        },
        Err(_) => {},
    };
}

pub fn add_game(game: Game, connection: &Conn) {

}

/*
pub fn insert(cats: Cat, connection: &Conn) -> Result<ObjectId, Error> {
    let insertable = InsertableCat::from_cat(cats.clone());
    match bson::to_bson(&insertable) {
        Ok(model_bson) => match model_bson {
            bson::Bson::Document(model_doc) => {
                match connection
                    .collection(COLLECTION)
                    .insert_one(model_doc, None)
                {
                    Ok(res) => match res.inserted_id {
                        Some(res) => match bson::from_bson(res) {
                            Ok(res) => Ok(res),
                            Err(_) => Err(Error::DefaultError(String::from("Failed to read BSON")))
                        },
                        None => Err(Error::DefaultError(String::from("None")))
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
*/

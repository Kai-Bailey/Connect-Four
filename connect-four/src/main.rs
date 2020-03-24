#![feature(decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
//extern crate rocket;
//extern crate mongodb;
//extern crate r2d2;
extern crate r2d2_mongodb;
//extern crate rocket_contrib;
//#[macro_use]
//extern crate serde_derive;
//extern crate serde_json;

use crate::mongo_connection::Conn;

#[macro_use] extern crate rocket;

use mongodb::{bson, coll::results::DeleteResult, doc, error::Error, oid::ObjectId};
use serde::{Serialize, Deserialize};
mod mongo_connection;
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use crate::r2d2_mongodb::mongodb::db::ThreadedDatabase;


#[derive(Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "_id")]
    pub id: Option<bson::oid::ObjectId>,
    gameType: String,
    gameNumber: String,
    Player1Name: String,
    Player2Name: String,
    WinnerName: String,
    GameDate: String
}

fn error_status(error: Error) -> Status {
    match error {
        Error::CursorNotFoundError => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

// index.js
#[get("/")]
pub fn games_get_all(connection: Conn) -> Result<Json<Vec<Game>>, Status> {
    match games_all_handler(&connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
}


pub fn games_all_handler(connection: &Conn) -> Result<Vec<Game>, Error> {
    let cursor = connection.collection("games").find(None, None).unwrap();

    cursor
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(_) => Err(Error::DefaultError(String::from(""))),
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<Game>, Error>>()
}


#[post("/")]
fn games_post(connection: Conn) -> Result<Json<Vec<Game>>, Status> {
    Err(Status::InternalServerError)
}

#[put("/<id>")]
fn games_id_put(id: i64, connection: Conn) -> Result<Json<Vec<Game>>, Status> {
    Err(Status::NotFound)
}

#[get("/<id>")]
fn games_id_get(id: i64, connection: Conn) -> Result<Json<Vec<Game>>, Status> {
    Err(Status::InternalServerError)
}


#[delete("/<id>")]
fn games_id_delete(id: i64, connection: Conn) -> Result<Json<Vec<Game>>, Status> {
    Err(Status::NotFound)
}

// users.js
#[get("/")]
fn users(connection: Conn) -> Result<Json<Vec<Game>>, Status> {
    Err(Status::NotFound)
}

// api.js
#[get("/posts")]
fn api_posts_get(connection: Conn) -> Result<Json<Vec<Game>>, Status> {
    Err(Status::NotFound)
}

#[post("/posts")]
fn api_posts_post(connection: Conn) -> Result<Json<Vec<Game>>, Status> {
    Err(Status::NotFound)
}

#[put("/posts/<id>")]
fn api_posts_id_put(id: i64, connection: Conn) -> Result<Json<Vec<Game>>, Status> {
    Err(Status::NotFound)
}

#[get("/posts/<id>")]
fn api_posts_id_get(id: i64, connection: Conn) -> Result<Json<Vec<Game>>, Status> {
    Err(Status::NotFound)
}

#[delete("/posts/<id>")]
fn api_posts_id_delete(id: i64, connection: Conn) -> Result<Json<Vec<Game>>, Status> {
    Err(Status::NotFound)
}

fn main() {

    rocket::ignite()
        .manage(mongo_connection::init_connection())
        .mount("/", StaticFiles::from("./public"))
        .mount("/games", routes![games_get_all, games_post, games_id_put, games_id_get, games_id_delete])
        .mount("/users", routes![users])
        .mount("/api", routes![api_posts_get, api_posts_post, api_posts_id_put, api_posts_id_get, api_posts_id_delete])
        .launch();
}

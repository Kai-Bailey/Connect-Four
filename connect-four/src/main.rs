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

mod mongo_connection;
mod game;
mod game_repository;

use crate::mongo_connection::Conn;

#[macro_use] extern crate rocket;

use mongodb::{bson, coll::results::DeleteResult, doc, error::Error, oid::ObjectId};
use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use crate::r2d2_mongodb::mongodb::db::ThreadedDatabase;
use crate::game::Game;


fn error_status(error: Error) -> Status {
    match error {
        Error::CursorNotFoundError => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

// index.js
#[get("/")]
pub fn get_all_games(connection: Conn) -> Result<Json<Vec<Game>>, Status> {
    match game_repository::get_all_games_handler(&connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
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
fn get_game_with_id(id: String, connection: Conn) -> Result<Json<Game>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(r) => match game_repository::get_game_with_id_handler(r, &connection) {
            Ok(res) => Ok(Json(res.unwrap())),
            Err(err) => Err(error_status(err)),
        }
        Err(_) => Err(error_status(Error::DefaultError(String::from("Error parsing ObjectId"))))
    }
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
        .mount("/games", routes![get_all_games, games_post, games_id_put, get_game_with_id, games_id_delete])
        .mount("/users", routes![users])
        .mount("/api", routes![api_posts_get, api_posts_post, api_posts_id_put, api_posts_id_get, api_posts_id_delete])
        .launch();
}

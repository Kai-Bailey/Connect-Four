#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;

// index.js
#[get("/")]
fn games_get() -> String {
    String::from("TODO")
}

#[post("/")]
fn games_post() -> String {
    String::from("TODO")
}

#[put("/<id>")]
fn games_id_put(id: i64) -> String {
    String::from("TODO")
}

#[get("/<id>")]
fn games_id_get(id: i64) -> String {
    String::from("TODO")
}

#[delete("/<id>")]
fn games_id_delete(id: i64) -> String {
    String::from("TODO")
}

// users.js
#[get("/")]
fn users() -> String {
    String::from("respond with a resource")
}

// api.js
#[get("/posts")]
fn api_posts_get() -> String {
    String::from("TODO")
}

#[post("/posts")]
fn api_posts_post() -> String {
    String::from("TODO")
}

#[put("/posts/<id>")]
fn api_posts_id_put(id: i64) -> String {
    String::from("TODO")
}

#[get("/posts/<id>")]
fn api_posts_id_get(id: i64) -> String {
    String::from("TODO")
}

#[delete("/posts/<id>")]
fn api_posts_id_delete(id: i64) -> String {
    String::from("TODO")
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("./public"))
        .mount("/games", routes![games_get, games_post, games_id_put, games_id_get, games_id_delete])
        .mount("/users", routes![users])
        .mount("/api", routes![api_posts_get, api_posts_post, api_posts_id_put, api_posts_id_get, api_posts_id_delete])
        .launch();
}

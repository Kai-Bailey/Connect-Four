/*
    Procedure for setting up Mongodb connection pool for allowing multiple users to modify
    database at once referenced from the following article:
    https://medium.com/@louis.beaumont/rest-api-with-rust-mongodb-10eeb6bd51d7
*/

use r2d2::PooledConnection;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
use std::ops::Deref;
use rocket::{Request, State, Outcome};
use rocket::request::{self, FromRequest};
use rocket::http::Status;


type Pool = r2d2::Pool<MongodbConnectionManager>;
pub struct Conn(pub PooledConnection<MongodbConnectionManager>);

pub fn init_connection() -> Pool {
    let mongo_address = "localhost";
    let mongo_port = 27017 as u16;
    let db_name = "Connect4DB";
    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host(&mongo_address, mongo_port)
            .with_db(&db_name)
            .build(),
    );

    match Pool::builder().max_size(64).build(manager) {
        Ok(pool) => pool,
        Err(e) => panic!(e),
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(db) => Outcome::Success(Conn(db)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = PooledConnection<MongodbConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
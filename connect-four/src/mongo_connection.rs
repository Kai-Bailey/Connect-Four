use r2d2::PooledConnection;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager, VerifyPeer};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::env;
use std::ops::Deref;

type Pool = r2d2::Pool<MongodbConnectionManager>;

pub struct Conn(pub PooledConnection<MongodbConnectionManager>);

/*
    create a connection pool of mongodb connections to allow a lot of users to modify db at same time.
*/
pub fn init_connection() -> Pool {
    let mongo_addr = "localhost";
    let mongo_port = 27017 as u16;
    let db_name = "Connect4DB";
    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host(&mongo_addr, mongo_port)
            .with_db(&db_name)
            .build(),
    );
    match Pool::builder().max_size(64).build(manager) {
        Ok(pool) => pool,
        Err(e) => panic!("Error: failed to create mongodb pool {}", e),
    }
}

/*
    Create a implementation of FromRequest so Conn can be provided at every api endpoint
*/
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

/*
    When Conn is dereferencd, return the mongo connection.
*/
impl Deref for Conn {
    type Target = PooledConnection<MongodbConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

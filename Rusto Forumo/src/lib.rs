#![recursion_limit = "128"]
#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate bcrypt;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_derives;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate tera;

use dotenv::dotenv;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::env;
use std::ops::Deref;

pub mod schema;
pub mod post_submission;
pub mod user_authentication;

pub fn database_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let DB_URL = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let secure = ConnectionManager::<PgConnection>::new(DB_URL);
    Pool::builder().build(secure).expect("Failed to create pool")
}

pub struct DbConn(PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool<ConnectionManager<PgConnection>>>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

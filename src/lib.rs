use diesel::pg::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}

pub mod models;
pub mod schema;
pub mod repo;
pub mod server;
pub mod server_actix;

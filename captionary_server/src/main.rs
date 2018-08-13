#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate r2d2;
extern crate r2d2_diesel;

pub mod controllers;
pub mod database;
pub mod models;
pub mod schema;

use dotenv::dotenv;
use std::env;
use rocket_contrib::Json;
use serde_json::Value;

fn main() {

    let conn_pool = establish_db_connection();

    rocket::ignite()
        .manage(conn_pool)
        .mount("/api/v1/captions", routes![
            controllers::captions::index,
            controllers::captions::show
            ])
        .catch(catchers![not_found])
        .launch();
}

#[catch(404)]
fn not_found() -> Json<Value> {
    Json(json!({  }))
}

fn establish_db_connection() -> database::ConnectionPool {
    dotenv().ok();

    let database_env_var = "DATABASE_URL";

    let db_url = env::var(database_env_var)
        .expect(&format!("Please set env var: {}", database_env_var));

    database::init_connection_pool(db_url)
}

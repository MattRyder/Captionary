#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#![warn(unused_extern_crates)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate fake;
extern crate curl;
extern crate frank_jwt;
extern crate futures;
extern crate lapin_futures as lapin;
extern crate names;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tokio;

pub mod amqp;
pub mod controllers;
pub mod database;
pub mod models;
pub mod schema;
pub mod util;

use dotenv::dotenv;
use rocket_contrib::Json;
use serde_json::Value;
use std::env;
use std::thread;

use database::ConnectionPool;

fn main() {
    dotenv().ok();

    let conn_pool = establish_db_connection();

    thread::spawn(|| {
        rocket::ignite()
            .manage(conn_pool)
            .manage(create_amqp_client())
            .mount(
                "/api/v1/captions",
                routes![controllers::captions::index, controllers::captions::show],
            ).mount("/api/v1/users", routes![controllers::users::create])
            .mount("/api/v1/rooms", routes![controllers::rooms::join])
            .catch(catchers![not_found])
            .launch();
    });

    let conn = database::DatabaseConnection(establish_db_connection().get().unwrap());
    create_amqp_client().consume(conn);
}

#[catch(404)]
fn not_found() -> Json<Value> {
    Json(json!({}))
}

fn create_amqp_client() -> amqp::Client {
    let ampq_host = env::var("AMPQ_HOST").expect("Please set AMPQ_HOST");
    let ampq_user = env::var("AMPQ_USER").expect("Please set AMPQ_USER");
    let ampq_pass = env::var("AMPQ_PASS").expect("Please set AMPQ_PASS");

    let amqp_credentials = amqp::Credentials {
        host: ampq_host.parse().unwrap(),
        username: ampq_user,
        password: ampq_pass,
    };

    amqp::Client::new(amqp_credentials)
}

fn establish_db_connection() -> database::ConnectionPool {
    let database_env_var = "DATABASE_URL";

    let db_url =
        env::var(database_env_var).expect(&format!("Please set env var: {}", database_env_var));

    database::init_connection_pool(db_url)
}

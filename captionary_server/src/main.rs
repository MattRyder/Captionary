#![feature(plugin, custom_derive)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate chrono;
extern crate curl;
extern crate dotenv;
extern crate frank_jwt;
extern crate futures;
extern crate lapin_futures as lapin;
extern crate names;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tokio;
extern crate ws;
extern crate fake;

pub mod amqp;
pub mod database;
pub mod models;
pub mod schema;
pub mod util;
pub mod web_socket;

use dotenv::dotenv;
use std::env;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

use amqp::{Client, Credentials};
use database::ConnectionPool;
use web_socket::server::Server;
use web_socket::user_handler::RoomsClientsMap;

fn main() {
    dotenv().ok();

    let (tx, rx) = channel();

    let rooms_clients_map = Arc::new(RoomsClientsMap::new());

    thread::spawn(move || {
        let websocket_env_var = "WEBSOCKET_HOST";

        let websocket_host = env::var(websocket_env_var)
            .expect(&format!("Please set env var: {}", websocket_env_var));

        let client = create_amqp_client();
        let conn_pool = establish_db_connection();

        let mut ws_server = Server::default();
        ws_server.connect(
            &websocket_host,
            conn_pool,
            client,
            rx,
            Arc::clone(&rooms_clients_map),
        );

        println!("Killing WebSocket Server...");
    });

    let conn = database::DatabaseConnection(establish_db_connection().get().unwrap());
    let client = create_amqp_client();
    amqp::Client::consume(client, conn, tx);
}

fn create_amqp_client() -> Client {
    let ampq_host = env::var("AMPQ_HOST").expect("Please set AMPQ_HOST");
    let ampq_user = env::var("AMPQ_USER").expect("Please set AMPQ_USER");
    let ampq_pass = env::var("AMPQ_PASS").expect("Please set AMPQ_PASS");

    let amqp_credentials = Credentials {
        host: ampq_host.parse().unwrap(),
        username: ampq_user,
        password: ampq_pass,
    };

    Client::new(amqp_credentials)
}

fn establish_db_connection() -> ConnectionPool {
    let database_env_var = "DATABASE_URL";

    let db_url =
        env::var(database_env_var).expect(&format!("Please set env var: {}", database_env_var));

    database::init_connection_pool(db_url)
}
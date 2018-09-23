#![feature(plugin, custom_derive, vec_remove_item)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate chrono;
extern crate curl;
extern crate dotenv;
extern crate fake;
extern crate frank_jwt;
extern crate futures;
extern crate lapin_futures as lapin;
extern crate names;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate tokio;
extern crate ws;

pub mod amqp;
pub mod database;
pub mod models;
pub mod schema;
pub mod web_socket;
pub mod flickr;
pub mod jwt;

use dotenv::dotenv;
use std::env;
use std::sync::mpsc::channel;
use std::thread;

use amqp::{Client, Credentials};
use database::ConnectionPool;
use web_socket::server::Server;

fn main() {
    dotenv().ok();
    let (ws_event_tx, ws_event_rx) = channel();

    {
        let client = create_amqp_client();
        let conn_pool = establish_db_connection();

        let mut ws_server = Server::new(conn_pool, client);

        {
            let event_tx = ws_event_tx.clone();
            thread::spawn(move || {
                let conn = database::DatabaseConnection(establish_db_connection().get().unwrap());
                let client = create_amqp_client();
                amqp::Client::consume(client, conn, event_tx);
            });
        }

        {
            let event_tx = ws_event_tx.clone();

            println!("Listening for incoming WebSocket Clients...");
            thread::spawn(|| {
                let client = create_amqp_client();
                let conn_pool = establish_db_connection();

                let websocket_env_var = "WEBSOCKET_HOST";

                let websocket_host = env::var(websocket_env_var)
                    .expect(&format!("Please set env var: {}", websocket_env_var));

                let mut ws_server = Server::new(conn_pool, client);
                ws_server.connect(&websocket_host, event_tx);
                println!("Connection over...");
            });
        }

        {
            println!("Waiting for WebSocket Events to be raised...");
            ws_server.handle_events(ws_event_rx);
        }

        println!("Captionary Server booted...");
    }
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

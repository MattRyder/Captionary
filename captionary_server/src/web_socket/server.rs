use std::collections::HashMap;
use ws;

use database::ConnectionPool;
use web_socket::user_handler::UserHandler;

pub struct Server {
    room_clients_map: HashMap<i32, Vec<UserHandler>>,
}

impl Server {
    pub fn connect(websocket_host: &String, connection_pool: ConnectionPool) {
        ws::listen(websocket_host, |out| UserHandler {
            socket_handle: out,
            user_id: None,
            db_connection: connection_pool.clone(),
        }).expect("Oh no! WebSocket Server failed to connect!");
    }
}

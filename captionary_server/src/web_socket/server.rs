use std::sync::Arc;
use std::sync::mpsc::Receiver;
use ws;
use ws::Message as WsMessage;
use serde_json::ser;
use std::thread;

use amqp::{Client, Message as Message};
use database::ConnectionPool;
use web_socket::user_handler::{UserHandler, ServerMessage, RoomsClientsMap};

#[derive(Default)]
pub struct Server {
}

impl Server {
    pub fn connect(
        &mut self,
        websocket_host: &String,
        connection_pool: ConnectionPool,
        amqp_client: Client,
        receiver: Receiver<Message>,
        room_clients_map: Arc<RoomsClientsMap>
    ) {
        thread::spawn(move || {
            loop {
                println!("Uh.. guys?");
                let message = receiver.recv().unwrap();

                println!("Got a message to send to clients: {:?}", &message);

                match message {
                    Message::StartGameForRoom(room_id) => {
                        if let Some(clients) = room_clients_map.get(&room_id) {
                            let msg = ServerMessage::GameStartedResponse { };
                            let msg = ser::to_string_pretty(&msg).unwrap();
                            let msg = WsMessage::Text(msg);
                            
                            for client in clients {
                                let msg_dupe = msg.clone();
                                client.socket_handle.send(msg_dupe).unwrap();
                            }
                        }
                        
                    }
                    _ => (),
                }
            }
        });

        ws::listen(websocket_host, |out| {        
            let user_handler = UserHandler {
                socket_handle: out,
                user_id: None,
                db_connection: connection_pool.clone(),
                amqp_client: amqp_client.clone(),
            };

            return user_handler;
        }).expect("Oh no! WebSocket Server failed to connect!");
    }
}

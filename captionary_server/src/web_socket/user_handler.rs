use chrono::Duration;
use serde_json;
use serde_json::de;
use std::collections::HashMap;
use ws::{CloseCode, Handler, Handshake, Message, Result, Sender};

use amqp::Client;
use amqp::Message as AmqpMessage;
use database::{ConnectionPool, DatabaseConnection};
use models::room::Room;
use models::user::{User, UserParams};

pub type RoomsClientsMap = HashMap<i32, Vec<UserHandler>>;

pub struct UserHandler {
    pub user_id: Option<i32>,
    pub socket_handle: Sender,
    pub db_connection: ConnectionPool,
    pub amqp_client: Client,
}

#[derive(Serialize, Deserialize, Debug)]
enum ClientMessage {
    UserLogin { username: String },
    ChatSent { message_text: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ServerMessage {
    UserLoginResponse {
        user: User,
    },
    ChatMessageResponse {
        username: String,
        message_text: String,
    },
    GameStartedResponse,
}

impl UserHandler {
    pub fn handle_message(&mut self, message_data: &String) {
        let msg = de::from_str(&message_data);

        if !msg.is_ok() {
            println!("Failed to parse ClientMessage: {}", &message_data);
            return;
        }

        let msg = msg.unwrap();

        match msg {
            ClientMessage::UserLogin { username } => {
                let response = self.user_login(username).unwrap();

                match &response {
                    ServerMessage::UserLoginResponse { user } => {
                        self.user_id = Some(user.id);
                        let response = serde_json::to_string_pretty(&response).unwrap();

                        self.socket_handle.send(Message::Text(response)).unwrap();
                    }
                    _ => (),
                }
            }
            ClientMessage::ChatSent { message_text } => {
                let db_connection = DatabaseConnection(self.db_connection.get().unwrap());
                let user = User::find(&db_connection, self.user_id.unwrap()).unwrap();

                let username = user.username;
                let response = ServerMessage::ChatMessageResponse {
                    username,
                    message_text,
                };
                let response = serde_json::to_string_pretty(&response).unwrap();

                self.socket_handle
                    .broadcast(Message::Text(response))
                    .unwrap();
            }
        }
    }

    fn user_login(&self, username: String) -> Option<ServerMessage> {
        let connection = DatabaseConnection(self.db_connection.get().unwrap());
        let user_params = UserParams { username };

        let user = User::create(&connection, &user_params).unwrap();

        // Add the user to a Room:
        let room_id = Room::find_available_room_id(&connection);
        let room = Room::find(&connection, room_id).unwrap();

        if let Ok(user) = user.join_room(&connection, &room) {
            self.amqp_client.publish(
                AmqpMessage::StartGameForRoom(room.id),
                Duration::milliseconds(5 * 1000),
            );

            Some(ServerMessage::UserLoginResponse { user })
        } else {
            None
        }
    }
}

impl Handler for UserHandler {
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        println!("Hello to peer: {}", handshake.peer_addr.unwrap());

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Goodbye to peer: {:#?}, reason: {}", code, reason);
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Text(message_text) => {
                println!("{}", message_text);
                self.handle_message(&message_text);
                Ok(())
            }
            Message::Binary(_) => Ok(()),
        }
    }
}

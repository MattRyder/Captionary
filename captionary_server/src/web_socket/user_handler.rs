use serde_json;
use serde_json::de;
use ws::{CloseCode, Handler, Handshake, Message, Result, Sender};

use database::{ConnectionPool, DatabaseConnection};
use models::room::Room;
use models::user::{User, UserParams};

pub struct UserHandler {
    pub user_id: Option<i32>,
    pub socket_handle: Sender,
    pub db_connection: ConnectionPool,
}

#[derive(Serialize, Deserialize, Debug)]
enum ClientMessage {
    UserLogin { username: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ServerMessage {
    UserLoginResponse { user: User },
}

impl UserHandler {
    pub fn handle_message(&self, message_data: &String) {
        let msg: ClientMessage = de::from_str(&message_data).unwrap();

        match msg {
            ClientMessage::UserLogin { username } => {
                let db_connection = DatabaseConnection(self.db_connection.get().unwrap());
                let response = UserHandler::user_login(&db_connection, username).unwrap();
                println!("UserLogin: {}", &response);

                self.socket_handle.send(Message::Text(response));
            }
        }
    }

    fn user_login(connection: &DatabaseConnection, username: String) -> Option<String> {
        let user_params = UserParams { username };

        let user = User::create(connection, &user_params).unwrap();

        // Add the user to a Room:
        let room_id = Room::find_available_room_id(&connection);
        let room = Room::find(&connection, room_id).unwrap();

        match user.join_room(&connection, &room) {
            Ok(updated_user) => {
                let response = ServerMessage::UserLoginResponse { user: updated_user };

                Some(serde_json::to_string_pretty(&response).unwrap())
            }
            Err(_) => None,
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
                self.handle_message(&message_text);
                Ok(())
            }
            Message::Binary(_) => Ok(()),
        }
    }
}

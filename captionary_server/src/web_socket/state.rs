use std::collections::HashMap;
use serde_json::ser;
use ws::Message as WsMessage;
use web_socket::client::Client;
use web_socket::server::ServerMessage;
use models::user::{User, UserParams};
use models::vote::Vote;
use models::caption::Caption;
use models::round::Round;
use models::room::Room;
use database::{ConnectionPool, DatabaseConnection};

pub struct State {
    connection_client_map: HashMap<u32, Client>,
    room_clients_map: HashMap<i32, Vec<u32>>,
    db_connection_pool: ConnectionPool,
}

#[derive(Default)]
pub struct RoomPlayerCount {
    pub room_id: i32,
    pub player_count: i64
}

impl State {
    pub fn new(db_connection_pool: ConnectionPool) -> State {
        State {
            connection_client_map: HashMap::default(),
            room_clients_map: HashMap::default(),
            db_connection_pool,
        }
    }

    fn get_db_connection(&self) -> DatabaseConnection {
        DatabaseConnection(self.db_connection_pool.get().unwrap())
    }

    pub fn on_client_connected(&mut self, connection_id: u32, client: Client) {
        self.connection_client_map.insert(connection_id, client);
    }

    pub fn on_client_disconnected(&mut self, connection_id: &u32) {
        if let Some(client) = self.connection_client_map.remove(connection_id) {
            if let Some(client_room) = self.room_clients_map.get_mut(&client.room_id) {
                client_room.remove_item(connection_id);
            }
        }
    }

    pub fn send_message_to_client(&self, connection_id: &u32, message: &ServerMessage) {
        let response = WsMessage::Text(ser::to_string_pretty(message).unwrap());
        
        if let Some(client) = self.connection_client_map.get(connection_id) {
            client.socket_handle.send(response).unwrap();
        }
    }

    pub fn send_message_to_room(&self, room_id: &i32, message: &ServerMessage) {
        if let Some(room_connections) = self.room_clients_map.get(room_id) {
            for connection_id in room_connections {
                self.send_message_to_client(connection_id, message);
            }
        }
    }

    pub fn on_user_login(&mut self, connection_id: &u32, username: String) {
        if let Some(user) = User::create(&self.get_db_connection(), &UserParams { username }) {
            if let Some(client) = self.connection_client_map.get_mut(connection_id) {
                client.user_id = Some(user.id);
            }

            let access_token = user.update_token();
            self.send_message_to_client(
                    connection_id,
                    &ServerMessage::UserLoginResponse { access_token, user });
        }
    }

    pub fn on_join_room(&mut self, connection_id: &u32, room_name: &Option<String>) -> Option<RoomPlayerCount> {
        let connection = self.get_db_connection();

        // Find a room for the user, if given, try to join room_name:
        let room = match room_name {
            Some(room_name) => Room::find_by_name(&connection, &room_name).unwrap(),
            None => Room::find_available_room(&connection)
        };

        if let Some(client) = self.connection_client_map.get(connection_id) {
            if client.user_id.is_none() {
                return None;
            }

            let user = User::find(&connection, client.user_id.unwrap()).unwrap();
            if let Ok(user) = user.join_room(&connection, &room) {
                let self_response = ServerMessage::UserJoinedRoomResponse { 
                    access_token: user.update_token(),
                    room: room.clone() 
                };
                let other_response = ServerMessage::BroadcastUserJoinedRoom {
                    user_id: user.id,
                    username: user.username
                };

                self.send_message_to_client(&connection_id, &self_response);
                self.send_message_to_room(&room.id, &other_response);
            }
        }

        if let Some(client) = self.connection_client_map.get_mut(connection_id) {
            let room_clients = self.room_clients_map.entry(room.id).or_insert(Vec::with_capacity(32));

            client.room_id = room.id;
            room_clients.push(connection_id.clone());

            return Some(RoomPlayerCount {
                room_id: room.id,
                player_count: room.get_user_count(&connection)
            });
        }

        return None;
    }

    pub fn on_caption_submit(&self, connection_id: &u32, room_id: &i32, user_id: i32, caption_text: &String) {
        let connection = self.get_db_connection();

        // Get round for room:
        if let Ok(room) = Room::find_by_id(&connection, room_id) {
            if let Some(game) = room.get_last_game(&connection) {
                if let Some(round) = game.get_last_round(&connection) {
                    Caption::create(&connection, round.id, user_id, caption_text);
                }
            }
        }


        self.send_message_to_client(
            connection_id,
            &ServerMessage::CaptionSubmittedResponse { saved: true, errors: None })
    }

    pub fn on_submission_closed(&self, room_id: &i32, round: Round) {
        let connection = self.get_db_connection();

        if let Ok(captions) = round.get_captions(&connection) {
            self.send_message_to_room(
                room_id,
                &ServerMessage::SubmissionClosedResponse { round_id: round.id, captions }
            );
        }
    }

    pub fn on_caption_vote(&self, connection_id: &u32, caption_id: i32) {
        let connection = self.get_db_connection();

        if let Some(client) = self.connection_client_map.get(connection_id) {
            if client.user_id.is_none() {
                return;
            }

            let vote = Vote::create(&connection, client.user_id.unwrap(), caption_id).unwrap();

            self.send_message_to_client(
                connection_id,
                &ServerMessage::VoteSubmittedResponse { vote });
        }       
    }

    pub fn on_round_finished(&self, room_id: &i32, round: Round) {
        let connection = self.get_db_connection();

        if let Some(winning_caption) = round.get_winning_caption(&connection) {
            self.send_message_to_room(
                room_id,
                &ServerMessage::RoundFinishedResponse { winning_caption });
        }
    }

    pub fn on_chat_message(&self, connection_id: &u32, room_id: &i32, message_text: String) {
        let connection = self.get_db_connection();

        if let Some(client) = self.connection_client_map.get(connection_id) {
            if client.user_id.is_none() {
                return;
            }

            if let Ok(user) = User::find(&connection, client.user_id.unwrap())
            {
                let response = ServerMessage::ChatMessageResponse {
                    user_id: user.id,
                    username: user.username,
                    message_text,
                };

                self.send_message_to_room(room_id, &response);
            }
        }
        
    }
}

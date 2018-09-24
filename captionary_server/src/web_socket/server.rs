use chrono::Duration;
use std::sync::mpsc::{Receiver, Sender};
use ws;

use jwt::Token;
use amqp::Client as AmqpClient;
use amqp::Message as AmqpMessage;
use database::ConnectionPool;
use models::caption::Caption;
use models::game::Game;
use models::room::Room;
use models::round::Round;
use models::user::User;
use models::vote::Vote;
use web_socket::client::{Client, ClientMessage};
use web_socket::event::Event;
use web_socket::state::State;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ServerMessage {
    UserLoginResponse {
        access_token: String,
        user: User,
    },
    UserJoinedRoomResponse {
        access_token: String,
        room: Room,
    },
    BroadcastUserJoinedRoom {
        user_id: i32,
        username: String,
    },
    ChatMessageResponse {
        user_id: i32,
        username: String,
        message_text: String,
    },
    GameStartedResponse {
        game: Game,
    },
    RoundStartResponse {
        round: Round,
    },
    CaptionSubmittedResponse {
        saved: bool,
        errors: Option<Vec<String>>,
    },
    VoteSubmittedResponse {
        vote: Vote,
    },
    SubmissionClosedResponse {
        round_id: i32,
        captions: Vec<Caption>,
    },
    RoundFinishedResponse {
        winning_caption: Caption,
    },
}

pub struct Server {
    client_state: State,
    amqp_client: AmqpClient,
}

impl Server {
    pub fn new(db_connection_pool: ConnectionPool, amqp_client: AmqpClient) -> Server {
        let client_state = State::new(db_connection_pool);
        Server {
            client_state,
            amqp_client,
        }
    }

    pub fn handle_incoming_message(&mut self, connection_id: &u32, client_message: &ClientMessage) {
        match client_message {
            ClientMessage::UserLogin { username } => {
                self.client_state.on_user_login(connection_id, username.to_string());
            }
            ClientMessage::JoinRoom { access_token, room_name } => {
                if let Some(payload) = Token::decode(&access_token) {
                    if let Some(_uid) = payload.get("uid") {
                        if let Some(room_player_count) = self.client_state.on_join_room(connection_id, room_name)
                        {
                            if room_player_count.player_count == 2 {
                                self.amqp_client.publish(
                                    AmqpMessage::StartGameForRoom(room_player_count.room_id),
                                    Duration::milliseconds(5 * 1000),
                                );
                            }
                        }
                    }

                    
                }
                
            }
            ClientMessage::SubmitCaption { access_token, round_id,  caption_text } => {
                self.client_state
                    .on_caption_submit(connection_id, round_id.clone(), caption_text);
            }
            ClientMessage::ChatSent { access_token, message_text } => {
                if let Some(payload) = Token::decode(&access_token) {
                    if let Some(_uid) = payload.get("uid") {
                        if let Some(rid) = payload.get("rid") {
                            let room_id = rid.as_str().unwrap();
                            let room_id = room_id.parse::<i32>().unwrap();

                            self.client_state.on_chat_message(connection_id, &room_id, message_text.to_string());
                        }
                    }
                }
            }
            ClientMessage::CaptionVote { access_token, caption_id } => {
                self.client_state
                    .on_caption_vote(connection_id, caption_id.clone());
            }
        }
    }

    pub fn handle_events(&mut self, event_rx: Receiver<Event>) {
        loop {
            if let Ok(event) = event_rx.recv() {
                match event {
                    Event::OnClientConnected { client } => {
                        let connection_id = client.socket_handle.connection_id();
                        self.client_state.on_client_connected(connection_id, client);
                    }
                    Event::OnGameStart { room_id, game } => {
                        self.client_state.send_message_to_room(
                            &room_id,
                            &ServerMessage::GameStartedResponse { game },
                        );
                    }
                    Event::OnRoundStart { room_id, round } => {
                        self.client_state.send_message_to_room(
                            &room_id,
                            &ServerMessage::RoundStartResponse { round },
                        );
                    }
                    Event::OnSubmissionClosed { room_id, round } => {
                        self.client_state.on_submission_closed(&room_id, round)
                    }
                    Event::OnRoundFinished { room_id, round } => {
                        self.client_state.on_round_finished(&room_id, round);
                    }
                    Event::OnMessageSent { connection_id, client_message } => {
                        self.handle_incoming_message(&connection_id, &client_message);
                    }
                    Event::OnClientDisconnected { connection_id } => {
                        self.client_state.on_client_disconnected(&connection_id);
                    }
                };
            }
        }
    }

    pub fn connect(&mut self, websocket_host: &String, event_tx: Sender<Event>) {
        ws::listen(websocket_host, |out| {
            let cli = Client {
                user_id: None,
                room_id: 0,
                socket_handle: out,
                event_tx: event_tx.clone(),
            };

            if let Err(_) = event_tx.send(Event::OnClientConnected {
                client: cli.clone(),
            }) {
                println!("The WebSocket server isn't up!");
            }

            cli
        }).expect("WebSocket Server failed to listen for clients...");
    }
}

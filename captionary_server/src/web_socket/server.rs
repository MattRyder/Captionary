use chrono::Duration;
use database::DatabaseConnection;
use serde_json::ser;
use std::sync::mpsc::{Receiver, Sender};
use ws;
use ws::Message as WsMessage;

use amqp::Client as AmqpClient;
use amqp::Message as AmqpMessage;
use database::ConnectionPool;
use models::caption::Caption;
use models::game::Game;
use models::vote::Vote;
use models::room::Room;
use models::round::Round;
use models::user::{User, UserParams};
use web_socket::client::{Client, ClientMessage};
use web_socket::event::Event;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ServerMessage {
    UserLoginResponse {
        user: User,
    },
    UserJoinedRoomResponse {
        room: Room,
        user: User,
    },
    ChatMessageResponse {
        username: String,
        message_text: String,
    },
    GameStartedResponse {
        room_id: i32,
        game: Game,
    },
    RoundStartResponse {
        game_id: i32,
        round: Round,
    },
    CaptionSubmittedResponse {
        saved: bool,
        errors: Option<Vec<String>>,
    },
    VoteSubmittedResponse {
        vote: Vote
    },
    SubmissionClosedResponse {
        game_id: i32,
        round_id: i32,
        captions: Vec<Caption>,
    },
    RoundFinishedResponse {
        round: Round,
        winning_caption: Caption,
    },
}

pub struct Server {
    connected_clients: Vec<Client>,
    db_connection_pool: ConnectionPool,
    amqp_client: AmqpClient,
}

impl Server {
    pub fn new(db_connection_pool: ConnectionPool, amqp_client: AmqpClient) -> Server {
        Server {
            connected_clients: vec![],
            db_connection_pool,
            amqp_client,
        }
    }

    fn user_login(&self, username: String) -> Option<User> {
        let connection = DatabaseConnection(self.db_connection_pool.get().unwrap());
        let user_params = UserParams { username };

        User::create(&connection, &user_params)
    }

    /// Adds the user to a Room
    fn join_room(&self, client: &Client, room_name: Option<String>) -> Option<ServerMessage> {
        let client_idx = self
            .connected_clients
            .iter()
            .position(|c| c == client)
            .unwrap();

        let client = self.connected_clients.get(client_idx).unwrap();

        if client.user_id.is_none() {
            return None;
        }

        let connection = self.get_db_connection();

        let room = match room_name {
            Some(room_name) => {
                match Room::find(&connection, &room_name) {
                    Ok(room) => room,
                    Err(_) => Room::find_available_room(&connection)
                }
            }
            None => Room::find_available_room(&connection),
        };

        let user = User::find(&connection, client.user_id.unwrap()).unwrap();

        if let Ok(user) = user.join_room(&connection, &room) {
            let user_count = room.get_user_count(&connection);
            if user_count == 2 {
                self.amqp_client.publish(
                    AmqpMessage::StartGameForRoom(room.id),
                    Duration::milliseconds(5 * 1000),
                );
            }

            Some(ServerMessage::UserJoinedRoomResponse {
                room,
                user,
            })
        } else {
            None
        }
    }

    fn get_db_connection(&self) -> DatabaseConnection {
        DatabaseConnection(self.db_connection_pool.get().unwrap())
    }

    pub fn handle_events(&mut self, event_rx: Receiver<Event>) {
        loop {
            match event_rx.recv() {
                Ok(event) => {
                    match event {
                        Event::OnClientConnected(client) => {
                            self.connected_clients.push(client);
                        }
                        Event::OnGameStart(room_id, game) => {
                            self.send_message_to_clients(
                                &ServerMessage::GameStartedResponse { room_id, game });
                        }
                        Event::OnRoundStart(game_id, round) => {
                            self.send_message_to_clients(
                                &ServerMessage::RoundStartResponse { game_id, round })
                        }
                        Event::OnSubmissionClosed(game_id, round) => {
                            let connection = self.get_db_connection();

                            if let Ok(captions) = round.get_captions(&connection) {
                                self.send_message_to_clients(
                                    &ServerMessage::SubmissionClosedResponse { game_id, round_id: round.id, captions }
                                );
                            }
                        }
                        Event::OnRoundFinished(_game_id, round) => {
                            let connection = self.get_db_connection();

                            if let Some(winning_caption) = 
                                round.get_winning_caption(&connection)
                            {
                                self.send_message_to_clients(
                                    &ServerMessage::RoundFinishedResponse {
                                    round,
                                    winning_caption,
                                });
                            }
                        }
                        Event::OnMessageSent(client, message) => {
                            // find the actual client:
                            let client_idx = self
                                .connected_clients
                                .iter()
                                .position(|c| c == &client)
                                .unwrap();

                            match message {
                                ClientMessage::UserLogin { username } => {
                                    if let Some(user) = self.user_login(username) {
                                        {
                                            let client =
                                                self.connected_clients.get_mut(client_idx).unwrap();
                                            client.user_id = Some(user.id);

                                            println!(
                                                "UserLogin: {} logged in as User: {:?}",
                                                &user.username, client.user_id
                                            );
                                        }

                                        self.send_message_to_clients(
                                            &ServerMessage::UserLoginResponse { user });
                                    }
                                }
                                ClientMessage::JoinRoom { room_id } => {
                                    if let Some(server_message) = self.join_room(&client, room_id) {
                                        self.send_message_to_clients(&server_message)
                                    }
                                }
                                ClientMessage::SubmitCaption { round_id, caption_text } => {
                                    let connection = self.get_db_connection();
                                    let client = self.connected_clients.get(client_idx).unwrap();

                                    if let Ok(user) =
                                        User::find(&connection, client.user_id.unwrap())
                                    {
                                        let _caption = Caption::create(
                                            &connection,
                                            round_id,
                                            user.id,
                                            &caption_text,
                                        );

                                        self.send_message_to_clients(
                                            &ServerMessage::CaptionSubmittedResponse {
                                                saved: true,
                                                errors: None,
                                            }
                                        )
                                    }
                                }
                                ClientMessage::ChatSent { message_text } => {
                                    let client = self.connected_clients.get(client_idx).unwrap();
                                    let connection = self.get_db_connection();
                                    if let Ok(user) = User::find(&connection, client.user_id.unwrap())
                                    {
                                        let response = ServerMessage::ChatMessageResponse {
                                            username: user.username,
                                            message_text,
                                        };

                                        self.send_message_to_clients(&response)
                                    }
                                }
                                ClientMessage::CaptionVote { caption_id } => {
                                    let connection = self.get_db_connection();
                                    let client = self.connected_clients.get(client_idx).unwrap();

                                    let user_id = client.user_id.unwrap();
                                    let vote = Vote::create(&connection, user_id, caption_id).unwrap();

                                    self.send_message_to_clients(
                                        &ServerMessage::VoteSubmittedResponse { vote }
                                    );
                                }
                            }
                        }
                        Event::OnClientDisconnected(client) => {
                            self.connected_clients.remove_item(&client).unwrap();
                        }
                    };
                }
                Err(_) => break,
            }
        }

        println!("WebSocket Server is no longer active.");
    }

    fn send_message_to_clients(&self, server_message: &ServerMessage) {
        let response = WsMessage::Text(ser::to_string_pretty(server_message).unwrap());

        for client in &self.connected_clients {
            let response = response.clone();
            client.socket_handle.send(response).unwrap()
        }
    }

    pub fn connect(&mut self, websocket_host: &String, event_tx: Sender<Event>) {
        ws::listen(websocket_host, |out| {
            let cli = Client {
                user_id: None,
                socket_handle: out,
                event_tx: event_tx.clone(),
            };

            if let Err(_) = event_tx.send(Event::OnClientConnected(cli.clone())) {
                println!("The WebSocket server isn't up!");
            }

            cli
        }).expect("WebSocket Server failed to listen for clients...");
    }
}

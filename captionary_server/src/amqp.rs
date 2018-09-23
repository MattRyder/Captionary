use chrono::Duration;
use database::DatabaseConnection;
use futures::future::Future;
use futures::Stream;
use lapin;
use lapin::channel::{
    BasicConsumeOptions, BasicProperties, BasicPublishOptions, ExchangeDeclareOptions,
    QueueBindOptions, QueueDeclareOptions,
};
use lapin::client::ConnectionOptions;
use lapin::message::Delivery;
use lapin::types::AMQPValue::{LongLongInt, LongString};
use lapin::types::FieldTable;
use serde_json::{de, ser};
use std::net::SocketAddr;
use tokio::executor::spawn;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

use std::sync::mpsc::Sender;
use web_socket::event::Event;

use models::game::Game;
use models::round::Round;

#[derive(Clone)]
pub struct Credentials {
    pub host: SocketAddr,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    StartGameForRoom(i32),
    StartRoundForGame(i32),
    SubmissionClosed(i32),
    RoundFinished(i32),
}

#[derive(Clone)]
pub struct Client {
    credentials: Credentials,
}

impl Client {
    pub fn new(server_credentials: Credentials) -> Client {
        Client {
            credentials: server_credentials,
        }
    }

    pub fn consume(
        amqp_client: Client,
        db_connection: DatabaseConnection,
        ws_event_tx: Sender<Event>
    ) {
        let queue_name = "Game.RoundEvents";

        let client = amqp_client.clone();

        Runtime::new()
            .unwrap()
            .block_on(
                TcpStream::connect(&amqp_client.credentials.host)
                    .and_then(|stream| {
                        lapin::client::Client::connect(
                            stream,
                            ConnectionOptions {
                                username: amqp_client.credentials.username,
                                password: amqp_client.credentials.password,
                                ..Default::default()
                            },
                        )
                    }).and_then(|(client, heartbeat)| {
                        spawn(heartbeat.map_err(|_| ()));
                        client.create_channel()
                    }).and_then(move |channel| {
                        let ch = channel.clone();

                        channel
                            .queue_declare(queue_name, QueueDeclareOptions::default(), FieldTable::new()
                            ).and_then(move |queue| {
                                channel.basic_consume(&queue, "cap_consumer_1", BasicConsumeOptions::default(), FieldTable::default())
                            }).and_then(move |stream| {
                                println!("AMQP Client now consuming...");

                                stream.for_each(move |message| {
                                    Client::handle_message(&client, &message, &db_connection, &ws_event_tx);
                                    ch.basic_ack(message.delivery_tag, false)
                                })
                            })
                    }),
            ).expect("runtime error");
    }

    pub fn publish(&self, message: Message, acknowledge_in: Duration) {
        let queue_name = "Game.RoundEvents";

        println!(
            "AMQP: Publishing message {:?}, to be read in: {:?}",
            &message, acknowledge_in
        );

        let user = self.credentials.username.to_string();
        let pass = self.credentials.password.to_string();

        let mut headers = FieldTable::new();
        headers.insert(
            "x-delay".into(),
            LongLongInt(acknowledge_in.num_milliseconds()),
        );

        Runtime::new()
            .unwrap()
            .block_on(
                TcpStream::connect(&self.credentials.host)
                    .and_then(|stream| {
                        lapin::client::Client::connect(
                            stream,
                            ConnectionOptions {
                                username: user,
                                password: pass,
                                ..Default::default()
                            },
                        )
                    }).and_then(|(client, _)| client.create_channel())
                    .and_then(move |channel| {
                        channel
                            .queue_declare(
                                queue_name,
                                QueueDeclareOptions::default(),
                                FieldTable::new(),
                            ).and_then(move |_| {
                                let mut args = FieldTable::new();
                                args.insert("x-delayed-type".into(), LongString("direct".into()));

                                channel
                                    .exchange_declare(
                                        "game-exchange",
                                        "x-delayed-message",
                                        ExchangeDeclareOptions::default(),
                                        args,
                                    ).and_then(move |_| {
                                        channel
                                            .queue_bind(
                                                queue_name,
                                                "game-exchange",
                                                "captionary-rk-1",
                                                QueueBindOptions::default(),
                                                FieldTable::new(),
                                            ).and_then(
                                                move |_| {
                                                    channel.basic_publish(
                                                        "game-exchange",
                                                        "captionary-rk-1",
                                                        ser::to_vec(&message).unwrap(),
                                                        BasicPublishOptions::default(),
                                                        BasicProperties::default()
                                                            .with_headers(headers),
                                                    )
                                                },
                                            )
                                    })
                            })
                    }),
            ).expect("runtime error");
    }

    pub fn handle_message(
        client: &Client, message: &Delivery, connection: &DatabaseConnection,
        ws_event_tx: &Sender<Event>
    ) {
        let message_data = message.data.clone();
        let foo = String::from_utf8(message_data).unwrap();
        let data: Result<Message, _> = de::from_str(&foo);

        let first_game_delay_sec = 5;
        let submission_length_sec = 30;
        let voting_length_sec = 30;
        let next_round_delay_sec = 30;

        match data {
            Ok(message) => {
                match message {
                    Message::StartGameForRoom(room_id) => {
                        let game = Game::create(&connection, room_id).unwrap();

                        client.publish(
                            Message::StartRoundForGame(game.id),
                            Duration::milliseconds(first_game_delay_sec * 1000),
                        );

                        let ws_event = Event::OnGameStart { room_id, game };
                        ws_event_tx.send(ws_event).unwrap();
                    }
                    Message::StartRoundForGame(game_id) => {
                        let game = Game::find(&connection, game_id).unwrap();
                        let round = game.start_round(&connection).unwrap();

                        client.publish(
                            Message::SubmissionClosed(round.id),
                            Duration::milliseconds(submission_length_sec * 1000),
                        );

                        let ws_event = Event::OnRoundStart { room_id: game.room_id, round };
                        ws_event_tx.send(ws_event).unwrap();
                    }
                    Message::SubmissionClosed(round_id) => {
                        let round = Round::find(&connection, round_id).unwrap();
                        round.set_submission_closed(&connection);

                        client.publish(
                            Message::RoundFinished(round.id),
                            Duration::milliseconds(voting_length_sec * 1000),
                        );

                        let game = Game::find(&connection, round.game_id).unwrap();
                        let ws_event = Event::OnSubmissionClosed{ room_id: game.room_id, round };
                        ws_event_tx.send(ws_event).unwrap();
                    }
                    Message::RoundFinished(round_id) => {
                        let round = Round::find(&connection, round_id).unwrap();
                        round.set_finished(&connection);

                        let game = Game::find(&connection, round.game_id).unwrap();
                        if game.can_start_round(&connection) {
                            client.publish(
                                Message::StartRoundForGame(game.id),
                                Duration::milliseconds(next_round_delay_sec * 1000),
                            );
                        } else {
                            // Close the game off, declare a winner:
                            game.set_finished(&connection).unwrap();
                            // client.publish(
                            //     Message::StartGameForRoom(game.room_id),
                            //     Duration::milliseconds(5 * 1000),
                            // )
                        }

                        let ws_event = Event::OnRoundFinished { room_id: game.room_id, round };
                        ws_event_tx.send(ws_event).unwrap();
                    }
                }
            }
            Err(_) => (panic!("Fucked up message parsing from json")),
        }
    }
}

use futures::future::Future;
use futures::Stream;
use lapin;
use lapin::message::Delivery;
use lapin::channel::{
    BasicConsumeOptions, BasicProperties, BasicPublishOptions, ExchangeDeclareOptions,
    QueueDeclareOptions, QueueBindOptions,
};
use lapin::client::ConnectionOptions;
use lapin::types::FieldTable;
use lapin::types::AMQPValue::{LongString, LongLongInt};
use std::net::SocketAddr;
use tokio::executor::spawn;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use chrono::Duration;
use database::DatabaseConnection;
use serde_json::{ser, de};

use models::round::Round;

#[derive(Clone)]
pub struct Credentials {
    pub host: SocketAddr,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    SubmissionClosed(i32),
    RoundFinished(i32),
}

pub struct Client {
    credentials: Credentials,
}

impl Client { 
    pub fn new(server_credentials: Credentials) -> Client {
        Client { 
            credentials: server_credentials,
        } 
    } 
    pub fn consume(&self, db_connection: DatabaseConnection) {
        let queue_name = "Game.RoundEvents";

        let user = self.credentials.username.to_string();
        let pass = self.credentials.password.to_string();

        let funct = |message: &Delivery, connection: &DatabaseConnection| {
            let message_data = message.data.clone();
            let foo = String::from_utf8(message_data).unwrap();
            let data : Result<Message, _> = de::from_str(&foo);

            match data {
                Ok(message) => {
                    match message {
                        Message::SubmissionClosed(round_id) => {
                            println!("Round {}: Submission Closed", round_id);
                            let round = Round::find(&connection, round_id).unwrap();
                            round.set_submission_closed(&connection);
                        },
                        Message::RoundFinished(round_id) => {
                            println!("Round {}: Finished", round_id);
                            let round = Round::find(&connection, round_id).unwrap();
                            round.set_finished(&connection);
                        }
                    }
                },
                Err(_) => (
                    panic!("Fucked up message parsing from json")
                )
            }
        };

        Runtime::new().unwrap().block_on(
            TcpStream::connect(&self.credentials.host).and_then(|stream| {
                lapin::client::Client::connect(stream, ConnectionOptions {
                    username: user,
                    password: pass,
                    ..Default::default()
                })
            }).and_then(|(client, heartbeat)| {
                spawn(heartbeat.map_err(|_| ()));
                client.create_channel()
            }).and_then(move |channel| {
                let ch = channel.clone();

                channel.queue_declare(queue_name, QueueDeclareOptions::default(), FieldTable::new()).and_then(move |queue| {
                    channel.basic_consume(&queue, "cap_consumer_1", BasicConsumeOptions::default(), FieldTable::default())}).and_then(move |stream| {
                        println!("AMQP Client now consuming...");

                        stream.for_each(move |message| {
                            funct(&message, &db_connection);
                            ch.basic_ack(message.delivery_tag, false)
                        })
                    })
                }),
            ).expect("runtime error");
    }

    pub fn publish(&self, message: Message, acknowledge_in: Duration) {
        let queue_name = "Game.RoundEvents";

        println!("AMQP: Publishing message {:?}, to be read in: {:?}", &message, acknowledge_in);

        let user = self.credentials.username.to_string();
        let pass = self.credentials.password.to_string();

        let mut headers = FieldTable::new();
        headers.insert("x-delay".into(), LongLongInt(acknowledge_in.num_milliseconds()));

        Runtime::new().unwrap().block_on(
            TcpStream::connect(&self.credentials.host).and_then(|stream| {
                lapin::client::Client::connect(stream, ConnectionOptions {
                    username: user,
                    password: pass,
                    ..Default::default()
                })
            }).and_then(|(client, _)| client.create_channel())
            .and_then(move |channel| {
                channel.queue_declare(queue_name, QueueDeclareOptions::default(), FieldTable::new()).and_then(move |_| {
                    let mut args = FieldTable::new();
                    args.insert("x-delayed-type".into(), LongString("direct".into()));

                    channel.exchange_declare("game-exchange", "x-delayed-message", ExchangeDeclareOptions::default(), args).and_then(move |_| {
                        channel.queue_bind(queue_name, "game-exchange", "captionary-rk-1", QueueBindOptions::default(), FieldTable::new()).and_then(move |_| {
                            channel.basic_publish(
                                "game-exchange",
                                "captionary-rk-1",
                                ser::to_vec(&message).unwrap(),
                                BasicPublishOptions::default(),
                                BasicProperties::default().with_headers(headers),
                            )
                        })
                    })
                                
                })
            })
            ).expect("runtime error");
    }
}

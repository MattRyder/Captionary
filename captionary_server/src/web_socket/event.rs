use web_socket::client::{Client, ClientMessage};
use models::game::Game;
use models::round::Round;

#[derive(Debug)]
pub enum Event {
    OnClientConnected(Client),
    OnGameStart(i32, Game),
    OnRoundStart(i32, Round),
    OnSubmissionClosed(i32, Round),
    OnRoundFinished(i32, Round),
    OnMessageSent(Client, ClientMessage),
    OnClientDisconnected(Client),
}
use web_socket::client::{Client, ClientMessage};
use models::game::Game;
use models::round::Round;

#[derive(Debug)]
pub enum Event {
    OnClientConnected { client: Client },
    OnGameStart { room_id: i32, game: Game },
    OnRoundStart { room_id: i32, round: Round },
    OnSubmissionClosed { room_id: i32, round: Round },
    OnRoundFinished { room_id: i32, round: Round },
    OnMessageSent { connection_id: u32, client_message: ClientMessage },
    OnClientDisconnected { connection_id: u32 }
}
use std::fmt;
use serde_json::de;
use ws::{CloseCode, Handler, Handshake, Message, Result};
use ws::Sender as WsSender;

use std::sync::mpsc::Sender;
use web_socket::event::Event;

#[derive(Clone)]
pub struct Client {
    pub user_id: Option<i32>,
    pub room_id: i32,
    pub socket_handle: WsSender,
    pub event_tx: Sender<Event>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    UserLogin { username: String },
    JoinRoom { access_token: String, room_name: Option<String> },
    ChatSent { access_token: String, message_text: String },
    CaptionVote { access_token: String, caption_id: i32 },
    SubmitCaption { access_token: String, caption_text: String },
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client: {{ user_id: {:?}, connection_id: {:?} }}", 
                self.user_id, self.socket_handle.connection_id())
    }
}

impl PartialEq for Client {
    fn eq(&self, other: &Client) -> bool {
        self.socket_handle.connection_id() == other.socket_handle.connection_id()
    }
}

impl Handler for Client {
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        println!("Hello to peer: {}", handshake.peer_addr.unwrap());
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Goodbye to peer: {:#?}, reason: {}", code, reason);

        let message = Event::OnClientDisconnected { 
            connection_id: self.socket_handle.connection_id()
        };

        self.event_tx.send(message).unwrap();
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Message::Text(message_text) = msg {
            if let Ok(client_message) = de::from_str(&message_text) {
                let event = Event::OnMessageSent { 
                    connection_id: self.socket_handle.connection_id(),
                    client_message: client_message
                };

                self.event_tx.send(event).unwrap();
            }
        }

        Ok(())
    }
}

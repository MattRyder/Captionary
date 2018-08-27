use database::DatabaseConnection;
use rocket::http::Status;
use rocket::response::Failure;
use rocket_contrib::Json;
use serde_json::Value;
use rocket::State;
use chrono::Duration;

use amqp::{Client, Message};
use models::room::Room;
use models::user::User;
use models::game::Game;
use models::round::Round;

#[post("/<room_id>/join/<user_id>")]
fn join(
    ampq_client: State<Client>,
    connection: DatabaseConnection,
    room_id: i32,
    user_id: i32,
) -> Result<Json<Value>, Failure> {
    let mut res = false;
    let room = Room::find(&connection, room_id).unwrap();
    let user = User::find(&connection, user_id).unwrap();

    match user.join_room(&connection, &room) {
        Ok(updated_user) => {
            let game = Game::create(&connection, room.id).unwrap();
            let round = Round::create(&connection, game.id).unwrap();

            let message = Message::SubmissionClosed(round.id);
            let event_delay = Duration::milliseconds(30 * 1000);
            ampq_client.publish(message, event_delay);

            let message2 = Message::RoundFinished(round.id);
            let event_delay2 = Duration::milliseconds(60 * 1000);
            ampq_client.publish(message2, event_delay2);

            Ok(Json(json!({"user" : updated_user})))
        },
        Err(_) => Err(Failure(Status::BadRequest)),
    }
}

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

#[post("/<room_id>/join/<user_id>")]
fn join(
    amqp_client: State<Client>,
    connection: DatabaseConnection,
    room_id: i32,
    user_id: i32,
) -> Result<Json<Value>, Failure> {
    let room = Room::find(&connection, room_id).unwrap();
    let user = User::find(&connection, user_id).unwrap();

    match user.join_room(&connection, &room) {
        Ok(updated_user) => {
            amqp_client.publish(
                Message::StartGameForRoom(room.id),
                Duration::milliseconds(5 * 1000)
            );

            Ok(Json(json!({"user" : updated_user})))
        },
        Err(_) => Err(Failure(Status::BadRequest)),
    }
}

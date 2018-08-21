use database::DatabaseConnection;
use rocket::http::Status;
use rocket::response::Failure;
use rocket_contrib::Json;
use serde_json::Value;

use models::room::Room;
use models::user::User;

#[post("/<room_id>/join/<user_id>")]
fn join(
    connection: DatabaseConnection,
    room_id: i32,
    user_id: i32,
) -> Result<Json<Value>, Failure> {
    let mut res = false;
    let room = Room::find(&connection, room_id);
    let user = User::find(&connection, user_id);

    if user.is_ok() && room.is_ok() {
        let user = user.unwrap();
        let room = room.unwrap();
        res = user.join_room(&connection, &room);
    }

    match res {
        true => Ok(Json(json!({}))),
        false => Err(Failure(Status::BadRequest)),
    }
}

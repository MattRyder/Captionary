use database::DatabaseConnection;
use rocket::http::Status;
use rocket::response::Failure;
use rocket_contrib::Json;
use serde_json::Value;

use models::user::{User, UserParams};
use models::room::Room;

#[post("/", format = "application/json", data = "<user_params>")]
fn create(connection: DatabaseConnection, user_params: Json<UserParams>) -> Result<Json<Value>, Failure> {
    let user = User::create(&connection, &user_params);

    match user {
        Some(user) => {
            // Add the user to a Room:
            let room_id = Room::find_available_room_id(&connection);
            let room = Room::find(&connection, room_id).unwrap();

            match user.join_room(&connection, &room) {
                Ok(updated_user) => Ok(Json(json!({ "user" : updated_user }))),
                Err(_) => panic!("Failed to add to Room!")
            }
        },
        None => Err(Failure(Status::BadRequest)),
    }
}
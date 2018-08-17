use database::DatabaseConnection;
use rocket::http::Status;
use rocket::response::Failure;
use rocket_contrib::Json;
use serde_json::Value;

use models::user::{User, UserParams};

#[post("/", format = "application/json", data = "<user_params>")]
fn create(
    connection: DatabaseConnection,
    user_params: Json<UserParams>,
) -> Result<Json<Value>, Failure> {
    let user = User::create(&connection, &user_params);

    match user {
        Some(user) => Ok(Json(json!({ "user": user }))),
        None => Err(Failure(Status::BadRequest)),
    }
}
use database::DatabaseConnection;
use rocket_contrib::Json;
use serde_json::Value;
use rocket::response::Failure;
use rocket::http::Status;

use models::session::{Session, SessionParams};

#[post("/", format = "application/json", data = "<session_params>")]
fn create(connection: DatabaseConnection, session_params: Json<SessionParams>) -> Result<Json<Value>, Failure> {

    let session = Session::new(&session_params.username);

    match session {
        Some(session) => Ok(Json(json!({ "session" : session.token }))),
        None => Err(Failure(Status::BadRequest))
    }
}
use database::DatabaseConnection;
use rocket_contrib::Json;
use serde_json::Value;
use rocket::http::Status;
use rocket::response::Failure;

use models::caption::Caption;

#[get("/", format = "application/json")]
fn index(connection: DatabaseConnection) -> Json<Value> {
    let captions = Caption::list_all(&connection);

    Json(json!({
        "captions": captions
    }))
}

#[get("/<caption_id>", format = "application/json")]
fn show(connection: DatabaseConnection, caption_id: i32) -> Result<Json<Value>, Failure> {

    match Caption::find(&connection, caption_id) {
        Ok(caption) => Ok(Json(json!({ "caption": caption }))),
        Err(_) =>  Err(Failure(Status::NotFound))   
    }
}
use database::DatabaseConnection;
use rocket::http::Status;
use rocket::response::Failure;
use rocket_contrib::Json;
use serde_json::Value;

use diesel::SaveChangesDsl;

use models::room::Room;
use models::game::Game;
use models::round::Round;
use models::user::{User, AddToRoomParams};

#[post("/<room_id>/join/<user_id>")]
fn join(connection: DatabaseConnection, room_id: i32, user_id: i32,) -> Result<Json<Value>, Failure> {
    let mut res = false;
    let room_res = Room::find(&connection, room_id);

    match room_res {
        Ok(room) => {
            if room.can_be_joined(&connection) {
                let params = AddToRoomParams {
                    id: user_id,
                    room_id: Some(room.id),
                };
                match params.save_changes::<User>(&connection) {
                    Ok(user) => {
                        println!("Added \"{}\" to the Room: {}", user.username, room.name);
                        let game = Game::create(&connection, room.id).unwrap();
                        let round = Round::create(&connection, game.id).unwrap();

                        println!("Started Game #{} with Round #{}", round.game_id, round.id);
                        res = true;   
                    }
                    Err(error) => {
                        println!("Error: {:?}", error);
                    }
                }

                // let room_user_count = room.get_user_count(&connection);
                // if room_user_count > 3 {
                    println!("Starting Game for Room: {}", room.name);
                    
                // }
            } else {
                return Err(Failure(Status::Forbidden));
            }
        }
        Err(_) => (),
    }

    match res {
        true => Ok(Json(json!({}))),
        false => Err(Failure(Status::BadRequest)),
    }
}
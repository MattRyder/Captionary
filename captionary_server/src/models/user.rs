#![allow(proc_macro_derive_resolution_fallback)]

use std::collections::HashMap;
use chrono::NaiveDateTime;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::{QueryDsl, RunQueryDsl};
use diesel::result::Error;
use diesel::SaveChangesDsl;

use jwt::Token;
use models::room::Room;
use schema::users;

#[derive(Associations, Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[table_name = "users"]
#[belongs_to(Room)]
pub struct User {
    pub id: i32,
    pub room_id: Option<i32>,
    pub username: String,
    pub token: String,
    pub ip_address: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UserParams {
    pub username: String,
}

#[derive(AsChangeset, Identifiable)]
#[table_name = "users"]
pub struct AddToRoomParams {
    pub id: i32,
    pub room_id: Option<i32>,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a String,
    pub ip_address: &'a String,
    pub token: &'a String,
}

impl User {
    pub fn find(conn: &PgConnection, user_id: i32) -> Result<User, Error> {
        users::table.find(user_id).first::<User>(conn)
    }

    pub fn create<'a>(conn: &PgConnection, user_params: &UserParams) -> Option<User> {
        let new_user = NewUser {
            token: &String::new(),
            username: &user_params.username,
            ip_address: &("127.0.0.1".into()),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .ok()
    }

    // Uses the current User to rebuild a JWT,
    // then saves it to the DB and returns the token
    pub fn update_token(&self) -> String {

        let mut user_payload : HashMap<String, String> = HashMap::new();
        user_payload.insert("uid".to_string(), self.id.to_string());

        if self.room_id.is_some() {
            user_payload.insert("rid".to_string(), self.room_id.unwrap().to_string());
        }

        Token::encode(user_payload).unwrap()
    }

    pub fn join_room(&self, conn: &PgConnection, room: &Room) -> Result<User, Error> {
        match room.can_be_joined(&conn) {
            true => {
                let params = AddToRoomParams {
                    id: self.id,
                    room_id: Some(room.id),
                };

                params.save_changes::<User>(&conn)
            },
            false => Err(Error::NotFound)
        }
    }
}



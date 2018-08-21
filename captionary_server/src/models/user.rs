use chrono::{NaiveDateTime, Utc};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::SaveChangesDsl;
use frank_jwt::{encode, Algorithm};
use std::env;

use models::room::Room;
use schema::users;

const ENV_JWT_ISSUER: &'static str = "JWT_ISSUER";
const ENV_JWT_SECRET: &'static str = "JWT_SECRET";

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

#[derive(Serialize, Deserialize, FromForm)]
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
        let token = Self::generate_token(&user_params.username).unwrap();

        let new_user = NewUser {
            token: &token,
            username: &user_params.username,
            ip_address: &("127.0.0.1".into()),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .ok()
    }

    pub fn join_room(&self, conn: &PgConnection, room: &Room) -> bool {
        if room.can_be_joined(&conn) {
            let params = AddToRoomParams {
                id: self.id,
                room_id: Some(room.id),
            };

            return params.save_changes::<User>(&conn).is_ok();
        }

        return false;
    }

    fn generate_token(username: &String) -> Option<String> {
        if username.is_empty() {
            return None;
        }

        let jwt_secret =
            env::var(ENV_JWT_SECRET).expect(&format!("Please set env {}", ENV_JWT_SECRET));

        let jwt_issuer =
            env::var(ENV_JWT_ISSUER).expect(&format!("Please set env {}", ENV_JWT_ISSUER));

        let header = json!({
            "alg": "HS256",
            "typ": "jwt"
        });

        let payload = json!({
            "iss" : jwt_issuer,
            "iat" : Utc::now(),
            "username" : username
        });

        encode(header, &jwt_secret.to_string(), &payload, Algorithm::HS256).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn generate_token_should_create_token() {
        dotenv().ok();

        let expected_header = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";

        let username = fake!(Internet.user_name);
        let token = User::generate_token(&username);

        match token {
            Some(token_str) => {
                let expected_parts_count = 3;
                let parts: Vec<&str> = token_str.split('.').collect();
                assert_eq!(expected_parts_count, parts.len());

                let actual_header = parts.first().unwrap();
                assert_eq!(&expected_header, actual_header);
            }
            None => assert!(false),
        }
    }

    #[test]
    fn generate_token_should_return_none_for_no_username() {
        dotenv().ok();

        let username = String::new();
        let token = User::generate_token(&username);

        assert!(token.is_none());
    }
}

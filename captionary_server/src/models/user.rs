use chrono::{NaiveDateTime, Utc};
use diesel;
use diesel::pg::PgConnection;
use rocket::request::FromForm;
use diesel::prelude::*;
use frank_jwt::{encode, Algorithm};
use std::env;

use schema::users;

const ENV_JWT_ISSUER: &'static str = "JWT_ISSUER";
const ENV_JWT_SECRET: &'static str = "JWT_SECRET";

#[derive(Serialize, Deserialize, Queryable, Debug)]
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
    pub username: String
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a String,
    pub ip_address: &'a String,
    pub token: &'a String,
}

impl User {
    pub fn create<'a>(conn: &PgConnection, user_params: &UserParams) -> User {
        let token = Self::generate_token(&user_params.username).unwrap();

        let new_user = NewUser {
            token: &token,
            username: &user_params.username,
            ip_address: &("127.0.0.1".into()),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .expect("Failed to create User.")
    }

    fn generate_token(username: &String) -> Option<String> {
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

use chrono::Utc;
use frank_jwt::{decode, encode, Algorithm};
use rocket::request::FromForm;
use std::env;

const ENV_JWT_ISSUER: &'static str = "JWT_ISSUER";
const ENV_JWT_SECRET: &'static str = "JWT_SECRET";

pub struct Session {
    pub token: String,
}

#[derive(Deserialize, Debug, FromForm)]
pub struct SessionParams {
    pub username: String,
}

impl Session {
    pub fn new(username: &String) -> Option<Session> {
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

        let token = encode(header, &jwt_secret.to_string(), &payload, Algorithm::HS256);

        match token {
            Ok(token) => Some(Session { token: token }),
            Err(token_error) => None,
        }
    }
}

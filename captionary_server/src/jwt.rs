use chrono::Utc;
use chrono::DateTime;
use frank_jwt::{decode, encode, Algorithm};
use serde_json::Value;
use std::collections::HashMap;
use std::env;

const ENV_JWT_ISSUER: &'static str = "JWT_ISSUER";
const ENV_JWT_SECRET: &'static str = "JWT_SECRET";

pub struct Token {}

impl Token {
    pub fn encode(mut payload: HashMap<String, String>) -> Option<String> {
        let jwt_secret =
            env::var(ENV_JWT_SECRET).expect(&format!("Please set env {}", ENV_JWT_SECRET));

        let jwt_issuer =
            env::var(ENV_JWT_ISSUER).expect(&format!("Please set env {}", ENV_JWT_ISSUER));

        let header = json!({
            "alg": "HS256",
            "typ": "jwt"
        });

        payload.insert("iss".into(), jwt_issuer);
        payload.insert("iat".into(), Utc::now().to_rfc3339());

        let payload = json!(payload);

        println!("Encoding JWT: {:?}", &payload);

        encode(header, &jwt_secret, &payload, Algorithm::HS256).ok()
    }

    pub fn decode(token: &String) -> Option<Value> {
        let jwt_secret =
            env::var(ENV_JWT_SECRET).expect(&format!("Please set env {}", ENV_JWT_SECRET));

        match decode(token, &jwt_secret, Algorithm::HS256) {
            Ok((_header, payload)) => Some(payload),
            Err(_error) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn encode_should_create_token() {
        dotenv().ok();

        let expected_header = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";

        let mut payload = HashMap::new();
        payload.insert("uid".to_string(), "1".to_string());

        match Token::encode(payload) {
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
    fn decode_should_return_value() {
        dotenv().ok();

        let mut payload = HashMap::new();
        payload.insert("uid".to_string(), "1".to_string());

        let token = Token::encode(payload);
        assert!(token.is_some());

        let tok = Token::decode(&token.unwrap());
        assert!(tok.is_some());
        let tok = tok.unwrap();

        let expected = env::var("JWT_ISSUER").unwrap();
        let expected = expected.as_str();
        let actual = tok["iss"].as_str().unwrap();
        assert_eq!(expected, actual);

        assert!(tok["iat"].as_str().is_some());
        match DateTime::parse_from_rfc3339(tok["iat"].as_str().unwrap()) {
            Ok(iat) => assert!(iat.timestamp() <= Utc::now().timestamp()),
            Err(_) => assert!(false)
        }

        assert!(tok["uid"].as_str().is_some());
        assert_eq!(tok["uid"].as_str(), Some("1"));
    }
}

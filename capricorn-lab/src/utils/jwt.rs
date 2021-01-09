use crate::config::CONFIG;
use argon2rs::argon2i_simple;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub app: i32,
    pub user: i32,
    pub exp: i64,
}

impl Claims {
    #[allow(dead_code)]
    pub fn new(app: i32, user: i32) -> Self {
        Self {
            app,
            user,
            exp: (Utc::now() + Duration::hours(CONFIG.jwt_expiration)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
#[allow(dead_code)]
pub fn create_jwt(claim: Claims) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    encode(&Header::default(), &claim, &encoding_key)
}

/// Decode a json web token (JWT)
#[allow(dead_code)]
pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    decode::<Claims>(token, &decoding_key, &Validation::default()).map(|data| data.claims)
}

/// Encrypt a password
///
/// Uses the argon2i algorithm.
/// salt is environment-condigured.
#[allow(dead_code)]
pub fn hash(password: &str) -> String {
    argon2i_simple(&password, &CONFIG.salt)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    // static PHONE: &str = "18326069658";
    // static ID: Uuid = Uuid::new_v4();

    #[test]
    fn it_hashes_a_password() {
        let password = "password";
        let hashed = hash(password);
        assert_ne!(password, hashed);
    }

    #[test]
    fn it_matches_2_hashed_passwords() {
        let password = "password";
        let hashed = hash(password);
        let hashed_again = hash(password);
        println!("{}", hashed);
        println!("{}", hashed_again);
        assert_eq!(hashed, hashed_again);
    }

    #[test]
    fn it_creates_a_jwt() {
        // let id = Uuid::new_v4();
        // let private_claim = Claims::new(id);
        // let jwt = create_jwt(private_claim);
        // assert!(jwt.is_ok());
    }

    #[test]
    fn it_decodes_a_jwt() {
        // // let id = Uuid::new_v4();
        // let private_claim = Claims::new(id);
        // let jwt = create_jwt(private_claim.clone()).unwrap();
        // let decoded = decode_jwt(&jwt).unwrap();
        // assert_eq!(private_claim, decoded);
    }
}

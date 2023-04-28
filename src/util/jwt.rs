use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::db::entity::account::Username;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_token(
    key: &EncodingKey,
    username: &Username,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: username.to_owned().into(),
        exp: OffsetDateTime::now_utc().unix_timestamp() as usize,
    };

    encode(&Header::default(), &claims, &key)
}

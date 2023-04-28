use std::usize;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::db::entity::account::Username;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_token(
    key: &EncodingKey,
    username: &Username,
    exp_hour: i64,
) -> jsonwebtoken::errors::Result<String> {
    let time = OffsetDateTime::now_utc();
    let exp = (time + Duration::hours(exp_hour)).unix_timestamp();
    let claims = Claims {
        sub: username.to_owned().into(),
        exp: usize::try_from(exp).unwrap(),
    };
    let token = encode(&Header::default(), &claims, key)?;
    Ok(token)
}

pub fn validate_token(key: &DecodingKey, token: String) -> jsonwebtoken::errors::Result<Username> {
    let claims = decode::<Claims>(&token, key, &Validation::default())?.claims;
    Ok(Username::new(claims.sub))
}

#[cfg(test)]
mod test {
    use jsonwebtoken::{DecodingKey, EncodingKey};

    use crate::{db::entity::account::Username, util::jwt::validate_token};

    use super::create_token;

    #[test]
    fn token_test() {
        let username = Username::new("test_user".to_string());
        let key = "secret";
        let encoding_key = EncodingKey::from_secret(key.as_ref());
        let decoding_key = DecodingKey::from_secret(key.as_ref());
        let token = create_token(&encoding_key, &username, 6).unwrap();
        let validated_user = validate_token(&decoding_key, token).unwrap();

        assert_eq!(username, validated_user);

        let invalid_token = create_token(&encoding_key, &username, -1).unwrap();
        if validate_token(&decoding_key, invalid_token).is_ok() {
            panic!("invalid_token is valid token");
        }
    }
}

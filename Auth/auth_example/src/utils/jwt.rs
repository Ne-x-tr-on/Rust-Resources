use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, TokenData};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::config;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub jti: Option<String>,
    pub kind: String,
}

pub fn create_access_token(user_id: &str) -> anyhow::Result<String> {
    let exp = (Utc::now() + Duration::seconds(config::access_exp())).timestamp();
    let claims = Claims { sub: user_id.to_string(), exp, jti: None, kind: "access".into() };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(config::jwt_secret().as_bytes()))?;
    Ok(token)
}

pub fn create_refresh_token(user_id: &str, jti: &str) -> anyhow::Result<String> {
    let exp = (Utc::now() + Duration::seconds(config::refresh_exp())).timestamp();
    let claims = Claims { sub: user_id.to_string(), exp, jti: Some(jti.to_string()), kind: "refresh".into() };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(config::jwt_secret().as_bytes()))?;
    Ok(token)
}

pub fn decode_token(token: &str) -> anyhow::Result<TokenData<Claims>> {
    let data = decode::<Claims>(token, &DecodingKey::from_secret(config::jwt_secret().as_bytes()), &Validation::default())?;
    Ok(data)
}

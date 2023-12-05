use axum::http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::api::jwt_claims::Claims;

pub fn is_authorised(jwt_secret: &String, token: &str) -> Result<(), (StatusCode, String)> {
    let token_message = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    );

    let _ = match token_message {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::UNAUTHORIZED, err.to_string())),
    };

    return Ok(());
}

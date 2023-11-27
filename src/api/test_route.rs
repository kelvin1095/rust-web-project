use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json, TypedHeader,
};
use dotenv::dotenv;
use jsonwebtoken::{decode, DecodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    name: String,
    iat: i64,
    exp: i64,
}

#[derive(Deserialize)]
pub struct Summation {
    num1: String,
    num2: String,
}

#[derive(Serialize)]
struct Total {
    total: f32,
    method: String,
}

static SECRET_KEY: Lazy<String> = Lazy::new(|| {
    dotenv().ok();
    let secret = std::env::var("JWT_SECRET_KEY").expect("missing secret key");
    return secret;
});

fn is_authorised(
    authorisation_header: TypedHeader<Authorization<Bearer>>,
) -> Result<(), (StatusCode, String)> {
    let token = authorisation_header.token();

    let token_message = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET_KEY.as_bytes()),
        &Validation::default(),
    );

    let _ = match token_message {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::UNAUTHORIZED, err.to_string())),
    };

    return Ok(());
}

pub async fn list_things(
    authorisation_header: TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<Summation>,
) -> Result<String, (StatusCode, String)> {
    let _ = is_authorised(authorisation_header)?;

    let num1_parsed = match payload.num1.parse::<f32>() {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    let num2_parsed = match payload.num2.parse::<f32>() {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    let total: Total = Total {
        total: num1_parsed + num2_parsed,
        method: "Post".to_string(),
    };

    let result = serde_json::to_string_pretty(&total);

    return match result {
        Ok(result) => Ok(result),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };
}

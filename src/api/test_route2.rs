use axum::{
    http::{header::HeaderMap, StatusCode},
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
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

pub async fn list_things2(
    auth_token: TypedHeader<Authorization<Bearer>>,
    headers: HeaderMap,
    Json(payload): Json<Summation>,
) -> Result<String, (StatusCode, String)> {
    println!("{:?}", auth_token);
    println!("{:?}", headers);

    let num1_parsed = match payload.num1.parse::<f32>() {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    let num2_parsed = match payload.num2.parse::<f32>() {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
    };

    let total: Total = Total {
        total: num1_parsed * num2_parsed,
        method: "Post".to_string(),
    };

    let result = serde_json::to_string_pretty(&total);

    return match result {
        Ok(result) => Ok(result),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };
}

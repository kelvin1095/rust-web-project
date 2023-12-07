use axum::{
    http::{header::SET_COOKIE, HeaderMap, StatusCode},
    response::IntoResponse,
};

pub async fn logout() -> Result<impl IntoResponse, (StatusCode, String)> {
    let token_header = "auth-token=; Max-Age=0";

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, token_header.parse().unwrap());

    return Ok(headers);
}

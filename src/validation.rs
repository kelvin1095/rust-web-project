use axum::{
    headers::authorization::{Authorization, Bearer},
    http::StatusCode,
    TypedHeader,
};

pub fn validation_test(authorisation_header: TypedHeader<Authorization<Bearer>>) {
    let token = authorisation_header.token();

    let token_message = decode::<Claims>(&token, &SECRET_KEY.decoding, &Validation::default());

    let _token_claim = match token_message {
        Ok(result) => result,
        Err(err) => return Err((StatusCode::UNAUTHORIZED, err.to_string())),
    };

    return Ok(());
}

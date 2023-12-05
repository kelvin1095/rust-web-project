use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub iat: i64,
    pub exp: i64,
}

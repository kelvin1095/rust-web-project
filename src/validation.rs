#[derive(Deserialize, Serialize)]
struct JWT {
    sub: String,
    name: String,
    exp: usize,
}

fn validation_test() {
    let key = "secret";
    let test_jwt = JWT {
        sub: "123456789".to_string,
        name: "test_user".to_string,
        exp: 1000000000,
    }
}

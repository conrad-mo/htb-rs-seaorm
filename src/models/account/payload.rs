use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignupPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

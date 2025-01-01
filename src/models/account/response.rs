use serde::Serialize;

#[derive(Serialize)]
pub struct SignupResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
}

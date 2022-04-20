use rocket::serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct SessionRequest {
    pub email: String,
}

#[derive(Deserialize, Clone)]
pub struct AuthRequest {
    pub email: String,
    pub code: String,
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub id: u32,
    pub name: String,
}

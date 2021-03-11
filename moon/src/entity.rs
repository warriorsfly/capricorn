use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Room {
    pub id: u32,
    pub name: String,
    pub owner: u32,
}

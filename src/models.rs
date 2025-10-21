use serde::{Deserialize, Serialize};

// Example model (you can expand this later)
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

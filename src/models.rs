use serde::{Deserialize, Serialize};

//EXAMPLE MODEL YOU CAN EXPAND TS LATER
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}


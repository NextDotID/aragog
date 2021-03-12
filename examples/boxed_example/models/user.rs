use aragog::Record;
use serde::{Deserialize, Serialize};

/// This is a User
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

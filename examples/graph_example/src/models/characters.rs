use serde::{Deserialize, Serialize};
use aragog::{Record, Validate};

#[derive(Clone, Serialize, Deserialize, Record, Debug, Validate)]
pub struct Character {
    pub name: String,
    pub surname: String,
}
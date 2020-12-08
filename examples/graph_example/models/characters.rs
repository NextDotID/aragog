use aragog::{Record, Validate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Record, Debug, Validate)]
pub struct Character {
    pub name: String,
    pub surname: String,
}

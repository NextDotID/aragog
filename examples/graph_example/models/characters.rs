use aragog::Record;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Record, Debug)]
pub struct Character {
    pub name: String,
    pub surname: String,
}

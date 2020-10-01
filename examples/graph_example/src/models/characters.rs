use serde::{Deserialize, Serialize};
use aragog::{Record, Validate};

#[derive(Clone, Serialize, Deserialize, Record, Debug)]
pub struct Character {
    pub name: String,
    pub surname: String,
}

impl Validate for Character {
    fn validations(&self, _errors: &mut Vec<String>) {}
}
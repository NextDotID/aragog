use serde::{Deserialize, Serialize};
use aragog::{Record, Validate};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
   pub username: String,
   pub first_name: String,
   pub last_name: String,
   pub age: usize
}

impl Record for User {
   fn collection_name() -> String {
       String::from("Users")
   }
}

impl Validate for User {
    fn validations(&self, _errors: &mut Vec<String>) { }
}
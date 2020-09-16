use serde::{Deserialize, Serialize};
use aragog::{Record, Validate, AuthorizeAction, DatabaseRecord};
use crate::models::dish::Dish;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
   pub username: String,
   pub first_name: String,
   pub last_name: String,
   pub age: usize,
   pub is_cook: bool,
   pub money: u16,
}

impl Record for User {
   fn collection_name() -> &'static str {
       "Users"
   }
}

impl Validate for User {
    fn validations(&self, _errors: &mut Vec<String>) { }
}

pub enum DishAction {
    Order,
    Cook
}

impl AuthorizeAction<Dish> for User {
    type Action = DishAction;

    fn is_action_authorized(&self, action: Self::Action, target: Option<&DatabaseRecord<Dish>>) -> bool {
        if target.is_none() { return false; }
        let target = target.unwrap();
        match action {
            DishAction::Order => {
                if self.money < target.record.price {
                    return false
                }
                if target.record.is_alcohol {
                    return self.age >= 18
                }
                true
            },
            DishAction::Cook => self.is_cook,
        }
    }
}
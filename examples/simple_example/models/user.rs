use crate::models::dish::Dish;
use aragog::{AuthorizeAction, DatabaseRecord, Record, Validate};
use serde::{Deserialize, Serialize};

/// This is a User
#[derive(Serialize, Deserialize, Clone, Record, Validate)]
#[before_create(func("validate"))]
#[before_save(func("validate"))]
pub struct User {
    #[validate(min_length = 5, func("validate_username"))]
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    #[validate(greater_than(13), lesser_than(99))]
    pub age: usize,
    pub is_cook: bool,
    #[validate(func("validate_money"))]
    pub money: u16,
    #[validate_each(min_length = 5, max_length = 15)]
    pub roles: Vec<String>,
}

#[allow(dead_code)]
pub enum DishAction {
    Order,
    Cook,
}

impl User {
    fn validate_username(field_name: &str, field_value: &str, errors: &mut Vec<String>) {
        if field_value == "ADMIN" {
            errors.push(format!("{} can't be ADMIN", field_name))
        }
    }
    fn validate_money(_field_name: &str, field_value: &u16, errors: &mut Vec<String>) {
        if *field_value < 50 {
            errors.push("User is poor".to_string())
        }
    }
}

impl AuthorizeAction<Dish> for User {
    type Action = DishAction;

    fn is_action_authorized(
        &self,
        action: Self::Action,
        target: Option<&DatabaseRecord<Dish>>,
    ) -> bool {
        if target.is_none() {
            return false;
        }
        let target = target.unwrap();
        match action {
            DishAction::Order => {
                if self.money < target.price {
                    return false;
                }
                if target.is_alcohol {
                    return self.age >= 18;
                }
                true
            }
            DishAction::Cook => self.is_cook,
        }
    }
}

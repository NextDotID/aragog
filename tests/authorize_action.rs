use serde::{Deserialize, Serialize};
use aragog::{Record, AuthorizeAction, DatabaseRecord, Validate};

pub mod common;

#[derive(Serialize, Deserialize, Clone)]
pub struct Dish {
    pub name: String,
    pub price: u16,
    pub is_alcohol: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub age: u8,
    pub money: u16,
    pub is_cook: bool,
}

impl Record for Dish {
    fn collection_name() -> &'static str { "Dishes" }
}

impl Validate for Dish {
    fn validations(&self, _errors: &mut Vec<String>) {}
}

pub enum DishAction {
    Order,
    Cook
}

impl AuthorizeAction<Dish> for User {
    type Action = DishAction;

    fn is_action_authorized(&self, action: Self::Action, target: &DatabaseRecord<Dish>) -> bool {
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

#[test]
fn can_authorize() -> Result<(), String> {
    common::with_db(|pool| {
        let dish = Dish {
            name: "Pizza".to_string(),
            price: 10,
            is_alcohol: false
        };
        let dish_record = tokio_test::block_on(DatabaseRecord::create(dish, pool)).unwrap();
        let user = User {
            name: "Kid".to_string(),
            age: 15,
            money: 11,
            is_cook: false
        };
        common::expect_assert(user.is_action_authorized(DishAction::Order, &dish_record))?;
        common::expect_assert(user.authorize_action(DishAction::Order, &dish_record).is_ok())?;
        Ok(())
    })
}


#[test]
fn can_fail() -> Result<(), String> {
    common::with_db(|pool| {
        let dish = Dish {
            name: "Forêt noire".to_string(),
            price: 10,
            is_alcohol: true
        };
        let dish_record = tokio_test::block_on(DatabaseRecord::create(dish, pool)).unwrap();

        // Not enough money and not cook
        let poor_user = User {
            name: "PoorAdult".to_string(),
            age: 18,
            money: 5,
            is_cook: false
        };
        common::expect_assert(!poor_user.is_action_authorized(DishAction::Order, &dish_record))?;
        common::expect_assert(poor_user.authorize_action(DishAction::Order, &dish_record).is_err())?;
        common::expect_assert(!poor_user.is_action_authorized(DishAction::Cook, &dish_record))?;
        common::expect_assert(poor_user.authorize_action(DishAction::Cook, &dish_record).is_err())?;

        // Not old enough and not cook
        let poor_user = User {
            name: "Kid".to_string(),
            age: 15,
            money: 15,
            is_cook: false
        };
        common::expect_assert(!poor_user.is_action_authorized(DishAction::Order, &dish_record))?;
        common::expect_assert(poor_user.authorize_action(DishAction::Order, &dish_record).is_err())?;
        common::expect_assert(!poor_user.is_action_authorized(DishAction::Cook, &dish_record))?;
        common::expect_assert(poor_user.authorize_action(DishAction::Cook, &dish_record).is_err())?;

        // Not old enough but is cook
        let poor_user = User {
            name: "Kid".to_string(),
            age: 15,
            money: 15,
            is_cook: true
        };
        common::expect_assert(!poor_user.is_action_authorized(DishAction::Order, &dish_record))?;
        common::expect_assert(poor_user.authorize_action(DishAction::Order, &dish_record).is_err())?;
        common::expect_assert(poor_user.is_action_authorized(DishAction::Cook, &dish_record))?;
        common::expect_assert(poor_user.authorize_action(DishAction::Cook, &dish_record).is_ok())?;
        Ok(())
    })
}
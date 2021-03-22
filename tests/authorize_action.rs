#![cfg(not(feature = "minimal_traits"))]

use serde::{Deserialize, Serialize};

use aragog::{AuthorizeAction, DatabaseRecord, Record};

pub mod common;

#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Dish {
    pub name: String,
    pub price: u16,
    pub is_alcohol: bool,
}

#[derive(Serialize, Deserialize, Clone, Record)]
pub struct User {
    pub name: String,
    pub age: u8,
    pub money: u16,
    pub is_cook: bool,
}

pub enum DishAction {
    Order,
    Cook,
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

#[maybe_async::test(
    any(feature = "blocking"),
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn can_authorize() -> Result<(), String> {
    let connection = common::setup_db().await;
    let dish = Dish {
        name: "Pizza".to_string(),
        price: 10,
        is_alcohol: false,
    };
    let dish_record = DatabaseRecord::create(dish, &connection).await.unwrap();
    let user = User {
        name: "Kid".to_string(),
        age: 15,
        money: 11,
        is_cook: false,
    };
    common::expect_assert(user.is_action_authorized(DishAction::Order, Some(&dish_record)))?;
    common::expect_assert(
        user.authorize_action(DishAction::Order, Some(&dish_record))
            .is_ok(),
    )?;
    Ok(())
}

#[maybe_async::test(
    any(feature = "blocking"),
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn can_fail() -> Result<(), String> {
    let connection = common::setup_db().await;
    let dish = Dish {
        name: "Forêt noire".to_string(),
        price: 10,
        is_alcohol: true,
    };
    let dish_record = DatabaseRecord::create(dish, &connection).await.unwrap();

    // Not enough money and not cook
    let poor_user = User {
        name: "PoorAdult".to_string(),
        age: 18,
        money: 5,
        is_cook: false,
    };
    common::expect_assert(!poor_user.is_action_authorized(DishAction::Order, Some(&dish_record)))?;
    common::expect_assert(
        poor_user
            .authorize_action(DishAction::Order, Some(&dish_record))
            .is_err(),
    )?;
    common::expect_assert(!poor_user.is_action_authorized(DishAction::Cook, Some(&dish_record)))?;
    common::expect_assert(
        poor_user
            .authorize_action(DishAction::Cook, Some(&dish_record))
            .is_err(),
    )?;

    // Not old enough and not cook
    let poor_user = User {
        name: "Kid".to_string(),
        age: 15,
        money: 15,
        is_cook: false,
    };
    common::expect_assert(!poor_user.is_action_authorized(DishAction::Order, Some(&dish_record)))?;
    common::expect_assert(
        poor_user
            .authorize_action(DishAction::Order, Some(&dish_record))
            .is_err(),
    )?;
    common::expect_assert(!poor_user.is_action_authorized(DishAction::Cook, Some(&dish_record)))?;
    common::expect_assert(
        poor_user
            .authorize_action(DishAction::Cook, Some(&dish_record))
            .is_err(),
    )?;

    // Not old enough but is cook
    let poor_user = User {
        name: "Kid".to_string(),
        age: 15,
        money: 15,
        is_cook: true,
    };
    common::expect_assert(!poor_user.is_action_authorized(DishAction::Order, Some(&dish_record)))?;
    common::expect_assert(
        poor_user
            .authorize_action(DishAction::Order, Some(&dish_record))
            .is_err(),
    )?;
    common::expect_assert(poor_user.is_action_authorized(DishAction::Cook, Some(&dish_record)))?;
    common::expect_assert(
        poor_user
            .authorize_action(DishAction::Cook, Some(&dish_record))
            .is_ok(),
    )?;
    Ok(())
}

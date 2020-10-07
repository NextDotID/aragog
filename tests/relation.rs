use serde::{Deserialize, Serialize};

use aragog::{DatabaseRecord, Record, Link, Validate, ForeignLink};
use aragog::query::{Comparison, Query, RecordQueryResult};
use std::borrow::Borrow;

mod common;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Record, Validate)]
pub struct Dish {
    pub name: String,
    pub description: String,
    pub price: u16,
    pub order_id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Record, Validate)]
pub struct Order {
    pub name: String,
}

impl Link<Order> for Dish {
    fn link_query(&self) -> Query {
        Order::query().filter(Comparison::field("_key").equals_str(&self.order_id).into())
    }
}

impl ForeignLink<Order> for Dish {
    fn foreign_key(&self) -> &str {
        self.order_id.borrow()
    }
}


impl Link<Dish> for Dish {
    fn link_query(&self) -> Query {
        Dish::query().filter(Comparison::field("order_id").equals_str(&self.order_id).into())
    }
}

#[test]
fn relation_work() -> Result<(), String> {
    common::with_db(|pool| {
        let order = tokio_test::block_on(DatabaseRecord::create(Order {
            name: "Test".to_string()
        }, pool)).unwrap();
        let dish = tokio_test::block_on(DatabaseRecord::create(Dish {
            name: "DishTest".to_string(),
            description: "relation Test".to_string(),
            price: 10,
            order_id: order.key.clone(),
        }, pool)).unwrap();

        let relation: RecordQueryResult<Order> = tokio_test::block_on(dish.record.linked_models(&pool)).unwrap();
        common::expect_assert_eq(&relation.uniq().unwrap().key, &order.key)?;
        Ok(())
    })
}

#[test]
fn foreign_key_relation_work() -> Result<(), String> {
    common::with_db(|pool| {
        let order = tokio_test::block_on(DatabaseRecord::create(Order {
            name: "Test".to_string()
        }, pool)).unwrap();
        let dish = tokio_test::block_on(DatabaseRecord::create(Dish {
            name: "DishTest".to_string(),
            description: "relation Test".to_string(),
            price: 10,
            order_id: order.key.clone(),
        }, pool)).unwrap();

        let relation: DatabaseRecord<Order> = tokio_test::block_on(dish.record.linked_model(&pool)).unwrap();
        common::expect_assert_eq(&relation.key, &order.key)?;
        Ok(())
    })
}
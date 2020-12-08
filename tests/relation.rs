use serde::{Deserialize, Serialize};

use aragog::query::{Comparison, Query, RecordQueryResult};
use aragog::{DatabaseRecord, ForeignLink, Link, Record, Validate};
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
        Dish::query().filter(
            Comparison::field("order_id")
                .equals_str(&self.order_id)
                .into(),
        )
    }
}

#[maybe_async::test(
    any(feature = "blocking"),
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn relation_work() -> Result<(), String> {
    common::with_db(|pool| async move {
        let order = DatabaseRecord::create(
            Order {
                name: "Test".to_string(),
            },
            &pool,
        )
        .await
        .unwrap();
        let dish = DatabaseRecord::create(
            Dish {
                name: "DishTest".to_string(),
                description: "relation Test".to_string(),
                price: 10,
                order_id: order.key.clone(),
            },
            &pool,
        )
        .await
        .unwrap();

        let relation: RecordQueryResult<Order> = dish.record.linked_models(&pool).await.unwrap();
        common::expect_assert_eq(&relation.uniq().unwrap().key, &order.key)?;
        Ok(())
    })
    .await
}

#[maybe_async::test(
    any(feature = "blocking"),
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn foreign_key_relation_work() -> Result<(), String> {
    common::with_db(|pool| async move {
        let order = DatabaseRecord::create(
            Order {
                name: "Test".to_string(),
            },
            &pool,
        )
        .await
        .unwrap();
        let dish = DatabaseRecord::create(
            Dish {
                name: "DishTest".to_string(),
                description: "relation Test".to_string(),
                price: 10,
                order_id: order.key.clone(),
            },
            &pool,
        )
        .await
        .unwrap();

        let relation: DatabaseRecord<Order> = dish.record.linked_model(&pool).await.unwrap();
        common::expect_assert_eq(&relation.key, &order.key)?;
        Ok(())
    })
    .await
}

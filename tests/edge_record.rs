use serde::{Deserialize, Serialize};

use aragog::{DatabaseConnectionPool, DatabaseRecord, EdgeRecord, Record, Validate};

mod common;

#[derive(Clone, Serialize, Deserialize, Record, Validate)]
pub struct Dish {
    pub name: String
}

#[derive(Clone, Serialize, Deserialize, Record, Validate)]
pub struct Order {
    pub name: String
}

fn create_dish(pool: &DatabaseConnectionPool) -> DatabaseRecord<Dish> {
    tokio_test::block_on(DatabaseRecord::create(
        Dish {
            name: "Pizza Mozarella".to_string()
        }, pool)).unwrap()
}

fn create_order(pool: &DatabaseConnectionPool) -> DatabaseRecord<Order> {
    tokio_test::block_on(DatabaseRecord::create(
        Order {
            name: "Menu Pizza".to_string()
        }, pool)).unwrap()
}


#[derive(Clone, EdgeRecord, Validate, Serialize, Deserialize)]
pub struct PartOf {
    _from: String,
    _to: String,
    description: String,
}

#[test]
fn edge_can_be_created() -> Result<(), String> {
    common::with_db(|pool| {
        let dish = create_dish(&pool);
        let order = create_order(&pool);

        tokio_test::block_on(DatabaseRecord::create(
            PartOf {
                _from: dish.id.clone(),
                _to: order.id.clone(),
                description: "part of".to_string(),
            }
            , &pool)).unwrap();
        Ok(())
    })
}

#[test]
fn edge_can_be_created_with_a_simple_link() -> Result<(), String> {
    common::with_db(|pool| {
        let dish = create_dish(&pool);
        let order = create_order(&pool);

        let record = tokio_test::block_on(
            DatabaseRecord::link(&dish, &order, &pool, |_from, _to| {
                PartOf { _from, _to, description: "Test".to_string() }
            })
        ).unwrap();
        common::expect_assert_eq(&record.record._from, &dish.id)?;
        common::expect_assert_eq(&record.record._to, &order.id)?;
        common::expect_assert_eq(record.record._from_collection_name().as_str(), Dish::collection_name())?;
        common::expect_assert_eq(record.record._to_collection_name().as_str(), Order::collection_name())?;
        Ok(())
    })
}

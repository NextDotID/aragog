use serde::{Deserialize, Serialize};

use aragog::{DatabaseConnectionPool, DatabaseRecord, EdgeRecord, Record, Validate};

mod common;

#[derive(Clone, Serialize, Deserialize, Record, Validate)]
pub struct Dish {
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Record, Validate)]
pub struct Order {
    pub name: String,
}

#[maybe_async::maybe_async]
async fn create_dish(pool: &DatabaseConnectionPool) -> DatabaseRecord<Dish> {
    DatabaseRecord::create(
        Dish {
            name: "Pizza Mozarella".to_string(),
        },
        pool,
    )
    .await
    .unwrap()
}

#[maybe_async::maybe_async]
async fn create_order(pool: &DatabaseConnectionPool) -> DatabaseRecord<Order> {
    DatabaseRecord::create(
        Order {
            name: "Menu Pizza".to_string(),
        },
        pool,
    )
    .await
    .unwrap()
}

#[derive(Clone, EdgeRecord, Serialize, Deserialize)]
pub struct PartOf {
    _from: String,
    _to: String,
    description: String,
}

impl Validate for PartOf {
    fn validations(&self, errors: &mut Vec<String>) {
        self.validate_edge_fields(errors);
    }
}

#[maybe_async::test(
    any(feature = "blocking"),
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn edge_can_be_created() -> Result<(), String> {
    let pool = common::setup_db().await;
    let dish = create_dish(&pool).await;
    let order = create_order(&pool).await;

    DatabaseRecord::create(
        PartOf {
            _from: dish.id.clone(),
            _to: order.id.clone(),
            description: "part of".to_string(),
        },
        &pool,
    )
    .await
    .unwrap();
    Ok(())
}

#[maybe_async::test(
    any(feature = "blocking"),
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn edge_can_be_created_with_a_simple_link() -> Result<(), String> {
    let pool = common::setup_db().await;
    let dish = create_dish(&pool).await;
    let order = create_order(&pool).await;

    let record = DatabaseRecord::link(&dish, &order, &pool, |_from, _to| PartOf {
        _from,
        _to,
        description: "Test".to_string(),
    })
    .await
    .unwrap();
    common::expect_assert_eq(&record.record._from, &dish.id)?;
    common::expect_assert_eq(&record.record._to, &order.id)?;
    common::expect_assert_eq(
        record.record._from_collection_name().as_str(),
        Dish::collection_name(),
    )?;
    common::expect_assert_eq(
        record.record._to_collection_name().as_str(),
        Order::collection_name(),
    )?;
    Ok(())
}

#[test]
fn edge_validated_format() -> Result<(), String> {
    let edge = PartOf {
        _from: "Dish/123".to_string(),
        _to: "Dish/234".to_string(),
        description: "part of".to_string(),
    };
    common::expect_assert(edge.is_valid())?;
    let edge = PartOf {
        _from: "Dish/".to_string(),
        _to: "Dish/234".to_string(),
        description: "part of".to_string(),
    };
    common::expect_assert(!edge.is_valid())?;
    let edge = PartOf {
        _from: "Dish//123".to_string(),
        _to: "Dish/234".to_string(),
        description: "part of".to_string(),
    };
    common::expect_assert(!edge.is_valid())?;
    let edge = PartOf {
        _from: "Dish/dish".to_string(),
        _to: "Dish/234".to_string(),
        description: "part of".to_string(),
    };
    common::expect_assert(!edge.is_valid())?;
    let edge = PartOf {
        _from: "/123".to_string(),
        _to: "Dish/234".to_string(),
        description: "part of".to_string(),
    };
    common::expect_assert(!edge.is_valid())?;
    Ok(())
}

use serde::{Deserialize, Serialize};

use aragog::{DatabaseConnection, DatabaseRecord, EdgeRecord, Record, ServiceError, Validate};

mod common;

#[derive(Clone, Serialize, Deserialize, Record)]
pub struct Dish {
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Record)]
pub struct Order {
    pub name: String,
}

#[maybe_async::maybe_async]
async fn create_dish(connection: &DatabaseConnection) -> DatabaseRecord<Dish> {
    DatabaseRecord::create(
        Dish {
            name: "Pizza Mozarella".to_string(),
        },
        connection,
    )
    .await
    .unwrap()
}

#[maybe_async::maybe_async]
async fn create_order(connection: &DatabaseConnection) -> DatabaseRecord<Order> {
    DatabaseRecord::create(
        Order {
            name: "Menu Pizza".to_string(),
        },
        connection,
    )
    .await
    .unwrap()
}

#[derive(Clone, Record, EdgeRecord, Serialize, Deserialize, Validate)]
#[validate(func("validate_edge_fields"))]
#[before_write(func = "validate")]
pub struct PartOf {
    _from: String,
    _to: String,
    description: String,
}

#[maybe_async::test(
    any(feature = "blocking"),
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn edge_can_be_created() -> Result<(), String> {
    let connection = common::setup_db().await;
    let dish = create_dish(&connection).await;
    let order = create_order(&connection).await;

    DatabaseRecord::create(
        PartOf {
            _from: dish.id().clone(),
            _to: order.id().clone(),
            description: "part of".to_string(),
        },
        &connection,
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
    let connection = common::setup_db().await;
    let dish = create_dish(&connection).await;
    let order = create_order(&connection).await;

    let record = DatabaseRecord::link(&dish, &order, &connection, |_from, _to| PartOf {
        _from,
        _to,
        description: "Test".to_string(),
    })
    .await
    .unwrap();
    common::expect_assert_eq(&record._from, dish.id())?;
    common::expect_assert_eq(&record._to, order.id())?;
    common::expect_assert_eq(
        record._from_collection_name().as_str(),
        Dish::collection_name(),
    )?;
    common::expect_assert_eq(
        record._to_collection_name().as_str(),
        Order::collection_name(),
    )?;
    Ok(())
}

#[maybe_async::test(
    any(feature = "blocking"),
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn link_launches_hooks() -> Result<(), String> {
    let connection = common::setup_db().await;
    let dish = create_dish(&connection).await;
    let order = create_order(&connection).await;

    let res = DatabaseRecord::link(&dish, &order, &connection, |_from, _to| PartOf {
        _from: "123".to_string(),
        _to,
        description: "Test".to_string(),
    })
    .await;
    match res {
        Ok(_) => Err(String::from("Validations should have failed")),
        Err(error) => match error {
            ServiceError::ValidationError(msg) => {
                common::expect_assert_eq(msg, r#"_from "123" has wrong format"#.to_string())
            }
            _ => Err(String::from("Wrong error returned")),
        },
    }
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

use serde::{Deserialize, Serialize};

use aragog::{DatabaseConnection, DatabaseRecord, EdgeRecord, Error, Record, Validate};

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

#[derive(Clone, Record, Serialize, Deserialize, Validate)]
#[before_write(func = "validate")]
pub struct PartOf {
    #[validate(min_length = 5)]
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

    let edge = EdgeRecord::new(
        dish.id().clone(),
        order.id().clone(),
        PartOf {
            description: "part of".to_string(),
        },
    )
    .unwrap();
    assert_eq!(edge.id_from(), dish.id());
    assert_eq!(edge.id_to(), order.id());
    assert_eq!(edge.description, "part of".to_string());
    let edge = DatabaseRecord::create(edge, &connection).await.unwrap();
    assert_eq!(edge.id_from(), dish.id());
    assert_eq!(edge.id_to(), order.id());
    assert_eq!(edge.description, "part of".to_string());

    let record: DatabaseRecord<EdgeRecord<PartOf>> =
        DatabaseRecord::find(edge.key(), &connection).await.unwrap();
    assert_eq!(record.key(), edge.key());
    Ok(())
}

#[maybe_async::test(
    any(feature = "blocking"),
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn edge_can_be_serialized() -> Result<(), String> {
    #[derive(Serialize, Deserialize)]
    struct SerializedPartOf {
        pub _key: String,
        pub _rev: String,
        pub _id: String,
        pub _from: String,
        pub _to: String,
        pub description: String,
    }

    let connection = common::setup_db().await;
    let dish = create_dish(&connection).await;
    let order = create_order(&connection).await;

    let edge = EdgeRecord::new(
        dish.id().clone(),
        order.id().clone(),
        PartOf {
            description: "part of".to_string(),
        },
    )
    .unwrap();
    let edge = DatabaseRecord::create(edge, &connection).await.unwrap();
    let json = serde_json::to_string(&edge).unwrap();
    let serialized: SerializedPartOf = serde_json::from_str(&json).unwrap();
    assert_eq!(&serialized._key, edge.key());
    assert_eq!(&serialized._rev, edge.rev());
    assert_eq!(&serialized._id, edge.id());
    assert_eq!(&serialized._from, edge.id_from());
    assert_eq!(&serialized._to, edge.id_to());
    assert_eq!(serialized.description, edge.description);
    let record: DatabaseRecord<EdgeRecord<PartOf>> = serde_json::from_str(&json).unwrap();
    assert_eq!(record.key(), edge.key());
    assert_eq!(record.rev(), edge.rev());
    assert_eq!(record.id(), edge.id());
    assert_eq!(record.id_from(), edge.id_from());
    assert_eq!(record.id_to(), edge.id_to());
    assert_eq!(record.description, edge.description);

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

    let record = DatabaseRecord::link(
        &dish,
        &order,
        &connection,
        PartOf {
            description: "Correct".to_string(),
        },
    )
    .await
    .unwrap();
    common::expect_assert_eq(record.id_from(), dish.id())?;
    common::expect_assert_eq(record.id_to(), order.id())?;
    common::expect_assert_eq(record.from_collection_name(), Dish::COLLECTION_NAME)?;
    common::expect_assert_eq(record.to_collection_name().as_str(), Order::COLLECTION_NAME)?;
    let from: DatabaseRecord<Dish> = record.from_record(&connection).await.unwrap();
    assert_eq!(from.id(), record.id_from());
    let to: DatabaseRecord<Order> = record.to_record(&connection).await.unwrap();
    assert_eq!(to.id(), record.id_to());
    Ok(())
}

#[maybe_async::test(
    any(feature = "blocking"),
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn retrieval_fails_on_non_edges() {
    let connection = common::setup_db().await;
    let dish = create_dish(&connection).await;

    // This works
    let rec = Dish::find(dish.key(), &connection).await;
    assert!(rec.is_ok());

    // This fails
    let rec = EdgeRecord::<Dish>::find(dish.key(), &connection).await;
    assert!(rec.is_err());
}

#[maybe_async::test(
    any(feature = "blocking"),
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn link_launches_hooks() -> Result<(), String> {
    let connection = common::setup_db().await;
    let dish = create_dish(&connection).await;
    let order = create_order(&connection).await;

    let res = DatabaseRecord::link(
        &dish,
        &order,
        &connection,
        PartOf {
            description: "Test".to_string(),
        },
    )
    .await;
    match res {
        Ok(_) => Err(String::from("Validations should have failed")),
        Err(error) => match error {
            Error::ValidationError(msg) => common::expect_assert_eq(
                msg,
                r#"description 'Test' is too short, min length: 5"#.to_string(),
            ),
            _ => Err(String::from("Wrong error returned")),
        },
    }
}

#[test]
fn edge_validated_format() -> Result<(), String> {
    let edge = EdgeRecord::new(
        "Dish/123".to_string(),
        "Dish/234".to_string(),
        PartOf {
            description: "part of".to_string(),
        },
    );
    assert!(edge.is_ok());
    let edge = EdgeRecord::new(
        "Dish/".to_string(),
        "Dish/234".to_string(),
        PartOf {
            description: "part of".to_string(),
        },
    );
    assert!(edge.is_err());
    let edge = EdgeRecord::new(
        "Dish//123".to_string(),
        "Dish/234".to_string(),
        PartOf {
            description: "part of".to_string(),
        },
    );
    assert!(edge.is_err());
    let edge = EdgeRecord::new(
        "Dish/custom_key".to_string(),
        "Dish/234".to_string(),
        PartOf {
            description: "part of".to_string(),
        },
    );
    assert!(edge.is_ok());
    let edge = EdgeRecord::new(
        "/123".to_string(),
        "Dish/234".to_string(),
        PartOf {
            description: "part of".to_string(),
        },
    );
    assert!(edge.is_err());
    Ok(())
}

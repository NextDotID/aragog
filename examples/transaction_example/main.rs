extern crate env_logger;

use aragog::transaction::Transaction;
use aragog::{AuthMode, DatabaseConnectionPool, DatabaseRecord, New, ServiceError, Update};

use crate::models::dish::{Dish, DishDTO};
use crate::models::order::Order;

mod models;

const DEFAULT_DB_HOST: &str = "http://localhost:8529";
const DEFAULT_DB_NAME: &str = "aragog_test";
const DEFAULT_DB_USER: &str = "test";
const DEFAULT_DB_PWD: &str = "test";

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info,aragog=trace");
    env_logger::init();

    // Connect to database and generates collections and indexes
    let db_pool = DatabaseConnectionPool::builder()
        .with_credentials(
            &std::env::var("DB_HOST").unwrap_or(DEFAULT_DB_HOST.to_string()),
            &std::env::var("DB_NAME").unwrap_or(DEFAULT_DB_NAME.to_string()),
            &std::env::var("DB_USER").unwrap_or(DEFAULT_DB_USER.to_string()),
            &std::env::var("DB_PWD").unwrap_or(DEFAULT_DB_PWD.to_string()),
        )
        .with_auth_mode(AuthMode::default())
        .with_schema_path("examples/transaction_example/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();

    // Testing purposes
    db_pool.truncate().await;

    // Instantiate a new dish
    let dish = Dish::new(DishDTO {
        name: "Pizza Regina".to_string(),
        description: "Tomato base, Ham, Mozzarella, egg".to_string(),
        price: 10,
        is_alcohol: false,
    })
    .unwrap();
    // New empty order
    let mut order = Order::new();

    let transaction = Transaction::new(&db_pool).await.unwrap();
    let transaction_output = transaction
        .safe_execute(|pool| async move {
            // Creates a database record
            let mut dish_record = DatabaseRecord::create(dish, &pool).await?;
            // Add a dish
            order.add(&dish_record.record);
            // Creates a database record
            let mut order_record = DatabaseRecord::create(order, &pool).await?;
            // Update dish
            dish_record
                .record
                .update(&DishDTO {
                    name: "Pizza Mozzarella".to_string(),
                    description: "Tomato base, Mozzarella".to_string(),
                    price: 7,
                    is_alcohol: false,
                })
                .unwrap();
            // Add the updated dish to the order
            order_record.record.add(&dish_record.record);
            // Save the order record
            order_record.save(&pool).await?;

            assert_eq!(order_record.record.dishes.len(), 2);
            assert_eq!(order_record.record.total_price, 17);
            Ok(dish_record)
        })
        .await
        .unwrap();

    // We check the transaction succeeded
    assert!(transaction_output.is_committed());
    println!("Transaction committed");

    // Checking
    assert_eq!(db_pool.collections["Dish"].record_count().await.unwrap(), 1);

    let mut dish_record = transaction_output.unwrap();

    let transaction = Transaction::new(&db_pool).await.unwrap();
    let transaction_output = transaction
        .safe_execute(|pool| async move {
            // Making validation fail
            dish_record.record.price = 0;
            dish_record.save(&pool).await?;
            Ok(())
        })
        .await
        .unwrap();

    // We check the transaction failed
    assert!(transaction_output.is_aborted());
    println!("Transaction aborted");

    let error = transaction_output.err().unwrap();

    match error {
        ServiceError::ValidationError(msg) => {
            assert_eq!(msg, String::from("price should be above zero"))
        }
        _ => panic!("Wrong error returned"),
    }
    println!("Done!")
}

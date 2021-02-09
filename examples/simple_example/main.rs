#[macro_use]
extern crate aragog;
extern crate env_logger;

use aragog::query::{Comparison, Filter};
use aragog::{
    AuthMode, DatabaseConnectionPool, DatabaseRecord, New, Record, ServiceError, Update, Validate,
};

use crate::models::dish::{Dish, DishDTO};
use crate::models::order::Order;
use crate::models::user::User;

mod models;

const DEFAULT_DB_HOST: &str = "http://localhost:8529";
const DEFAULT_DB_NAME: &str = "aragog_test";
const DEFAULT_DB_USER: &str = "test";
const DEFAULT_DB_PWD: &str = "test";

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info,aragog=debug");
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
        .with_schema_path("examples/simple_example/schema.yaml")
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
    // Creates a database record
    let mut dish_record = DatabaseRecord::create(dish, &db_pool).await.unwrap();

    // New empty order
    let mut order = Order::new();
    // An empty order is not valid
    assert!(!order.is_valid());
    // Add a dish
    order.add(&dish_record.record);
    // Creates a database record
    let mut order_record = DatabaseRecord::create(order, &db_pool).await.unwrap();
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
    order_record.save(&db_pool).await.unwrap();

    // Checking
    assert_eq!(order_record.record.dishes.len(), 2);
    assert_eq!(order_record.record.total_price, 17);
    assert_eq!(db_pool.collections["Dish"].record_count().await.unwrap(), 1);

    // Making validation fail
    dish_record.record.price = 0;
    match dish_record.save(&db_pool).await {
        Ok(()) => panic!("Validations should have failed"),
        Err(error) => match error {
            ServiceError::ValidationError(msg) => {
                assert_eq!(msg, String::from("price '0' must be greater than 0"))
            }
            _ => panic!("Wrong error returned"),
        },
    }

    // Query examples

    let user = User {
        username: String::from("LeRevenant1234"),
        first_name: String::from("Robert"),
        last_name: String::from("Surcouf"),
        age: 18,
        is_cook: false,
        money: 100,
    };
    let record = DatabaseRecord::create(user, &db_pool).await.unwrap();

    // Find with the primary key
    let _user_record = User::find(&record.key, &db_pool).await.unwrap();

    // Build a query
    let query = User::query().filter(
        Filter::new(compare!(field "last_name").equals_str("Surcouf"))
            .and(Comparison::field("age").greater_than(15)),
    );
    let query_b = query.clone();
    // Call the query and get safe JSON results to parse
    let json_result = query.call(&db_pool).await.unwrap();
    let _user_results = json_result.get_records::<User>();
    // OR Retrieve only the USer records (unsafe on graph queries)
    let _user_results = User::get(query_b, &db_pool).await.unwrap();

    // Get an unique record (fails otherwise):
    let _user = _user_results.uniq().unwrap();

    println!("Done !");
}

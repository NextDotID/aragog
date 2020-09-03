use aragog::{DatabaseConnectionPool, New, DatabaseRecord, Update, ServiceError, Record};
use crate::models::order::Order;
use crate::models::dish::{Dish, DishDTO};
use crate::models::user::User;

mod models;

#[tokio::main]
async fn main() {
    std::env::set_var("SCHEMA_PATH", "./schema.json");

    let db_host = "http://localhost:8529";
    let db_user = "user";
    let db_password = "password";
    let db_name = "dishesAndOrders";

    // Connect to database and generates collections and indexes
    let db_pool = DatabaseConnectionPool::new(db_host, db_name, db_user, db_password).await;

    // Instantiate a new dish
    let dish = Dish::new(DishDTO {
        name: "Pizza Regina".to_string(),
        description: "Tomato base, Ham, Mozzarella, egg".to_string(),
        price: 10
    }).unwrap();
    // Creates a database record
    let mut dish_record = DatabaseRecord::create(dish, &db_pool).await.unwrap();

    // New empty order
    let mut order = Order::new();
    // Add a dish
    order.add(&dish_record.record);
    // Creates a database record
    let mut order_record = DatabaseRecord::create(order, &db_pool).await.unwrap();
    // Update dish
    dish_record.record.update(&DishDTO {
        name: "Pizza Mozzarella".to_string(),
        description: "Tomato base, Mozzarella".to_string(),
        price: 7
    }).unwrap();
    // Add the updated dish to the order
    order_record.record.add(&dish_record.record);
    // Save the order record
    order_record.save(&db_pool).await.unwrap();

    // Checking
    assert_eq!(order_record.record.dishes.len(), 2);
    assert_eq!(order_record.record.total_price, 17);
    assert_eq!(db_pool.collections["Dishes"].record_count().await.unwrap(), 1);

    // Making validation fail
    dish_record.record.price = 0;
    match dish_record.save(&db_pool).await {
        Ok(()) => panic!("Validations should have failed"),
        Err(error) => match error {
            ServiceError::ValidationError(msg) => {
                assert_eq!(msg, String::from("price should be above zero"))
            },
            _ => panic!("Wrong error returned")
        }
    }

    // Query examples

    let user = User {
        username: String::from("LeRevenant1234"),
        first_name: String::from("Robert"),
        last_name: String::from("Surcouf"),
        age: 18,
    };
    let record = DatabaseRecord::create(user, &db_pool).await.unwrap();

    // Find with the primary key
    let _user_record = User::find(&record.key, &db_pool).await.unwrap();

    // Find with a single condition
    let user_record = User::find_by(r#"username == "LeRevenant1234""#, &db_pool).await.unwrap();

    // Find with a single but formatted condition
    let condition = format!(r#"first_name == "{}""#, user_record.record.first_name);
    let _user_record = User::find_by(&condition, &db_pool).await.unwrap();

    // Find a user with multiple conditions
    let mut find_conditions = Vec::new();
    find_conditions.push(r#"last_name == "Surcouf""#);
    find_conditions.push("age > 15");
    let _user_record = User::find_where(find_conditions, &db_pool).await.unwrap();

    // Find all users with multiple conditions
    let mut find_conditions = Vec::new();
    find_conditions.push(r#"last_name == "Surcouf""#);
    find_conditions.push("age > 15");
    let _user_records = User::get_where(find_conditions, &db_pool).await.unwrap();
}
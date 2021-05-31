extern crate env_logger;

use crate::boxed_connection::BoxedConnection;
use crate::models::user::User;
use aragog::{AuthMode, DatabaseConnection, DatabaseRecord};

mod boxed_connection;
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
    let db_connection = DatabaseConnection::builder()
        .with_credentials(
            &std::env::var("DB_HOST").unwrap_or_else(|_| DEFAULT_DB_HOST.to_string()),
            &std::env::var("DB_NAME").unwrap_or_else(|_| DEFAULT_DB_NAME.to_string()),
            &std::env::var("DB_USER").unwrap_or_else(|_| DEFAULT_DB_USER.to_string()),
            &std::env::var("DB_PWD").unwrap_or_else(|_| DEFAULT_DB_PWD.to_string()),
        )
        .with_auth_mode(AuthMode::default())
        .with_schema_path("examples/boxed_example/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();

    // Testing purposes
    db_connection.truncate().await;

    let boxed_connection = BoxedConnection {
        connection: Box::new(db_connection),
    };

    let user = User {
        username: String::from("LeRevenant1234"),
        first_name: String::from("Robert"),
        last_name: String::from("Surcouf"),
    };

    DatabaseRecord::create(user, boxed_connection.connection())
        .await
        .unwrap();

    println!("Done !");
}

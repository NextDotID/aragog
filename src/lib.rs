//! `aragog` is a simple lightweight ODM library for [ArangoDB][ArangoDB] using the [arangors][arangors] driver.
//! The main concept is to provide behaviors allowing to synchronize documents and structs as simply an lightly as possible.
//! In the future versions `aragog` will also be able to act as a ORM and OGM for [ArangoDB][ArangoDB]
//!
//! ### Features
//!
//! By now the available features are:
//! * Creating a database connection pool from a defined `schema.json`
//! * Structures can implement different behaviors:
//! * `Record`: The structure can be written into a ArangoDB collection as well as retrieved, from its `_key` or other query arguments.
//! * `New`: The structure can be initialized from an other type (a form for example). It allows to maintain a privacy level in the model and to use different data formats.
//! * `Update`: The structure can be updated from an other type (a form for example). It allows to maintain a privacy level in the model and to use different data formats.
//! * `Validate`: The structure can perform simple validations before being created or saved into the database.
//! * `Authenticate`: The structure can define a authentication behaviour from a `secret` (a password for example)
//! * Different operations can return a `ServiceError` error that can easily be transformed into a Http Error (can be used for the actix framework)
//!
//! #### Cargo features
//!
//! ##### Actix Http Error
//!
//! If you use this crate with the [actix][actix] framework, you may want the `aragog` errors to be usable as http errors.
//! To do so cou can add to your `cargo.toml` the following `feature`: `actix_http_error`.
//!
//! ```toml
//! aragog = { version = "0.2.2", features = ["actix_http_error"] }
//! ```
//!
//! ##### Password hashing
//!
//! You may want `aragog` to provide a more complete `Authenticate` trait allowing to hash and verify passwords.
//! To do so cou can add to your `cargo.toml` the following `feature`: `password_hashing`.
//!
//! ```toml
//! aragog = { version = "0.2.2", features = ["password_hashing"] }
//! ```
//!
//! It will add two functions in the `Authenticate` trait:
//! ```rust ignore
//! fn hash_password(password: &str, secret_key: &str) -> Result<String, ServiceError>;
//! fn verify_password(password: &str, password_hash: &str, secret_key: &str) -> Result<(), ServiceError>;
//! ```
//! * `hash_password` will return a Argon2 encrypted password hash you can safely store to your database
//! * `verify_password` will check if the provided `password` matches the Argon2 encrypted hash you stored.
//!
//! The Argon2 encryption is based on the [argonautica][argonautica] crate.
//! That crate requires the `clang` lib, so if you deploy on docker you will need to install it or define a custom image.
//!
//! ### Schema and collections
//!
//! In order for everything yo work you need to specify a `schema.json` file. The path of the schema must be set in `SCHEMA_PATH` environment variable or by default the pool will look for it in `src/config/db/schema.json`.
//! > There is an example `schema.json` file in [/examples/simple_app][example_path]
//!
//! The json must look like this:
//!
//! ```json
//! {
//!    "collections": [
//!        {
//!            "name": "collection1",
//!            "indexes": []
//!        },
//!        {
//!            "name": "collection2",
//!            "indexes": [
//!                {
//!                    "name": "byUsernameAndEmail",
//!                    "fields": ["username", "email"],
//!                    "settings": {
//!                        "type": "persistent",
//!                        "unique": true,
//!                        "sparse": false,
//!                        "deduplicate": false
//!                    }
//!                }
//!            ]
//!        }
//!    ]
//! }
//! ```
//!
//! When initializing the `DatabaseConnectionPool` every collection `name` will be searched in the database and if not found the collection will be automatically created.
//! > You don't need to create the collections yourself
//!
//! ##### Indexes
//!
//! The array of Index in `indexes` must have that exact format:
//! * `name`: the index name,
//! * `fields`: an array of the fields concerned on that compound index,
//! * `settings`: this json bloc must be the serialized version of an [IndexSettings][IndexSettings] variant from [arangors][arangors] driver.
//!
//! #### Database Record
//!
//! The global architecture is simple, every *Model* you define that can be synced with the database must implement `Record` and derive from `serde::Serialize`, `serde::Deserialize` and `Clone`.
//! If you want any of the other behaviors you can implement the associated *trait*
//!
//! The final *Model* structure will be an **Exact** representation of the content of a ArangoDB *document*, so without its `_key`, `_id` and `_rev`.
//! Your project should contain some `models` folder with every `struct` representation of your database documents.
//!
//! The real representation of a complete document is `DatabaseRecord<T>` where `T` is your model structure.
//!
//! **Example:**
//!
//! ```rust no_run
//! use aragog::{Record, DatabaseConnectionPool, DatabaseRecord, Validate};
//! use serde::{Serialize, Deserialize};
//! use tokio;
//!
//! #[derive(Serialize, Deserialize, Clone)]
//! pub struct User {
//!     pub username: String,
//!     pub first_name: String,
//!     pub last_name: String,
//!     pub age: usize
//! }
//!
//! impl Record for User {
//!     fn collection_name() -> &'static str { "Users" }
//! }
//!
//! impl Validate for User {
//!     fn validations(&self,errors: &mut Vec<String>) { }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//! // Database connection Setup
//! # std::env::set_var("SCHEMA_PATH", "examples/simple_app/schema.json");
//!
//!     let database_pool = DatabaseConnectionPool::new(
//!         &std::env::var("DB_HOST").unwrap(),
//!         &std::env::var("DB_NAME").unwrap(),
//!         &std::env::var("DB_USER").unwrap(),
//!         &std::env::var("DB_PWD").unwrap()).await;
//! #     database_pool.truncate().await;
//!     // Define a document
//!     let mut user = User {
//!         username: String::from("LeRevenant1234"),
//!         first_name: String::from("Robert"),
//!         last_name: String::from("Surcouf"),
//!         age: 18
//!     };
//!     // user_record is a DatabaseRecord<User>
//!     let mut user_record = DatabaseRecord::create(user, &database_pool).await.unwrap();
//!     // You can access and edit the document
//!     user_record.record.username = String::from("LeRevenant1524356");
//!     // And directly save it
//!     user_record.save(&database_pool).await;
//! }
//! ```
//!
//! #### Querying
//!
//! You can retrieve a document from the database as simply as it gets, from the unique ArangoDB `_key` or from multiple conditions.
//! The example below show different ways to retrieve records, look at each function documentation for more exhaustive exaplanations.
//!
//! **Example**
//! ```rust no_run
//! # use aragog::{Record, DatabaseConnectionPool, DatabaseRecord, Validate};
//! # use serde::{Serialize, Deserialize};
//! # use tokio;
//! #
//! # #[derive(Serialize, Deserialize, Clone)]
//! # pub struct User {
//! #     pub username: String,
//! #     pub first_name: String,
//! #     pub last_name: String,
//! #     pub age: usize
//! # }
//!#
//! # impl Record for User {
//! #     fn collection_name() -> &'static str { "Users" }
//! # }
//!#
//! # impl Validate for User {
//! #     fn validations(&self,errors: &mut Vec<String>) { }
//! # }
//! #
//! # #[tokio::main]
//! # async fn main() {
//! # std::env::set_var("SCHEMA_PATH", "examples/simple_app/schema.json");
//! # let database_pool = DatabaseConnectionPool::new(
//! #       &std::env::var("DB_HOST").unwrap(),
//! #       &std::env::var("DB_NAME").unwrap(),
//! #       &std::env::var("DB_USER").unwrap(),
//! #       &std::env::var("DB_PWD").unwrap()).await;
//! # database_pool.truncate().await;
//! # let mut user = User {
//! #     username: String::from("LeRevenant1234"),
//! #     first_name: String::from("Robert"),
//! #     last_name: String::from("Surcouf"),
//! #     age: 18,
//! # };
//!
//! let record = DatabaseRecord::create(user, &database_pool).await.unwrap();
//! // Find with the primary key
//! let user_record = User::find(&record.key, &database_pool).await.unwrap();
//!
//! // Find with a single condition
//! let user_record = User::find_by(r#"username == "LeRevenant1234""#, &database_pool).await.unwrap();
//!
//! // Find with a single but formatted condition
//! let condition = format!(r#"first_name == "{}""#, user_record.record.first_name);
//! let user_record = User::find_by(&condition, &database_pool).await.unwrap();
//!
//! // Find a user with multiple conditions
//! let mut find_conditions = vec![r#"last_name == "Surcouf""#, "age > 15"];
//!
//! let user_record = User::find_where(find_conditions, &database_pool).await.unwrap();
//!
//! // Find all users with multiple conditions
//! let mut find_conditions = vec![r#"last_name == "Surcouf""#, "age > 15"];
//! let user_records = User::get_where(find_conditions, &database_pool).await.unwrap();
//! # }
//! ```
//!
//! [arangors]: https://docs.rs/arangors
//! [argonautica]: https://github.com/bcmyers/argonautica
//! [example_path]: examples/simple_app
//! [ArangoDB]: https://www.arangodb.com/
//! [IndexSettings]: https://docs.rs/arangors/latest/arangors/index/enum.IndexSettings.html
//! [actix]: https://actix.rs/ "Actix Homepage"
#![forbid(missing_docs)]

pub use {
    authenticate::Authenticate,
    db::database_record::DatabaseRecord,
    db::database_connection_pool::DatabaseConnectionPool,
    error::ServiceError,
    new::New,
    record::Record,
    update::Update,
    validate::Validate,
};

/// Contains useful tools to parse json value and to valiate string formats
pub mod helpers;
mod db;
mod record;
mod authenticate;
mod update;
mod validate;
mod new;
mod error;

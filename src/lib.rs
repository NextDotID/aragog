//! [![Logo](https://gitlab.com/qonfucius/aragog/-/snippets/2090578/raw/master/logo.svg)](http://aragog.rs)
//!
//! # Aragog
//!
//! [![pipeline status](https://gitlab.com/qonfucius/aragog/badges/master/pipeline.svg)](https://gitlab.com/qonfucius/aragog/commits/master)
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![Crates.io](https://img.shields.io/crates/v/aragog.svg)](https://crates.io/crates/aragog)
//! [![aragog](https://docs.rs/aragog/badge.svg)](https://docs.rs/aragog)
//! [![Discord](https://img.shields.io/discord/763034131335741440.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/Xyx3hUP)
//! [![Gitter](https://badges.gitter.im/aragog-rs/community.svg)](https://gitter.im/aragog-rs/community)
//!
//! `aragog` is a fully featured ODM and OGM library for [ArangoDB][ArangoDB] using the [arangors][arangors] driver.
//!
//! The main concept is to provide behaviors allowing to map your structs with ArangoDB documents as simply an lightly as possible.
//! Inspired by Rails's [Active Record](https://github.com/rails/rails/tree/main/activerecord) library
//! `aragog` also provides **hooks** and **validations** for your models.
//!
//! The crate also provides a powerful [AQL][AQL] querying tool allowing complex and safe ArangoDB queries in *Rust*.
//!
//! See the [official website](http://aragog.rs)
//!
//! Minimum rust version: `1.46` (2018 edition)
//!
//! ## Migrations CLI
//!
//! `aragog` provides a safe schema generation and migrations command line interface: [aragog_cli][CLI].
//!
//! ## Features
//!
//! By now the available features are:
//! * Creating a database connection from a defined `schema.yaml` (See [aragog_cli][CLI])
//! * Structures can implement different behaviors:
//!     * `Record`: The structure can be written and retrieved as an ArangoDB [collection document][collection_document]. This is the main trait for your models
//!     * `EdgeRecord`: The structure can be written and retrieved as an ArangoDB [edge collection document][edge_document]
//!     * `Validate`: The structure can perform simple validations before being created or saved into the database.
//!     * `Link`: The structure can define relations with other models based on defined queries.
//!     * `ForeignLink`: The structure can define relations with other models based on defined foreign key.
//! * Structures can also implement optional traits (disabled with the `minimal_traits` feature):
//!     * `AuthorizeAction`: The structure can define authorization behavior on a target record with custom Action type.
//!     * `New`: The structure can be initialized from an other type (a form for example). It allows to maintain a privacy level in the model and to use different data formats.
//!     * `Update`: The structure can be updated from an other type (a form for example). It allows to maintain a privacy level in the model and to use different data formats.
//! * Different operations can return a `Error` error that can easily be transformed into a Http Error
//! * Transactional operations
//!
//! For detailed explanations on theses feature, read the [book](https://gitlab.com/qonfucius/aragog/-/tree/master/book) ([published version](https://aragog.rs/book))
//!
//! ## Quick Reference
//!
//! ### Schema and collections
//!
//! In order for everything to work you need a `schema.yaml` file. Use [aragog_cli][CLI] to create migrations and generate the file.
//!
//! ### Creating a database connection
//!
//! To connect to the database and initialize a database connection you may use the following builder pattern options:
//!
//! ```rust
//! # use aragog::{AuthMode, DatabaseConnection, OperationOptions};
//! # use aragog::schema::DatabaseSchema;
//! # #[tokio::main]
//! # async fn main() {
//! let db_connection = DatabaseConnection::builder()
//!     // You can specify a host and credentials with this method.
//!     // Otherwise, the builder will look for the env vars: `DB_HOST`, `DB_NAME`, `DB_USER` and `DB_PASSWORD`.
//!     .with_credentials("http://localhost:8529", "db", "user", "password")
//!     // You can specify a authentication mode between `Basic` and `Jwt`
//!     // Otherwise the default value will be used (`Basic`).
//!     .with_auth_mode(AuthMode::Basic)
//!     // You can specify some operations options that will be used for every `write` operations like
//!     // `create`, `save` and `delete`.
//!     .with_operation_options(OperationOptions::default())
//!     // You can specify a schema path to initialize the database connection
//!     // Otherwise the env var `SCHEMA_PATH` or the default value `config/db/schema.yaml` will be used.
//!     .with_schema_path("config/db/schema.yaml")
//!     // If you prefer you can use your own custom schema
//!     .with_schema(DatabaseSchema::default())
//! #   .with_schema_path("tests/schema.yaml")
//! #   .with_credentials(
//! #       &std::env::var("DB_HOST").unwrap(),
//! #       &std::env::var("DB_NAME").unwrap(),
//! #       &std::env::var("DB_USER").unwrap(),
//! #       &std::env::var("DB_PASSWORD").unwrap()
//! #   )
//!     // The schema wil silently apply to the database, useful only if you don't use the CLI and migrations
//!     .apply_schema()
//!     // You then need to build the connection
//!     .build()
//!     .await
//!     .unwrap();
//! # }
//! ```
//! None of these options are mandatory.
//!
//! ### Record
//!
//! The global architecture is simple, every *model* you define that can be synced with the database must implement `serde::Serialize`, `serde::Deserialize` and `Clone`.
//! To declare a `struct` as a Model it must derive from `aragog::Record` (the collection name must be the same as the struct) or implement it.
//!
//! The final *model* structure will be an **Exact** representation of the content of a ArangoDB *document*, so without its `_key`, `_id` and `_rev`.
//! Your project should contain some `models` folder with every `struct` representation of your database documents.
//!
//! The real representation of a complete document is `DatabaseRecord<T>` where `T` is your model structure.
//!
//! **Example:**
//!
//! ```rust
//! use aragog::{Record, DatabaseConnection, DatabaseRecord, AuthMode};
//! use serde::{Serialize, Deserialize};
//! use tokio;
//!
//! #[derive(Serialize, Deserialize, Clone, Record)]
//! pub struct User {
//!     pub username: String,
//!     pub first_name: String,
//!     pub last_name: String,
//!     pub age: usize
//! }
//!
//! #[tokio::main]
//! async fn main() {
//! // Database connection Setup
//!     let database_connection = DatabaseConnection::builder()
//! # .with_schema_path("tests/schema.yaml").apply_schema()
//!         .build()
//!         .await
//!         .unwrap();
//! #     database_connection.truncate().await;
//!     // Define a document
//!     let mut user = User {
//!         username: String::from("LeRevenant1234"),
//!         first_name: String::from("Robert"),
//!         last_name: String::from("Surcouf"),
//!         age: 18
//!     };
//!     // user_record is a DatabaseRecord<User>
//!     let mut user_record = DatabaseRecord::create(user, &database_connection).await.unwrap();
//!     // You can access and edit the document
//!     user_record.username = String::from("LeRevenant1524356");
//!     // And directly save it
//!     user_record.save(&database_connection).await.unwrap();
//! }
//! ```
//! ### Edge Record
//!
//! You can declare Edge collection models by deriving from `aragog::EdgeRecord`, the structure requires two string fields: `_from` and `_to`.
//! When deriving from `EdgeRecord` the struct will also automatically derive from `Record` so you'll need to implement `Validate` as well.
//!
//! **Example:**
//!
//! ```rust
//! # use aragog::{Record, EdgeRecord, DatabaseConnection, DatabaseRecord, AuthMode};
//! # use serde::{Serialize, Deserialize};
//! # use tokio;
//! #
//! #[derive(Serialize, Deserialize, Clone, Record)]
//! pub struct Dish {
//!     pub name: String,
//!     pub price: usize
//! }
//!
//! #[derive(Serialize, Deserialize, Clone, Record)]
//! pub struct Order {
//!     pub name: String,
//! }
//!
//! #[derive(Serialize, Deserialize, Clone, Record)]
//! pub struct PartOf { }
//!
//! #[tokio::main]
//! async fn main() {
//! # let database_connection = DatabaseConnection::builder().with_schema_path("tests/schema.yaml").apply_schema().build().await.unwrap();
//! #  database_connection.truncate().await;
//!     // Define a document
//!     let mut dish = DatabaseRecord::create(Dish {
//!         name: "Pizza".to_string(),
//!         price: 10,
//!     }, &database_connection).await.unwrap();
//!     let mut order = DatabaseRecord::create(Order {
//!         name: "Order 1".to_string(),
//!     }, &database_connection).await.unwrap();
//!
//!     let edge = DatabaseRecord::link(&dish, &order, &database_connection, {
//!         PartOf { }
//!     }).await.unwrap();
//!     assert_eq!(edge.id_from(), dish.id());
//!     assert_eq!(edge.id_to(), order.id());
//!     assert_eq!(&edge.key_from(), dish.key());
//!     assert_eq!(&edge.key_to(), order.key());
//! }
//! ```
//!
//! ### Transactions
//!
//! Aragog now supports transactional operations without API changes through the new `Transaction` Object.
//!
//! ```rust
//! # use aragog::{Record, transaction::Transaction, DatabaseConnection, DatabaseRecord, AuthMode};
//! # use serde::{Serialize, Deserialize};
//! # use tokio;
//! #
//! #[derive(Serialize, Deserialize, Clone, Record)]
//! pub struct Dish {
//!     pub name: String,
//!     pub price: usize
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let database_connection = DatabaseConnection::builder()
//!         # .with_schema_path("tests/schema.yaml").apply_schema()
//!         .build().await.unwrap();
//!     #  database_connection.truncate().await;
//!
//!     // Instantiate a new transaction
//!     let transaction = Transaction::new(&database_connection).await.unwrap();
//!     // Safely execute operations:
//!     let output = transaction.safe_execute(|transaction_connection| async move {
//!         // We use the provided `transaction_connection` instead of the classic connection
//!         DatabaseRecord::create(Dish {
//!             name: "Pizza".to_string(),
//!             price: 10,
//!         }, &transaction_connection).await?;
//!         DatabaseRecord::create(Dish {
//!             name: "Pasta".to_string(),
//!             price: 8,
//!         }, &transaction_connection).await?;
//!         DatabaseRecord::create(Dish {
//!             name: "Sandwich".to_string(),
//!             price: 5,
//!         }, &transaction_connection).await?;
//!         Ok(())
//!     }).await.unwrap();
//!
//!     // The output allows to check the transaction state: Committed or Aborted
//!     assert!(output.is_committed());
//! }
//! ```
//!
//! If an operation fails in the `safe_execute` block the transaction will be aborted and every operaiton cancelled.
//!
//!> Note: All the `DatabaseRecord` operation (create, save, link, etc) work as transactional, simply use the the provded transaction `database_connection` instead of the classic connection
//!
//! ### Querying
//!
//! You can retrieve a document from the database as simply as it gets, from the unique ArangoDB `_key` or from multiple conditions.
//! The example below show different ways to retrieve records, look at each function documentation for more exhaustive exaplanations.
//!
//! **Example**
//! ```rust
//! # use aragog::{Record, DatabaseConnection, DatabaseRecord, AuthMode};
//! # use serde::{Serialize, Deserialize};
//! # use aragog::query::{Comparison, Filter};
//! # use tokio;
//! #
//! # #[derive(Serialize, Deserialize, Clone, Record)]
//! # pub struct User {
//! #     pub username: String,
//! #     pub first_name: String,
//! #     pub last_name: String,
//! #     pub age: usize
//! # }
//! #
//! # #[tokio::main]
//! # async fn main() {
//! # let database_connection = DatabaseConnection::builder().with_schema_path("tests/schema.yaml").apply_schema().build().await.unwrap();
//! # database_connection.truncate().await;
//! # let mut user = User {
//! #     username: String::from("LeRevenant1234"),
//! #     first_name: String::from("Robert"),
//! #     last_name: String::from("Surcouf"),
//! #     age: 18,
//! # };
//! // User creation
//! let record = DatabaseRecord::create(user, &database_connection).await.unwrap();
//!
//! // Find with the primary key or..
//! let user_record = User::find(record.key(), &database_connection).await.unwrap();
//! // .. Generate a query and..
//! let query = User::query().filter(Filter::new(Comparison::field("last_name").equals_str("Surcouf")).and(Comparison::field("age").greater_than(15)));
//! // get the only record (fails if no or multiple records)
//! let user_record = User::get(query, &database_connection).await.unwrap().uniq().unwrap();
//!
//! // Find all users with multiple conditions
//! let query = User::query().filter(Filter::new(Comparison::field("last_name").like("%Surc%")).and(Comparison::field("age").in_array(&[15,16,17,18])));
//! let clone_query = query.clone(); // we clone the query
//!
//! // This syntax is valid...
//! let user_records = User::get(query, &database_connection).await.unwrap();
//! // ... This one too
//! let user_records = clone_query.call(&database_connection).await.unwrap().get_records::<User>();
//! # }
//! ```
//! You can simplify the previous queries with some tweaks and macros:
//! ```rust
//! #[macro_use]
//! extern crate aragog;
//! # use aragog::{Record, DatabaseConnection, DatabaseRecord, AuthMode};
//! # use serde::{Serialize, Deserialize};
//! # use aragog::query::{Query, Comparison, Filter, QueryResult};
//! # use tokio;
//! #
//! # #[derive(Serialize, Deserialize, Clone, Record)]
//! # pub struct User {
//! #     pub username: String,
//! #     pub first_name: String,
//! #     pub last_name: String,
//! #     pub age: usize
//! # }
//! # #[tokio::main]
//! # async fn main() {
//! let database_connection = DatabaseConnection::builder().with_schema_path("tests/schema.yaml").apply_schema().build().await.unwrap();
//! # database_connection.truncate().await;
//! # let mut user = User {
//! #     username: String::from("LeRevenant1234"),
//! #     first_name: String::from("Robert"),
//! #     last_name: String::from("Surcouf"),
//! #     age: 18,
//! # };
//!
//! let record = DatabaseRecord::create(user, &database_connection).await.unwrap();
//!
//! // Find a user with a query
//! let query = User::query().filter(compare!(field "last_name").equals_str("Surcouf").and(compare!(field "age").greater_than(15)));
//!
//! // get the only record (fails if no or multiple records)
//! let user_record = User::get(query, &database_connection).await.unwrap().uniq().unwrap();
//!
//! // Find all users with multiple conditions
//! let query = User::query().filter(compare!(field "last_name").like("%Surc%").and(compare!(field "age").in_array(&[15,16,17,18])));
//! // This syntax is valid...
//! let user_records :QueryResult<User> = User::get(query.clone(), &database_connection).await.unwrap();
//! // ... This one too
//! let user_records :QueryResult<User> = query.call(&database_connection).await.unwrap();
//! # }
//! ```
//! #### Query Object
//!
//! You can intialize a query in the following ways:
//! * `Query::new("CollectionName")`
//! * `Object.query()` (only works if `Object` implements `Record`)
//! * `query!("CollectionName")`
//!
//! You can customize the query with the following methods:
//! * `filter()` you can specify AQL comparisons
//! * `prune()` you can specify blocking AQL comparisons for traversal queries
//! * `sort()` you can specify fields to sort with
//! * `limit()` you can skip and limit the query results
//! * `distinct()` you can skip duplicate documents
//! > The order of operations will be respected in the rendered AQL query (except for `distinct`)
//!
//! you can then call a query in the following ways:
//! * `query.call::<Object>(&database_connection)`
//! * `Object::get(query, &database_connection)`
//!
//! Which will return a `JsonQueryResult` containing a `Vec` of `serde_json::Value`.
//! `JsonQueryResult` can return deserialized models as `DatabaseRecord` by calling `.get_records::<T>()`
//!
//! ##### Filter
//!
//! You can initialize a `Filter` with `Filter::new(comparison)`
//!
//! Each comparison is a `Comparison` struct built via `ComparisonBuilder`:
//! ```rust ignore
//! // for a simple field comparison
//!
//! // Explicit
//! Comparison::field("some_field").some_comparison("compared_value");
//! // Macro
//! compare!(field "some_field").some_comparison("compared_value");
//!
//! // for field arrays (see ArangoDB operators)
//!
//! // Explicit
//! Comparison::all("some_field_array").some_comparison("compared_value");
//! // Macro
//! compare!(all "some_field_array").some_comparison("compared_value");
//!
//! // Explicit
//! Comparison::any("some_field_array").some_comparison("compared_value");
//! // Macro
//! compare!(any "some_field_array").some_comparison("compared_value");
//!
//! // Explicit
//! Comparison::none("some_field_array").some_comparison("compared_value");
//! // Macro
//! compare!(none "some_field_array").some_comparison("compared_value");
//! ```
//! All the currently implemented comparison methods are listed under [ComparisonBuilder][ComparisonBuilder] documentation page.
//!
//! Filters can be defined explicitely like this:
//!
//! ```rust
//! # use aragog::query::{Comparison, Filter};
//! let filter = Filter::new(Comparison::field("name").equals_str("felix"));
//! ```
//!
//! or
//!
//! ```rust
//! # use aragog::query::{Comparison, Filter};
//! let filter :Filter = Comparison::field("name").equals_str("felix").into();
//! ```
//!
//! #### Traversal Querying
//!
//! You can use graph features with sub-queries with different ways:
//!
//! ##### Straightforward Traversal query
//!
//! * Explicit way
//! ```rust
//! # use aragog::query::Query;
//! let query = Query::outbound(1, 2, "edgeCollection", "User/123");
//! let query = Query::inbound(1, 2, "edgeCollection", "User/123");
//! let query = Query::any(1, 2, "edgeCollection", "User/123");
//! // Named graph
//! let query = Query::outbound_graph(1, 2, "NamedGraph", "User/123");
//! let query = Query::inbound_graph(1, 2, "NamedGraph", "User/123");
//! let query = Query::any_graph(1, 2, "NamedGraph", "User/123");
//! ```
//!
//! * Implicit way from a `DatabaseRecord<T>`
//!
//! ```rust ignore
//! # use aragog::query::Query;
//! let query = user_record.outbound_query(1, 2, "edgeCollection");
//! let query = user_record.inbound_query(1, 2, "edgeCollection");
//! // Named graph
//! let query = user_record.outbound_graph(1, 2, "NamedGraph");
//! let query = user_record.inbound_graph(1, 2, "NamedGraph");
//! ```
//!
//! ##### Sub queries
//!
//! Queries can be joined together through
//! * Edge traversal:
//!
//! ```rust
//! # use aragog::query::Query;
//! let query = Query::new("User")
//!     .join_inbound(1, 2, false, Query::new("edgeCollection"));
//! ```
//!
//! * Named Graph traversal:
//!
//! ```rust
//! # use aragog::query::Query;
//! let query = Query::new("User")
//!     .join_inbound(1, 2, true, Query::new("SomeGraph"));
//! ```
//!
//! It works with complex queries:
//!
//! ```rust
//! # use aragog::query::{Query, Comparison};
//! let query = Query::new("User")
//!     .filter(Comparison::field("age").greater_than(10).into())
//!     .join_inbound(1, 2, false,
//!         Query::new("edgeCollection")
//!             .sort("_key", None)
//!             .join_outbound(1, 5, true,
//!                 Query::new("SomeGraph")
//!                     .filter(Comparison::any("roles").like("%Manager%").into())
//!                     .distinct()
//!                 )
//!     );
//! ```
//!
//! [arangors]: https://docs.rs/arangors
//! [argonautica]: https://github.com/bcmyers/argonautica
//! [ArangoDB]: https://www.arangodb.com/
//! [IndexSettings]: https://docs.rs/arangors/latest/arangors/index/enum.IndexSettings.html
//! [ComparisonBuilder]: https://docs.rs/aragog/latest/aragog/query/struct.ComparisonBuilder.html
//! [CLI]: https://crates.io/crates/aragog_cli
//! [edge_document]: https://www.arangodb.com/docs/stable/data-modeling-documents-document-methods.html#edges
//! [collection_document]: https://www.arangodb.com/docs/stable/data-modeling-documents-document-methods.html#document
//! [arango_download]: https://www.arangodb.com/download "Download Arango"
//! [arango_doc]: https://www.arangodb.com/docs/stable/getting-started.html "Arango getting started"
//! [AQL]: https://www.arangodb.com/docs/stable/aql/ "AQL"
//!
#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![deny(warnings)]
// #![warn(rustdoc::broken_intra_doc_links)]

pub extern crate async_trait;

#[cfg(all(feature = "async", feature = "blocking"))]
compile_error!(
    r#"feature "blocking" and "async" cannot be set at the same time.
    If what you want is "blocking", please turn off default features by adding "default-features=false" in your Cargo.toml"#
);

#[cfg(all(not(feature = "async"), not(feature = "blocking")))]
compile_error!(
    r#"feature "blocking" and "async" cannot be disabled at the same time. Enable one of them"#
);

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use aragog_macros::*;

#[cfg(not(feature = "minimal_traits"))]
pub use {authorize_action::AuthorizeAction, new::New, update::Update};
pub use {
    db::database_access::DatabaseAccess, db::database_connection::AuthMode,
    db::database_connection::DatabaseConnection, db::database_record::DatabaseRecord,
    db::operation_options::OperationOptions, db::transaction, edge_record::EdgeRecord,
    error::Error, foreign_link::ForeignLink, link::Link, record::Record,
    undefined_record::UndefinedRecord, validate::Validate,
};

#[cfg(not(feature = "minimal_traits"))]
mod authorize_action;
mod db;
mod edge_record;
mod foreign_link;
mod link;
#[cfg(not(feature = "minimal_traits"))]
mod new;
mod record;
#[cfg(not(feature = "minimal_traits"))]
mod update;
mod validate;

/// Error handling
pub mod error;
/// contains querying struct and functions.
pub mod query;
/// Database schema construction utility, available for advanced development.
/// For classic usage use the `aragog_cli` and its migration engine to generate your schema
pub mod schema;
mod undefined_record;

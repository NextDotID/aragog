<!-- cargo-sync-readme start -->

# Aragog

[![pipeline status](https://gitlab.com/qonfucius/aragog/badges/master/pipeline.svg)](https://gitlab.com/qonfucius/aragog/commits/master)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/aragog.svg)](https://crates.io/crates/aragog)
[![aragog](https://docs.rs/aragog/badge.svg)](https://docs.rs/aragog)

`aragog` is a simple lightweight ODM/OGM library for [ArangoDB][ArangoDB] using the [arangors][arangors] driver.
The main concept is to provide behaviors allowing to synchronize documents and structs as simply an lightly as possible.

The crate also provides a powerful AQL querying tool allowing complex graph queries in *Rust*

### Features

By now the available features are:
* Creating a database connection pool from a defined `schema.json`
* Structures can implement different behaviors:
    * `Record`: The structure can be written into a ArangoDB collection as well as retrieved, from its `_key` or other query arguments.
    * `New`: The structure can be initialized from an other type (a form for example). It allows to maintain a privacy level in the model and to use different data formats.
    * `Update`: The structure can be updated from an other type (a form for example). It allows to maintain a privacy level in the model and to use different data formats.
    * `Validate`: The structure can perform simple validations before being created or saved into the database.
    * `Authenticate`: The structure can define a authentication behaviour from a `secret` (a password for example) (see `password_hashing` section)
    * `AuthorizeAction`: The structure can define authorization behavior on a target record with custom Action type.
* Different operations can return a `ServiceError` error that can easily be transformed into a Http Error (can be used for the actix framework)

#### Cargo features

##### Actix and Open API

If you use this crate with the [actix-web][actix] framework, you may want the `aragog` errors to be usable as http errors.
To do so you can add to your `cargo.toml` the following `feature`: `actix`. This will add Actix 3 dependency and compatibility

```toml
aragog = { version = "^0.5", features = ["actix"] }
```

If you also want to be able to use [paperclip][paperclip], you may want the `aragog` to be compatible.
To do so you can add to your `cargo.toml` the following `feature`: `open-api`.

```toml
aragog = { version = "^0.5", features = ["actix", "open-api"] }
```

##### Password hashing

You may want `aragog` to provide a more complete `Authenticate` trait allowing to hash and verify passwords.
To do so you can add to your `cargo.toml` the following `feature`: `password_hashing`.

```toml
aragog = { version = "^0.5", features = ["password_hashing"] }
```

It will add two functions in the `Authenticate` trait:
```rust 
fn hash_password(password: &str, secret_key: &str) -> Result<String, ServiceError>;
fn verify_password(password: &str, password_hash: &str, secret_key: &str) -> Result<(), ServiceError>;
```
* `hash_password` will return a Argon2 encrypted password hash you can safely store to your database
* `verify_password` will check if the provided `password` matches the Argon2 encrypted hash you stored.

The Argon2 encryption is based on the [argonautica][argonautica] crate. 
That crate requires the `clang` lib, so if you deploy on docker you will need to install it or define a custom image.

### Schema and collections

In order for everything yo work you need to specify a `schema.json` file. The path of the schema must be set in `SCHEMA_PATH` environment variable or by default the pool will look for it in `src/config/db/schema.json`.
> There are example `schema.json` files in [/examples/][example_path]

The json must look like this:

```json
{
  "collections": [
    {
      "name": "Collection1",
      "indexes": []
    },
    {
      "name": "Collection2",
      "indexes": [
        {
          "name": "byUsernameAndEmail",
          "fields": ["username", "email"],
          "settings": {
            "type": "persistent",
            "unique": true,
            "sparse": false,
            "deduplicate": false
          }
        } 
      ]
    }
  ],
  "edge_collections": [
    {
      "name": "EdgeCollection1"
    } 
  ]
}
```

When initializing the `DatabaseConnectionPool` every collection `name` will be searched in the database and if not found the collection will be automatically created.
> You don't need to create the collections yourself

##### Indexes

The array of Index in `indexes` must have that exact format:
* `name`: the index name,
* `fields`: an array of the fields concerned on that compound index,
* `settings`: this json bloc must be the serialized version of an [IndexSettings][IndexSettings] variant from [arangors][arangors] driver.
> There is no indexing for `edge_collections`

#### Record

The global architecture is simple, every *model* you define that can be synced with the database must implement `serde::Serialize`, `serde::Deserialize` and `Clone`.
To declare a `struct` as a Model it must derive from `aragog::Record` (the collection name must be the same as the struct)

The model needs to have *Validations* so you'll need to either:
* Implement `aragog::Validate` and specify validations
* Derive `aragog::Validate` and validations will be empty.

If you want any of the other behaviors you can implement the associated *trait*

The final *model* structure will be an **Exact** representation of the content of a ArangoDB *document*, so without its `_key`, `_id` and `_rev`.
Your project should contain some `models` folder with every `struct` representation of your database documents.

The real representation of a complete document is `DatabaseRecord<T>` where `T` is your model structure.

**Example:**

```rust
use aragog::{Record, DatabaseConnectionPool, DatabaseRecord, Validate};
use serde::{Serialize, Deserialize};
use tokio;

#[derive(Serialize, Deserialize, Clone, Record, Validate)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: usize
}

#[tokio::main]
async fn main() {
    // Database connection Setup
    let database_pool = DatabaseConnectionPool::new("http://localhost:8529", "db", "user", "password").await;
    // Define a document
    let mut user = User {
        username: String::from("LeRevenant1234"),
        first_name: String::from("Robert"),
        last_name: String::from("Surcouf"),
        age: 18
    };
    // user_record is a DatabaseRecord<User>
    let mut user_record = DatabaseRecord::create(user, &database_pool).await.unwrap();
    // You can access and edit the document
    user_record.record.username = String::from("LeRevenant1524356");
    // And directly save it
    user_record.save(&database_pool).await;
}
```

#### Edge Record

You can declare Edge collection models by deriving from `aragog::EdgeRecord`, the structure requires two string fields: `_from` and `_to`.
When deriving from `EdgeRecord` the struct will also automatically derive from `Record` so you'll need to implement `Validate` as well.

**Example:**
```rust
#[derive(Serialize, Deserialize, Clone, Record, Validate)]
pub struct Dish {
    pub name: String,
    pub price: usize
}
#[derive(Serialize, Deserialize, Clone, Record, Validate)]
pub struct Order {
    pub name: String,
}
#[derive(Serialize, Deserialize, Clone, EdgeRecord, Validate)]
pub struct PartOf {
    pub _from: String,
    pub _to: String,
}
#[tokio::main]
async fn main() {
   let database_pool = DatabaseConnectionPool::new("http://localhost:8529", "db", "user", "password").await;
    // Define a document
    let mut dish = DatabaseRecord::create(Dish {
        name: "Pizza".to_string(),
        price: 10,
    }, &database_pool).await.unwrap();
    let mut order = DatabaseRecord::create(Order {
        name: "Order 1".to_string(),
    }, &database_pool).await.unwrap();
    let edge = DatabaseRecord::link(&dish, &order, &database_pool, |_from, _to| {
        PartOf { _from, _to }
    }).await.unwrap();
    assert_eq!(&edge.record._from(), &dish.id);
    assert_eq!(&edge.record._to(), &order.id);
    assert_eq!(&edge.record._from_key(), &dish.key);
    assert_eq!(&edge.record._to_key(), &order.key);
}
```

#### Querying

You can retrieve a document from the database as simply as it gets, from the unique ArangoDB `_key` or from multiple conditions.
The example below show different ways to retrieve records, look at each function documentation for more exhaustive explanations.

**Example**
```rust
// User creation
let record = DatabaseRecord::create(user, &database_pool).await.unwrap();
// Find with the primary key or..
let user_record = User::find(&record.key, &database_pool).await.unwrap();
// .. Generate a query and..
let query = User::query().filter(Filter::new(Comparison::field("last_name").equals_str("Surcouf")).and(Comparison::field("age").greater_than(15)));
// get the only record (fails if no or multiple records)
let user_record = User::get(query, &database_pool).await.unwrap().uniq().unwrap();

// Find all users with multiple conditions
let query = User::query().filter(Filter::new(Comparison::field("last_name").like("%Surc%")).and(Comparison::field("age").in_array(&[15,16,17,18])));
let clone_query = query.clone(); // we clone the query
// This syntax is valid...
let user_records = User::get(query, &database_pool).await.unwrap();
// ... This one too
let user_records = clone_query.call(&database_pool).await.unwrap().get_records::<User>();
```

You can simplify the previous queries with some tweaks and macros:
```rust
#[macro_use] extern crate aragog;
// Find a user with multiple conditions
let query = query!("Users").filter(compare!(field "last_name").equals_str("Surcouf").and(compare!(field "age").greater_than(15)));
let records = User::get(query, &database_pool).await.unwrap();
```

The querying system hierarchy works this way:
```rust
Query::new("collection_name").filter(comparison_1).and(comparison_2).or(comparison_3).sort().limit().distinct();
```

##### Query Object

You can intialize a query in the following ways:
* `Query::new("CollectionName")`
* `Object.query()` (only works if `Object` implements `Record`)
* `query!("CollectionName")`

You can customize the query with the following operations:
* `filter()` you can specify AQL comparisons
* `prune()` you can specify blocking AQL comparisons for traversal queries
* `sort()` you can specify fields to sort with
* `limit()` you can skip and limit the query results
* `distinct()` you can skip duplicate documents
> The order of operations will be respected in the rendered AQL query (except for `distinct`)

you can then call a query in the following ways:
* `query.call::<Object>(&database_connection_pool)`
* `Object::get(query, &database_connection_pool`

Which will return a `JsonQueryResult` containing a `Vec` of `serde_json::Value`.
`JsonQueryResult` can return deserialized models as `DatabaseRecord` by calling `.get_records::<T>()`
If you want to receive a unique record and render an error in case of multiple record you can use `uniq()`.

###### Filter

You can initialize a `Filter` with `Filter::new(comparison)`

Each comparison is a `Comparison` struct built via `ComparisonBuilder`:
```rust
// for a simple field comparison

// Explicit
Comparison::field("some_field").some_comparison("compared_value");
// Macro
compare!(field "some_field").some_comparison("compared_value");

// for field arrays (see ArangoDB operators)

// Explicit
Comparison::all("some_field_array").some_comparison("compared_value");
// Macro
compare!(all "some_field_array").some_comparison("compared_value");

// Explicit
Comparison::any("some_field_array").some_comparison("compared_value");
// Macro
compare!(any "some_field_array").some_comparison("compared_value");

// Explicit
Comparison::none("some_field_array").some_comparison("compared_value");
// Macro
compare!(none "some_field_array").some_comparison("compared_value");
```
All the currently implemented comparison methods are listed under [ComparisonBuilder][ComparisonBuilder] documentation page.

Filters can be defined explicitely like this:
```rust
let filter = Filter::new(Comparison::field("name").equals_str("felix"));
```
or
```rust
let filter :Filter = Comparison::field("name").equals_str("felix").into();
```

##### Traversal Querying

You can use graph features with sub-queries with different ways:

###### Straightforward traversal query

* Explicit way
```rust
use aragog::query::Query;

let query = Query::outbound(1, 2, "edgeCollection", "User/123");
let query = Query::inbound(1, 2, "edgeCollection", "User/123");
let query = Query::any(1, 2, "edgeCollection", "User/123");
// Named graph
let query = Query::outbound_graph(1, 2, "NamedGraph", "User/123");
let query = Query::inbound_graph(1, 2, "NamedGraph", "User/123");
let query = Query::any_graph(1, 2, "NamedGraph", "User/123");
```
* Implicit way from a `DatabaseRecord<T>`
```rust
# use aragog::query::Query;
let query = user_record.outbound_query(1, 2, "edgeCollection");
let query = user_record.inbound_query(1, 2, "edgeCollection");
```
###### Sub queries

Queries can be joined together through
* Edge traversal:
```rust
# use aragog::query::Query;
let query = Query::new("User")
    .join_inbound(1, 2, false, Query::new("edgeCollection"));
```
* Named Graph traversal:
```rust
# use aragog::query::Query;
let query = Query::new("User")
    .join_inbound(1, 2, true, Query::new("SomeGraph"));
```
It works with complex queries:
```rust
# use aragog::query::{Query, Comparison};
let query = Query::new("User")
    .filter(Comparison::field("age").greater_than(10).into())
    .join_inbound(1, 2, false,
        Query::new("edgeCollection")
            .sort("_key", None)
            .join_outbound(1, 5, true,
                Query::new("SomeGraph")
                    .filter(Comparison::any("roles").like("%Manager%").into())
                    .distinct()
        )
    );
```
### TODO

* Query system:
    - [X] Simple and modular query system
    - [ ] Advanced query system supporting:
        - [X] Array variant querying (`ANY`, `NONE`, `ALL`)
        - [X] Sort, limit and distinct methods
        - [ ] Custom return system
        - [X] `PRUNE` operation
        - [ ] Procedural Macros for syntax simplification and field presence validation at compile time
        - [ ] ArangoDB functions (`LENGTH`, `ABS`, etc.)
* ORM and OGM
    - [X] Pundit like authorizations (authorize actions on model)
    - [ ] Relations
        - [X] Handle graph vertices and edges
        - [ ] Handle SQL-like relations (foreign keys)
    - [ ] Handle key-value pair system (redis like)
* Middle and long term:
    - [ ] Handle revisions/concurrency correctly
    - [ ] Code Generation
        - [X] Record `derive` macro
        - [X] EdgeRecord `derive` macro checking `_from` and `_to` presence at compile time
        - [ ] Handle a `attribute` macro for indexes and generate the schema at compile time
        - [ ] Handle Migrations
    - [ ] Define possible `async` validations for database advance state check

### Arango db setup

**Installation** (See official documentation [Here][arango_doc])

* [Download Link][arango_download]
* Run it with `/usr/local/sbin/arangod` The default installation contains one database `_system` and a user named `root`
* Create a user and database for the project with the `arangosh` shell
```
arangosh> db._createDatabase("DB_NAME");
arangosh> var users = require("@arangodb/users");
arangosh> users.save("DB_USER", "DB_PASSWORD");
arangosh> users.grantDatabase("DB_USER", "DB_NAME");
```
> It is a good practice to create a test db and a development db.
* you can connect to the new created db with
```
$> arangosh --server.username $DB_USER --server.database $DB_NAME
```

### Q&A

* How can I customize the `collection_name` of my `Record` ?
> Instead of deriving from `aragog::Record` you can implement `aragog::Record` directly and declare `collection_name()` as a string litteral
> This is not recommended as it will not be possible in the future with automatic schema generation
>
* How can I customize the `collection_name` of my `EdgeRecord` ?
> Instead of deriving from `aragog::EdgeRecord` you can implement `aragog::EdgeRecord` and `aragog::Record` directly and declare `collection_name()` as a string litteral.
> This is not recommended as it will not be possible in the future with automatic schema generation

### License

`aragog` is provided under the MIT license. See [LICENSE](./LICENSE).
An simple lightweight ODM for [ArangoDB][ArangoDB] based on [arangors][arangors].

Special thanks to [fMeow][fMeow] creator of [arangors][arangors] and [inzanez][inzanez]

<!-- cargo-sync-readme end -->

[arangors]: https://github.com/fMeow/arangors
[argonautica]: https://github.com/bcmyers/argonautica
[example_path]: examples/
[fMeow]: https://github.com/fMeow/
[inzanez]: https://github.com/inzanez/
[ArangoDB]: https://www.arangodb.com/
[IndexSettings]: https://docs.rs/arangors/latest/arangors/index/enum.IndexSettings.html
[ComparisonBuilder]: https://docs.rs/aragog/latest/aragog/query/struct.ComparisonBuilder.html
[arango_download]: https://www.arangodb.com/download "Download Arango"
[arango_doc]: https://www.arangodb.com/docs/stable/getting-started.html "Arango getting started"
[actix]: https://actix.rs/ "Actix Homepage"
[paperclip]: https://github.com/wafflespeanut/paperclip "Paperclip Github"
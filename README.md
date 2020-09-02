<!-- cargo-sync-readme start -->

# Aragog

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/aragog.svg)](https://crates.io/crates/aragog)
[![aragog](https://docs.rs/aragog/badge.svg)](https://docs.rs/aragog)

`aragog` is a simple lightweight ODM library for [ArangoDB][ArangoDB] using the [arangors][arangors] driver.
The main concept is to provide behaviors allowing to synchronize documents and structs as simply an lightly as possible.
In the future versions `aragog` will also be able to act as a ORM and OGM for [ArangoDB][ArangoDB]

### Features

By now the available features are:
* Creating a database connection pool from a defined `schema.json`
* Structures can implement different behaviors:
    * `Record`: The structure can be written into a ArangoDB collection as well as retrieved, from its `_key` or other query arguments.
    * `New`: The structure can be initialized from an other type (a form for example). It allows to maintain a privacy level in the model and to use different data formats.
    * `Update`: The structure can be updated from an other type (a form for example). It allows to maintain a privacy level in the model and to use different data formats.
    * `Validate`: The structure can perform simple validations before being created or saved into the database.
    * `Authenticate`: The structure can define a authentication behaviour from a `secret` (a password for example)
* Different operations can return a `AragogServiceError` error that can easily be transformed into a Http Error (can be used for the actix framework)

#### Schema and collections

In order for everything yo work you need to specify a `schema.json` file. The path of the schema must be set in `SCHEMA_PATH` environment variable or by default the pool will look for it in `src/config/db/schema.json`.
> There is an example `schema.json` file in [/examples/simple_food_order_app][example_path]

The json must look like this:

````json
{
  "collections": [
    {
      "name": "collection1",
      "indexes": []
    },
    {
      "name": "collection2",
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
  ]
}
````

When initializing the `DatabaseConnectionPool` every collection `name` will be searched in the database and if not found the collection will be automatically created.
> You don't need to create the collections yourself

##### Indexes

The array of Index in `indexes` must have that exact format:
* `name`: the index name,
* `fields`: an array of the fields concerned on that compound index,
* `settings`: this json bloc must be the serialized version of an [IndexSettings][IndexSettings] variant from [arangors][arangors] driver.

#### Database Record

The global architecture is simple, every *Model* you define that can be synced with the database must implement `Record` and derive from `serde::Serialize`, `serde::Deserialize` and `Clone`.
If you want any of the other behaviors you can implement the associated *trait*

The final *Model* structure will be an **Exact** representation of the content of a ArangoDB *document*, so without its `_key`, `_id` and `_rev`.
Your project should contain some `models` folder with every `struct` representation of your database documents.

The real representation of a complete document is `DatabaseRecord<T>` where `T` is your model structure.

> Example:

```rust
use serde::{Deserialize, Serialize};
use aragog::{Record, DatabaseRecord, DatabaseConnectionPool};

#[derive(Serialize, Deserialize, Clone)]
pub struct User { 
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

impl Record for User {
    fn collection_name() -> String {
        String::from("Users")  
    }
}

async fn main() {
    /// Database connection Setup
    let database_pool = DatabaseConnectionPool::new("http://localhost:8529", "db", "root", "").await;
    /// Define a document
    let mut user = User {
        username: String::from("LeRevenant1234"),
        first_name: String::from("Robert"),
        last_name: String::from("Surcouf")
    };
    /// user_record is a DatabaseRecord<User>
    let user_record = DatabaseRecord::create(user, &database_pool).await;
    /// You can access and edit the document
    user_record.record.username = String::from("LeRevenant1524356");
    /// And directly save it
    user_record.save(&database_pool).await;
}
```

#### Querying

You can retrieve a document from the database as simply as it gets, from the unique ArangoDB `_key` or from multiple conditions.
The example below show different ways to retrieve records, look at each function documentation for more exhaustive exaplanations.

> Example
````rust
/// Find with the primary key
let user_record = User::find("1234567", &database_pool).await.unwrap();

/// Find with a single condition
let user_record = User::find_by("username" ,"LeRevenant1234", &database_pool).await.unwrap();

/// Find a user with multiple conditions
let mut find_conditions = Vec::new();
find_conditions.push(r#"username == "LeRevenant1234""#);
find_conditions.push(r#"last_name == "Surcouf""#);
find_conditions.push("age > 15");
let user_record = User::find_where(find_conditions, &database_pool).await.unwrap();

/// Find all users with multiple conditions
let mut find_conditions = Vec::new();
find_conditions.push(r#"username == "LeRevenant1234""#);
find_conditions.push(r#"last_name == "Surcouf""#);
find_conditions.push("age > 15");
let user_records = User::get_where(find_conditions, &database_pool).await.unwrap();
````

### TODO

* Critic features:
    - [ ] Advanced and modular query system
* ORM and OGM
    - [ ] Relations
        - [ ] Handle graph vertices and edges
        - [ ] Handle SQL-like relations (foreign keys)
    - [ ] Handle key-value pair system (redis like)
* Middle and long term:
    - [ ] Handle revisions/concurrency correctly
    - [ ] Code Generation
        - [ ] Avoid string literals as collection names
        - [ ] Handle Migrations
    - [ ] Define possible `async` validations for database advance state check

### Arango db setup

**Installation** (See official documentation [Here] [arango_doc])

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
```$> arangosh --server.username $DB_USER --server.database $DB_NAME```

### License

`aragog` is provided under the MIT license. See [LICENSE](./LICENSE).
An simple lightweight ODM for [ArangoDB][ArangoDB] based on [arangors][arangors].

Special thanks to [fMeow][fMeow] creator of [arangors][arangors] and [inzanez][inzanez]

<!-- cargo-sync-readme end -->

[arangors]: https://github.com/fMeow/arangors
[example_path]: ./examples/simple_food_order_app
[fMeow]: https://github.com/fMeow/
[inzanez]: https://github.com/inzanez/
[ArangoDB]: https://www.arangodb.com/
[IndexSettings]: https://docs.rs/arangors/0.4.3/arangors/index/enum.IndexSettings.html
[arango_download]: https://www.arangodb.com/download "Download Arango"
[arango_doc]: https://www.arangodb.com/docs/stable/getting-started.html "Arango getting started"
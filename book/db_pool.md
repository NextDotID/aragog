
# The Database Pool

The database pool is the main handler of every ArangoDB communication. 
You need to initialize one to be able to use the other library features.

The pool loads a **Database Schema** (`schema.yaml`) file. Use [aragog_cli](https://crates.io/crates/aragog_cli) to create migrations and generate the file.

> Note: You can also create your own file but it's not recommended

## Creating a pool

To connect to the database and initialize a connection pool you may use the following builder pattern options:

 ```rust
 let db_pool = DatabaseConnectionPool::builder()
     // You can specify a host and credentials with this method.
     // Otherwise, the builder will look for the env vars: `DB_HOST`, `DB_NAME`, `DB_USER` and `DB_PASSWORD`.
     .with_credentials("http://localhost:8529", "db", "user", "password")
     // You can specify a authentication mode between `Basic` and `Jwt`
     // Otherwise the default value will be used (`Basic`).
     .with_auth_mode(AuthMode::Basic)
     // You can specify a schema path to initialize the database pool
     // Otherwise the env var `SCHEMA_PATH` or the default value `config/db/schema.yaml` will be used.
     .with_schema_path("config/db/schema.yaml")
     // If you prefer you can use your own custom schema
     .with_schema(DatabaseSchema::default())
     // The schema wil silently apply to the database, useful only if you don't use the CLI and migrations
     .apply_schema()
     // You then need to build the pool
     .build()
     .await
     .unwrap();
 ```

### Using env vars

None of the builder options are mandatory, the following works perfectly if all required environment variables are set:

 ```rust
 let db_pool = DatabaseConnectionPool::builder()
     .build()
     .await
     .unwrap();
 ```

The env vars are the following:

| Name                | Description                                                     |
|---------------------|-----------------------------------------------------------------|
| DB_HOST             | The ArangoDB host, usually `http://localhost:8259`              |
| DB_NAME             | The ArangoDB database name                                      |
| DB_USER             | The Database user you want to use                               |
| DB_PASSWORD         | The `DB_USER` password                                          |
| SCHEMA_PATH         | The path of the schema file, by default `config/db/schema.yaml` |

> It is recommended to leave the `SCHEMA_PATH` unset, as the default value is idiomatic

### Set up ArangoDB

**Installation** (See official documentation [Here](https://www.arangodb.com/docs/stable/getting-started.html))

* [Download Link](https://www.arangodb.com/download)
* Run it with `/usr/local/sbin/arangod` The default installation contains one database `_system` and a user named `root`
* Create a user and database for the project with the `arangosh` shell

 ```bash
 arangosh> db._createDatabase("DB_NAME");
 arangosh> var users = require("@arangodb/users");
 arangosh> users.save("DB_USER", "DB_PASSWORD");
 arangosh> users.grantDatabase("DB_USER", "DB_NAME");
 ```
> It is a good practice to create a test db and a development db.
* you can connect to the new created db with
 ```bash
 $> arangosh --server.username $DB_USER --server.database $DB_NAME
 ```

## Technical notes

### `DatabaseAccess` trait
   
The object `DatabaseConnectionPool` is the default handler for every database operation but you can create your own.

Every operation taking the pool as an argument is taking a **Generic** type implementing `DatabaseAccess`,
so you can implement it on your own struct.

> Note: This is not recommended, and modification to `DatabaseAccess` can happen without considering them as **breaking**.

### `truncate_database`

The `DatabaseConnectionPool` provides a `truncate_database` method but you should use it only for testing purposes,
it is highly destructive as it will drop every collection known to the pool.

## Todo list

- [ ] Add `with_host` builder option to remove the host from `with_credentials`
- [ ] Add `with_database` builder option to remove the database name from `with_credentials`
- [ ] Optional implementation of the `r2d2` pool pattern
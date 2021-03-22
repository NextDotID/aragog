
# The Database Connection

The database connection is the main handler of every ArangoDB communication. 
You need to initialize one to be able to use the other library features.

The connection loads a **Database Schema** (`schema.yaml`) file. Use [aragog_cli](https://crates.io/crates/aragog_cli) to create migrations and generate the file.

> Note: You can also create your own file but it's not recommended

## Creating a Database Connection

To connect to the database and initialize a database connection you may use the following builder pattern options:

 ```rust
 let db_connection = DatabaseConnection::builder()
     // You can specify a host and credentials with this method.
     // Otherwise, the builder will look for the env vars: `DB_HOST`, `DB_NAME`, `DB_USER` and `DB_PASSWORD`.
     .with_credentials("http://localhost:8529", "db", "user", "password")
     // You can specify a authentication mode between `Basic` and `Jwt`
     // Otherwise the default value will be used (`Basic`).
     .with_auth_mode(AuthMode::Basic)
     // You can specify some operations options that will be used for every `write` operations like
     // `create`, `save` and `delete`.
     .with_operation_options(OperationOptions::default())
     // You can specify a schema path to initialize the database connection
     // Otherwise the env var `SCHEMA_PATH` or the default value `config/db/schema.yaml` will be used.
     .with_schema_path("config/db/schema.yaml")
     // If you prefer you can use your own custom schema
     .with_schema(DatabaseSchema::default())
     // The schema wil silently apply to the database, useful only if you don't use the CLI and migrations
     .apply_schema()
     // You then need to build the connection
     .build()
     .await
     .unwrap();
 ```

### Using env vars

None of the builder options are mandatory, the following works perfectly if all required environment variables are set:

 ```rust
 let db_connection = DatabaseConnection::builder()
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
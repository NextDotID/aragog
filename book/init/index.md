# Initialization

In order to work, **Aragog** needs you to define your database schema in an `schema.yaml`.

We provide [aragog_cli](https://crates.io/crates/aragog_cli) for this purpose, generating a synchronized and versioned
schema and a safe migration system.

With this schema you can initialize a *connection pool* to provide a database access required for all database operations.

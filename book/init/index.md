# Initialization

In order for **Aragog** to work you need to define your database schema in an `schema.yaml`.

We provide [aragog_cli](https://crates.io/crates/aragog_cli) for this purpose, generating a synchronized and versioned
schema and a safe migration system.

With this schema you can initialize a *connection pool* which provides database access and is required for all database operations.

# Schema Generation

[aragog_cli][CLI] provides the following basics:

## Creating a Migration

Command: `aragog create_migration MIGRATION_NAME`

Creates a new migration file in `SCHEMA_PATH/migrations/`. If the `db` folder is missing it will be created automatically.

## Launch migrations

Command: `aragog migrate`

Will launch every migration in `SCHEMA_PATH/migrations/` and update the schema according to its current version.
If there is no schema it will be generated.

> Note: ArangoDB doesn't handle transactional operations for collection, index and graph management

## Rollback migrations

Command: `aragog rollback`

Will rollback 1 migration in `SCHEMA_PATH/migrations/` and update the schema according to its current version.

Command: `aragog rollback COUNT`

Will rollback `COUNT` migration in `$CHEMA_PATH/migrations/` and update the schema according to its current version.

> The schema is generated twice:
>   - once on the file system (`schema.yaml`)
>   - once in the database, the snapshot (synchronized version)
> 
> This allows seamless deployment, as the migrations launch will check the current snapshot

## Using aragog with an exiting database

The [aragog_cli][CLI] provides a `discover` command creating a migration file for already initialized database and apply it to the schema.

[CLI]: https://crates.io/crates/aragog_cli "cli"
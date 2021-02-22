# Schema Generation

[aragog_cli](https://crates.io/crates/aragog_cli) provides the following basics:

## Creating a Migration

Command: `aragog create_migration MIGRATION_NAME`

Creates a new migration file in `SCHEMA_PATH/migrations/`. If the `db` folder is missing it will be created automatically.

## Launching migrations

Command: `aragog migrate`

Will launch every migration in `SCHEMA_PATH/migrations/` and update the schema according to its current version.
If there is no schema it will be generated.

## Rollbacking migrations

Command: `aragog rollback`

Will rollback 1 migration in `SCHEMA_PATH/migrations/` and update the schema according to its current version.

Command: `aragog rollback COUNT`

Will rollback `COUNT` migrations in `$CHEMA_PATH/migrations/` and update the schema according to its current version.

> The schema is generated twice:
>   - one on the file system (`schema.yaml`)
>   - one in the database, the snapshot (synchronized version)
> 
> This allows seamless deployment, as the migrations launch will check the current snapshot
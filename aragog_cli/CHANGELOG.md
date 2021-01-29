# Changelog

## 0.2.5

* Enabled `rocksdb` feature

## 0.2.4

* ArangoDB connection is no longer async (`request/blocking`)

## 0.2.3

* Fixed `TODO` on `Clone` implementation with `aragog`0.7.8 and `arangors` 0.4.6
* `describe` command for CLI
* Added `MigrationConfig` `Display` implementation and debug on verbose

## 0.2.2

* Fixed Index unicity issue, index deletion requires collection.
* A few optimisation

## 0.2.1

* Fixed crash on first launch

## 0.2.0

* New `check` job
* Verbose options
* The schema is stored in a specific collection document and used to synchronize the migrations and database. The schema file is still generated.

## 0.1.4

* Error handling improvement
* Few clippy fixes
* cleanup unused const

## 0.1.3

* Fixed error handling on schema path

## 0.1.2

* rustfmt fix
* Using `aragog::schema::DEFAULT_SCHEMA_PATH` as a default value if env var `SCHEMA_PATH` is missing
* Updated `--help` indications

## 0.1.1

* Migration path is `$SCHEMA_PATH/migrations` instead of `$SCHEMA_PATH/db`
* Updated `--help` indication on truncation

## 0.1.0

First version, migrations and rollback are functional.
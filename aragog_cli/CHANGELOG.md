# Changelog

## Unreleased

* `aragog` dependency bumped to 0.13

## 0.3.0

* `arangors` dependency bumped to `=0.4.8`
* Clippy fixes
* Added `discover` command
* Added a Dockerfile

## 0.2.12

* `aragog` dependency from 0.9 to 0.10

## 0.2.11

* Fixed `desribe` command always giving 0 document count.

## 0.2.10

* `aragog` dependency with `minimal_traits` feature to avoid useless traits

## 0.2.9

* `aragog` dependency from 0.8 to 0.9

Fixes:
  
* updated `arangors` dependency requirement to >= `0.4.6` to avoid build issues

## 0.2.8

* `aragog` dependency from 0.7 to 0.8

## 0.2.7

* `create_collection` and `create_edge_collection` support `wait_for_sync` optional_attribute

## 0.2.6

* Internal small refactoring (`MigrationError` to `AragogCliError`)
* `describe_index` subcommand

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
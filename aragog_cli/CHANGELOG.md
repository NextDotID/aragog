# Changelog

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
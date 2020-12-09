# Changelog

## 0.1.2

* rustfmt fix
* Using `aragog::schema::DEFAULT_SCHEMA_PATH` as a default value if env var `SCHEMA_PATH` is missing
* Updated `--help` indications

## 0.1.1

* Migration path is `$SCHEMA_PATH/migrations` instead of `$SCHEMA_PATH/db`
* Updated `--help` indication on truncation

## 0.1.0

First version, migrations and rollback are functional.
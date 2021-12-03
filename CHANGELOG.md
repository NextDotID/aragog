# Changelog

## Unreleased

* Rust 2021 edition
* `openssl` explicit cargo feature
* Improved book
* Clippy restrictions
* removed deprecated `Transaction::pool` method. Use `Transaction::database_connection`

## 0.15.0

* Using `arangors_lite` instead of `arangors` driver:
* Simplified feature gates:
  * Removed `async_rustls` and `blocking` rustls features
  * Added `rustls` feature

## 0.14.0

* bumped `arangors` to 0.5
  * (**BREAKING**) `tokio` dependency is no longer 0.2.x but 1.x
* Support for rust >= `1.46` with a new CI job
* (**BREAKING**) Renamed `ServiceError` into `Error`

## 0.13.2

* Added status check for transaction `commit` and `abort` 
* Resolved a few `TODO` elements

## 0.13.1

### Added

* `QueryResult::first_record` method
* `Validate::SIMPLE_EMAIL_REGEX` const string literal 
* `Validate::RFC_5322_EMAIL_REGEX` const string literal

### Changed

* `regex` dependency bumped to 1.5
* `Record::collection_name()` static method is now a `const COLLECTION_NAME`, this is breaking only if you implement the `Record` trait directly instead of the derive macro 

## 0.13.0

### Changed

* `schema::IndexSchema` and `schema::GraphSchema` are cloned on application
* (Perf) Transaction building clones the collection accessors with the transaction id header instead
of requesting them.
* Added test on authentication failure with `AuthMode::Jwt`  
* `arangors` 0.4.8
* (**BREAKING**) Merged `JsonQueryResult` and `RecordQueryResut` into `QueryResult`.
* `DatabaseRecord::get` and `Record::get` now try to deserialize the ArangoDB result directly instead of ignoring potential parsing errors.
  > To handle corrupt documents or parsing errors a `UndefinedRecord` generic struct was added (see the **Added** section)

### Fixed

* Fixed Clippy errors and warning (Rust 1.53 support).
* Fixed `ServiceError` sources. 

### Removed

* (**BREAKING**) `actix` feature removed
* (**BREAKING**) `open-api` feature removed

### Added

* optional source to `ServiceError::Unauthorized` and `ServiceError::Forbidden`
* `ServiceError::Conflict`
* Enums can now derive `Record`
* Batch queries (see book) :  
  - `QueryCursor` struct
  - query in batch methods for `DatabaseAccess`, `DatabaseRecord`, `Record` and `Query`
  - `Query` now have a `raw_call` variant to query `UndefinedRecord` instead of typed `Record`  structs
* `UndefinedRecord` struct wrapping `serde_json::Value` allowing safe an modular querying  

## 0.12.0

* Bumped `aragog_macros` to `0.6` (see its [CHANGELOG](aragog_macros/CHANGELOG.md))
* (**BREAKING**) Renaming:
  - `DatabaseConnectionPool` is now `DatabaseConnection`
  - `TransactionPool` is now `TransactionDatabaseConnection`
* (**BREAKING**) Dropped `EdgeRecord` trait, which is now a **struct** wrapping any `Record` of an ArangoDB *Edge Collection* (see [book section](book/edge_record_struct/index.md))
* Added new structure `OperationOptions` allowing to customize behavior regarding:
  - ArangoDB revision system
  - Hooks
  - Wait for database synchronization
  
  These custom options bring new methods in `DatabaseRecord` (see the [book section](book/record_trait/index.md):
  - `create_with_options`
  - `save_with_options`
  - `delete_with_options`

  And can be set globally on the `DatabaseConnection` on start. (see the [book section](book/init/db_connection.md))
* (**BREAKING**) Removed the `password_hashing` feature, irrelevant in this library and the `argonautica` crate not being maintained.
* Deprecated `Transaction::pool()` method, prefer the new `Transaction::database_connection()` method.
* **Aragog** now forbids unsafe code and denies compilation warning
* Various documentation, tests and clippy related improvements

## 0.11.1

* bumped `aragog_macro` dependency to 0.5.0

## 0.11.0

* Book improvements:
  - Syntax
  - Fixed external links
  - Removed todo lists in favor of [gitlab issues](https://gitlab.com/qonfucius/aragog/-/issues)
* Added a `create` method for `Record` trait (wraps `DatabaseRecord::create`)
* `DatabaseCollection` improved:  
  - (**BREAKING**) removed `collection_name` field
  - Added `name()` method
  - Implemented `Deref` with `arangors::Collection<ReqwestClient>` target
* `DatabaseAccess` refactoring:
  - (**BREAKING**) The `get_collection` method now returns a `DatabaseCollection` instead of the `arangors` `Collection`
  - Added a `collection` method
  - (**BREAKING**) `TransactionPool` and `DatabaseConnectionPool` fields are private.
* Added `TransactionResult::expect` method
* Support for `DatabaseAccess` used as a trait object throughout the complete API. (see [boxed_example](examples/boxed_example))

## 0.10.1

* (**FIX**) Errors have a correct http code (actix compatibility fixed)

## 0.10.0

* Error System
  - Complete mapping of ArangoDB error codes
  - Complete mapping of ArangoDB Http error codes
  - (**BREAKING**) Changed some `ServiceError` variants

* `DatabaseRecord`:
  - (**BREAKING**) `key`,`id` and `rev` fields are now private
  - Added getters for now private fields
  - (**BREAKING**) dropped legacy `get_id` method
  - (**BREAKING**) `create` method requires a mutable record
  - Added `force_create`, `force_save` and `force_delete` to skip hooks
  - Updated and fixed some obsolete doc comments
  - (**FIX**) `link` method correctly launches creation hooks
  
* Transactions
  - (**BREAKING**) Dropped `new_with_option` function
  - New `TransactionBuilder` object for custom transactions
  - `Record` now implements restricted transaction creation methods:
    - `transaction`
    - `transaction_builder`
  
* Book `mdbook` support, every 
* examples update

## 0.9.1

* `DatabaseRecord<T>` implements `Deref` and `DerefMut` for `T`
* Removed `DatabaseRecord::authenticate` method, it doesn't break current code with the new `Deref` implementation
* Updated documentation and Readme

## 0.9.0

Features:

* `DatabaseRecord<T>` new methods `reload` and `reload_mut`
* (**BREAKING**) `DatabaseRecord::save` and `create` no longer require `Validate` implementation and don't automatically launch validations.
  Use hooks instead (see the [book](./book/record.md))
* `Record` hooks (see [book](./book/record.md)):
  - **before_save_hook**
  - **before_create_hook**
  - **after_save_hook**
  - **after_create_hook**
* `aragog_macros` `0.3` (see [CHANGELOG](./aragog_macros/CHANGELOG.md)):
  - (**BREAKING**)`EdgeRecord` derive macro no longer implements `Record`
  - `Record` derive macro `hook` extension attribute to define hooks
  
Fixes:
    
- updated `arangors` dependency requirement to >= `0.4.6` to avoid build issues

Note: Two breaking changes from 0.8 to 0.9

## 0.8.0

* `aragog_macros` `0.2` (see [CHANGELOG](./aragog_macros/CHANGELOG.md)):
  - `Validate` derive proc macro attributes
    - `validate` field attributes
    - `validate` extension attributes
  - Updated examples and tests using the new macros
* Transaction support:
  - New `Transaction` and `TransactionPool` object
  - All Database related function can take a `TransactionPool` through `DatabaseAccess`
  - New example
* `EdgeRecord` is only available for structs implementing `Record`
* Added official book and cleaned the `README`

Note: No breaking changes from 0.7 to 0.8

## 0.7.9

* Collection schemas support `wait_for_sync` attribute

## 0.7.8

* Schema types implement `Clone`
* `arangors` 0.4.6 available

## 0.7.7

Features:
* Some traits can be opted out with `minimal_traits` feature
* Documentation improved
* Fixed some Database Record wrong doc examples
* `DatabaseRecord` derives `Clone` and implements `Display`

Repository:  
* Updated CI configuration

## 0.7.6

* Added debug logging on database service functions
* Added debug logging on AQL execution

## 0.7.5

* Reworked tests to work with the blocking feature
* Fixed CI issue with rusfmt
* Fixed wrong imports for Query `fmt` implementation thanks to `serde 1.0.119`

## 0.7.4

* Small improvements on schema application methods
* `IndexSchema` doesn't store its `id`, "collection/name" is used for deletion.

## 0.7.3

* Fixed `IndexSchema` application
* More verbose information on db connection on `DatabaseConnectionPool`
* Few clippy fixes
* small cleanup

## 0.7.2

* default schema path is a directory
* Added a default schema file name

## 0.7.1

* default schema path is made public in `aragog::schema` module

## 0.7.0

* Added a hole new `schema` module with structs for better schema serializations and apply
* `DatabaseConnectionPool` has a builder
* New CLI (`aragog_cli`)
* The schema file must be in YAML
* Moved string validations methods in `Validate`
* New features: `derive`, `async` and `blocking`

## 0.6.1

* Paperclip 0.5.0

## 0.6.0

* Added `EdgeRecord` trait representing Edge Collections
* `Record` and `EdgeRecord` can be derived
* `Query` can handle sub queries and new operations
* Added linking methods between `DatabaseRecord` through edges
* New `ForeignLink` and `Link` traits to define useful relations between models
* New Authentication mode enums for `DatabaseConnectionPool`

## 0.5.1

* Query filters can be built via comparisons (syntax cleaning no breaking changes)
* Added Paperclip feature
* Renamed actix feature

## 0.5.0

* Improved Querying:
    * now `Query` handles a complete AQL query
    * `Query` can be sorted, limited, distincted
    * Added simplifying macros
    * Query can call the database itself
* `DatabaseRecord` and `Record` methods updated:
    * `get_where` becomes `get`
    * `get` response is a `QueryResult` instead of a vector
    * `find_where` was removed, use `QueryResult::uniq()` method instead
    * `Record` can build a query with `query()` method

## 0.4.4

* `actix-web` version 3
* better cargo version handling

## 0.4.3

* Fixed `AuthorizeAction` trait to allow optional target

## 0.4.2

* `AuthorizeAction` trait

## 0.4.1

* New query builder comparisons (bolean and null comparators)
* Better Readme
* Added array `All`, `Any`, `None` filters for query

## 0.4.0

* New `is_valid` method for `Record` trait.
* New query system for `Record`.

## 0.3.2

* Fixed `ServiceError`::`NotFound` message
* On find error the Not Found message is improved

## 0.3.1

* Added new `Validate` validation helper

## 0.3.0

* Added truncation method for database connection pool
* Improved lib.rs and Readme documentation
* License is owned by Qonfucius
* Fixed some broken documentation links
* `Record` `collection_name` is a a static `&str`

## 0.2.2

* Fixed Documentation broken links

## 0.2.1

* Added new cargo feature `password_hashing`
* Added new CI job
* Added `lib.rs` documentation
* Fixed non documented elements

## 0.2.0

* Renamed AragogServiceError to ServiceError (breaking changes)
* Added direct transformation of arangors ClientError to ServiceError
* Added `actix_http_error` cargo feature for actix_web errors implementation

## 0.1.1
* Documentation fixes
* Crate contact information fixed
* Added CI configuration

## 0.1.0
* First version, minimal features available
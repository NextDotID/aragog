# Changelog

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
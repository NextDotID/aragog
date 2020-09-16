# Changelog

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
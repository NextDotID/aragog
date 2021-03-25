# Changelog

## 0.6.0

* Added a `README`
* Complete rework of derive macro attribute parsing and error handling
* (**BREAKING**) Removed `EdgeRecord` derive macro
* (**BREAKING**) `Record` derive macro attributes (hooks) no longer use the `hook` keyword:
  - `#[hook(before_save(func = "func"))]` becomes `#[before_create(func = "func")]`
* (**BREAKING**) `Validate` derive macro forbids the following functions to avoid conflict and recursion:
  - `validations`
* (**BREAKING**) `Record` derive macro forbids the following functions to avoid conflict and recursion:
  - `before_create_hook`
  - `before_save_hook`
  - `before_delete_hook`
  - `before_write_hook`
  - `before_all_hook`
  - `after_create_hook`
  - `after_save_hook`
  - `after_delete_hook`
  - `after_write_hook`
  - `after_all_hook`
* Added new `Validate` derive macro attribute `validate_each` allowing to perform validation operation on an iterator.
* Added the following `Validate` derive macro validation operations:
  - `is_some` and `is_none` for `Option<>` fields.
  - `call_validations` for validation propagation on fields which type implements `aragog::Validate`
  - `max_count`, `min_count`, `count` to validate the number of elements in a `Vec<>`
* (**FIX**) The following validation operations now correctly support types that can't be safely cast to `i32`:
  - All numeric types are handled
  - All types implementing `PartiaOrd` are handled
  - Custom types implementing `PartialOrd` like custom structs or enums are handled
  
## 0.5.0

* `Record` hooks implementation matches the new `?Sized` requirements of aragog 0.11.0

## 0.4.1

* (**FIXED**) Validate macro attribute `lesser_than` was not working

## 0.4.0

* Adapted `EdgeRecord` implementation to aragog 0.10
* `Record` derive attributes:
    - new `before_delete` and `after_delete` attributes
    - `before_all` and `after_all` include deletion
    - new `before_write` and `after_write` for create and save only

## 0.3.0

- Added `Record` derive proc macro attribute `hook` see [book](../book/record.md)
- Added `blocking` cargo feature for synced hooks
- (**BREAKING**): `EdgeRecord` derive proc macro no longer implements `Record`

## 0.2.2

- HOTFIX: (yanking 0.2.1) validate derive macro was crashing compilation on other derive macro attributes.

## 0.2.1

- HOTFIX: (yanking 0.2.0) validate derive macro was crashing compilation on doc comments
- `Validate` derive macro attribute `func` can be field-specific

## 0.2.0

- `Validate` derive macro handles attributes see [book](../book/validate.md)

## 0.1.0

- `Record` derive macro
- `EdgeRecord` derive macro
- `Validate` derive macro
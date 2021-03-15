# Changelog

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
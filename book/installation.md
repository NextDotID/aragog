# Installation

## Aragog CLI

Install the aragog migration and schema generation command line interface with **cargo**:

`cargo install aragog_cli`

## Aragog Lib

Add to your `cargo.toml` the following:
````toml
aragog = "0.10"
````

### Cargo features

#### Async and Blocking

By default all `aragog` items are asynchronous, you can compile `aragog` in a synchronous build using the `blocking` feature:
```toml
aragog = { version = "0.10", features = ["blocking"], default-features = false }
```

You need to disable the default features. Don't forget to add the `derive` feature to use the derive macros.

#### Actix and Open API

If you use this crate with the [actix-web][actix] framework, you may want the `aragog` errors to be usable as http errors.
To do so you can add to your `cargo.toml` the following `feature`: `actix`. This will add Actix 3 dependency and compatibility

```toml
aragog = { version = "0.10", features = ["actix"] }
```

If you also want to be able to use [paperclip][paperclip], you may want `aragog` elements to be compatible.
To do so you can add to your `cargo.toml` the following `feature`: `open-api`.

```toml
aragog = { version = "0.10", features = ["actix", "open-api"] }
```

#### Password hashing

You may want `aragog` to provide a more complete `Authenticate` trait allowing to hash and verify passwords.
To do so you can add to your `cargo.toml` the following `feature`: `password_hashing`.

```toml
aragog = { version = "0.10", features = ["password_hashing"] }
```

It will add two functions in the `Authenticate` trait:

```rust
fn hash_password(password: &str, secret_key: &str) -> Result<String, ServiceError>;
fn verify_password(password: &str, password_hash: &str, secret_key: &str) -> Result<(), ServiceError>;
```

* `hash_password` will return a Argon2 encrypted password hash you can safely store to your database
* `verify_password` will check if the provided `password` matches the Argon2 encrypted hash you stored.

The Argon2 encryption is based on the [argonautica][argonautica] crate.
That crate requires the `clang` lib, so if you deploy on docker you will need to install it or define a custom image.

#### Minimal Traits

If you don't need the following traits:
* `Authenticate`
* `AuthorizeAction`
* `New`
* `Update`

You can disable them with the `minimal_traits` feature:

```toml
aragog = { version = "0.10", features = ["minimal_traits"] }
```

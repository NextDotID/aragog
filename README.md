# Aragog

[![pipeline status](https://gitlab.com/qonfucius/aragog/badges/master/pipeline.svg)](https://gitlab.com/qonfucius/aragog/commits/master)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/aragog.svg)](https://crates.io/crates/aragog)
[![aragog](https://docs.rs/aragog/badge.svg)](https://docs.rs/aragog)
[![Discord](https://img.shields.io/discord/763034131335741440.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/Xyx3hUP)
[![Gitter](https://badges.gitter.im/aragog-rs/community.svg)](https://gitter.im/aragog-rs/community)

`aragog` is a fully featured ODM and OGM library for [ArangoDB][ArangoDB] using the [arangors][arangors] driver.

The main concept is to provide behaviors allowing to map your structs with ArangoDB documents as simply an lightly as possible.
Inspired by Rails's [Active Record](https://github.com/rails/rails/tree/main/activerecord) library
`aragog` aslo provides **hooks** and **validations** for your models.

The crate also provides a powerful [AQL][AQL] querying tool allowing complex and safe ArangoDB queries in *Rust*.

## Migrations CLI

`aragog` provides a safe schema generation and migrations command line interface: [aragog_cli][CLI].

## Book and Documentation

- See the offical documentation [here]((https://docs.rs/aragog))
- See The offical [book](./book) ([published version](https://qonfucius.gitlab.io/aragog))
- See the [examples](./examples)

## Project Layout

- [`aragog`](https://crates.io/crates/aragog): The main ODM/OGM library
- [`aragog_macros`](https://crates.io/crates/aragog_macros): The derive proc macros for `aragog`
- [`aragog_cli`](https://crates.io/crates/aragog_cli): The migration and schema generation Command line interface

## Cargo features

### Async and Blocking

By default all `aragog` items are asynchronous, you can compile `aragog` in a synchronous build using the `blocking` feature:
```toml
aragog = { version = "0.11", features = ["blocking"], default-features = false }
```

You need to disable the default features. Don't forget to add the `derive` feature to use the derive macros.

### Actix and Open API

If you use this crate with the [actix-web][actix] framework, you may want the `aragog` errors to be usable as http errors.
To do so you can add to your `cargo.toml` the following `feature`: `actix`. This will add Actix 3 dependency and compatibility

```toml
aragog = { version = "0.11", features = ["actix"] }
```

If you also want to be able to use [paperclip][paperclip], you may want `aragog` elements to be compatible.
To do so you can add to your `cargo.toml` the following `feature`: `open-api`.

```toml
aragog = { version = "0.11", features = ["actix", "open-api"] }
```

### Password hashing

You may want `aragog` to provide a more complete `Authenticate` trait allowing to hash and verify passwords.
To do so you can add to your `cargo.toml` the following `feature`: `password_hashing`.

```toml
aragog = { version = "0.11", features = ["password_hashing"] }
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

### Minimal Traits

If you don't need the following traits:
* `Authenticate`
* `AuthorizeAction`
* `New`
* `Update`

You can disable them with the `minimal_traits` feature:

```toml
aragog = { version = "0.11", features = ["minimal_traits"] }
```

## License

`aragog` is provided under the MIT license. See [LICENSE](./LICENSE).

A simple lightweight ODM for [ArangoDB][ArangoDB] based on [arangors][arangors].

Special thanks to [fMeow][fMeow] creator of [arangors][arangors] and [inzanez][inzanez]

[arangors]: https://docs.rs/arangors
[argonautica]: https://github.com/bcmyers/argonautica
[ArangoDB]: https://www.arangodb.com/
[actix]: https://actix.rs/ "Actix Homepage"
[paperclip]: https://github.com/wafflespeanut/paperclip "Paperclip Github"
[CLI]: https://crates.io/crates/aragog_cli
[fMeow]: https://github.com/fMeow/
[inzanez]: https://github.com/inzanez/
[AQL]: https://www.arangodb.com/docs/stable/aql/ "AQL"

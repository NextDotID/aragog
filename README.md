[![Logo](https://gitlab.com/qonfucius/aragog/-/snippets/2090578/raw/master/logo.svg)](http://aragog.rs)

# Aragog

[![pipeline status](https://gitlab.com/qonfucius/aragog/badges/master/pipeline.svg)](https://gitlab.com/qonfucius/aragog/commits/master)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/aragog.svg)](https://crates.io/crates/aragog)
[![aragog](https://docs.rs/aragog/badge.svg)](https://docs.rs/aragog)
[![dependency status](https://deps.rs/crate/aragog/0.16.1/status.svg)](https://deps.rs/crate/aragog)

[![Discord](https://img.shields.io/discord/763034131335741440.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/Xyx3hUP)
[![Gitter](https://badges.gitter.im/aragog-rs/community.svg)](https://gitter.im/aragog-rs/community)

`aragog` is a fully featured ODM and OGM library for [ArangoDB][ArangoDB] using the [arangors][arangors] driver.

The main concept is to provide behaviors allowing to map your structs with ArangoDB documents as simply an lightly as possible.
Inspired by Rails's [Active Record](https://github.com/rails/rails/tree/main/activerecord) library
`aragog` also provides **hooks** and **validations** for your models.

The crate also provides a powerful [AQL][AQL] querying tool allowing complex and safe ArangoDB queries in *Rust*.

## Migrations CLI

`aragog` provides a safe schema generation and migrations command line interface: [aragog_cli][CLI].

## Book and Documentation

- See the [official website](http://aragog.rs)
- See the [official documentation](https://docs.rs/aragog)
- See the [official book](./book) ([published version](https://aragog.rs/book))
- See the [examples](./examples)

## Project Layout

- [`aragog`](https://crates.io/crates/aragog): The main ODM/OGM library
- [`aragog_macros`](https://crates.io/crates/aragog_macros): The derive proc macros for `aragog`
- [`aragog_cli`](https://crates.io/crates/aragog_cli): The migration and schema generation Command line interface

## License

`aragog` is provided under the MIT license. See [LICENSE](./LICENSE).

A simple lightweight ODM for [ArangoDB][ArangoDB] based on [arangors][arangors].

[arangors]: https://docs.rs/arangors
[argonautica]: https://github.com/bcmyers/argonautica
[ArangoDB]: https://www.arangodb.com/
[CLI]: https://crates.io/crates/aragog_cli
[AQL]: https://www.arangodb.com/docs/stable/aql/ "AQL"

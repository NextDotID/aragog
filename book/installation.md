# Installation

## Aragog CLI

Install the aragog migration and schema generation command line interface with **cargo**:

`cargo install aragog_cli`

## Aragog Lib

Add to your `cargo.toml` the following:
````toml
aragog = "0.11"
````

### Cargo features

#### Async and Blocking

By default all `aragog` items are asynchronous, you can compile `aragog` in a synchronous build using the `blocking` feature:
```toml
aragog = { version = "0.11", features = ["blocking"], default-features = false }
```

You need to disable the default features. Don't forget to add the `derive` feature to use the derive macros.

#### Actix and Open API

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

#### Minimal Traits

If you don't need the following traits:
* `AuthorizeAction`
* `New`
* `Update`

You can disable them with the `minimal_traits` feature:

```toml
aragog = { version = "0.11", features = ["minimal_traits"] }
```

[actix]: https://actix.rs/ "Actix Homepage"
[argonautica]: https://github.com/bcmyers/argonautica
[paperclip]: https://github.com/wafflespeanut/paperclip "Paperclip Github"
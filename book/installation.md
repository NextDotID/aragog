# Installation

## Aragog CLI

Install the aragog migration and schema generation command line interface with **cargo**:

`cargo install aragog_cli`

## Aragog Lib

Add to your `cargo.toml` the following:
````toml
aragog = "0.15"
````

### Cargo features

#### Async and Blocking

By default, all `aragog` items are asynchronous, you can compile `aragog` in a synchronous build using the `blocking` feature:
```toml
aragog = { version = "0.15", features = ["blocking"] }
```

#### OpenSSL and Rustls

`aragog` uses `reqwest` to query ArangoDB. By default, OpenSSL is used, but you can compile `aragog` to use rustls using the `rustls` feature:
```toml
aragog = { version = "0.15", features = ["rustls"], default-features = false }
```

You need to disable the default features. Don't forget to add the `derive` feature to use the derive macros.

#### Minimal Traits

If you don't need the following traits:
* `AuthorizeAction`
* `New`
* `Update`

You can disable them with the `minimal_traits` feature:

```toml
aragog = { version = "0.15", features = ["minimal_traits"] }
```

[actix]: https://actix.rs/ "Actix Homepage"
[argonautica]: https://github.com/bcmyers/argonautica
[paperclip]: https://github.com/wafflespeanut/paperclip "Paperclip Github"
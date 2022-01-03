[![Logo](https://gitlab.com/qonfucius/aragog/-/snippets/2090578/raw/master/logo.svg)](http://aragog.rs)

# Aragog Macros

[![pipeline status](https://gitlab.com/qonfucius/aragog/badges/master/pipeline.svg)](https://gitlab.com/qonfucius/aragog/commits/master)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/aragog_macros.svg)](https://crates.io/crates/aragog_macros)
[![dependency status](https://deps.rs/crate/aragog-macros/0.7.3/status.svg)](https://deps.rs/crate/aragog-macros)

[![Discord](https://img.shields.io/discord/763034131335741440.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/Xyx3hUP)
[![Gitter](https://badges.gitter.im/aragog-rs/community.svg)](https://gitter.im/aragog-rs/community)

Procedural macros utility for [aragog](http://aragog.rs) ([crates.io](https://crates.io/crates/aragog)).

## Record derive macro

`aragog_macros` allows to derive `aragog::Record` instead of direct implementation. (see the [book section](../book/record_trait/index.md))

Available attributes:
- *before_create* 
- *before_save* 
- *before_delete* 
- *before_write* 
- *before_all* 
- *after_create* 
- *after_save* 
- *after_delete* 
- *after_write* 
- *after_all* 

All these attributes are Record [hooks](../book/record_trait/hooks.md).

## Validate derive macro

`aragog_macros` allows to derive `aragog::Validate` instead of direct implementation. (see the [book section](../book/validate_trait/index.md))

Available attributes:
- *validate*
- *validate_each*
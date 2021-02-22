
# Technical notes

## Direct implementation

You can implement `EdgeRecord` directly instead of deriving it.

> We strongly suggest you derive the `EdgeRecord` trait instead of implementing it.
> you would loose the compiler check on the required field presence.

You need to specify the `_from()` and `_to()` methods which, when deriving, are automatically filled.
You also need to implement or derive `Record` (see [previous section](./record.md))

Example:
```rust
use aragog::{Record, EdgeRecord};

#[derive(Serialize, Deserialize, Clone, Record)]
pub struct ChildOf {
    pub _from: String,
    pub _to: String,
}

impl EdgeRecord for ChildOf {
    fn _from(&self) -> String { self._from.clone() }

    fn _to(&self) -> String { self._to.clone() }
}
```

## Todo list

Nothing here for the moment.

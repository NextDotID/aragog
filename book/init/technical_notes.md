
# Technical notes

## `DatabaseAccess` trait

The object `DatabaseConnection` is the default handler for every database operations but you can create your own.

Every operation taking the connection as an argument is taking a **Generic** type implementing `DatabaseAccess`,
so you can implement it on your own struct.

> Note: This is not recommended, modification to `DatabaseAccess` can happen without considering them as **breaking**.

### Trait object

All `aragog` API using a `DatabaseAccess` type also use `?Sized` (`Sized` [restriction relaxation](https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait))

This means you can use dynamically typed [trait objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html).

Example extract from [boxed example](https://gitlab.com/qonfucius/aragog/-/tree/master/examples/boxed_example):

```rust
pub struct BoxedConnection {
    pub connection: Box<dyn DatabaseAccess>,
}

impl BoxedConnection {
    pub fn connection(&self) -> &dyn DatabaseAccess {
        self.connection.deref()
    }
}
```

## Database truncation

The `DatabaseConnection` provides a `truncate_database` method but you should use it only for testing purposes,
it is highly destructive as it will drop every collections known to the connection.
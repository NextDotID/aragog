
# Technical notes

## `DatabaseAccess` trait

The object `DatabaseConnectionPool` is the default handler for every database operation but you can create your own.

Every operation taking the pool as an argument is taking a **Generic** type implementing `DatabaseAccess`,
so you can implement it on your own struct.

> Note: This is not recommended, and modification to `DatabaseAccess` can happen without considering them as **breaking**.

## `truncate_database`

The `DatabaseConnectionPool` provides a `truncate_database` method but you should use it only for testing purposes,
it is highly destructive as it will drop every collection known to the pool.

## Todo list

- [ ] Add `with_host` builder option to remove the host from `with_credentials`
- [ ] Add `with_database` builder option to remove the database name from `with_credentials`
- [ ] Optional implementation of the `r2d2` pool pattern
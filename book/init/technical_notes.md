
# Technical notes

## `DatabaseAccess` trait

The object `DatabaseConnectionPool` is the default handler for every database operations but you can create your own.

Every operation taking the pool as an argument is taking a **Generic** type implementing `DatabaseAccess`,
so you can implement it on your own struct.

> Note: This is not recommended, modification to `DatabaseAccess` can happen without considering them as **breaking**.

## `truncate_database`

The `DatabaseConnectionPool` provides a `truncate_database` method but you should use it only for testing purposes,
it is highly destructive as it will drop every collections known to the pool.
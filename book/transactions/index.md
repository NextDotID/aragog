# Transactions

`aragog` supports transactional operations without using a specific API thanks to the new `Transaction` Object.

## Creating a transaction

To initiate a transaction we need to use the `DatabaseConnectionPool`, as the transaction will create an equivalent `DatabaseAccess` object:
The `TransactionPool`, that can be used instead of the classic pool to use the transactional features.

Example:
```rust
// We build the pool
let database_pool = DatabaseConnectionPool::builder()
         .build().await.unwrap();
// We instantiate a new transaction
let transaction = Transaction::new(&database_pool).await.unwrap();
```

## Transaction states

An ArangoDB transaction has three states:
- *Running*
- *Committed*
- *Aborted*

After successfully initializing a `Transaction` Object, a *Running* transaction is created.
We can now use its pool:

Example:
````rust
let database_pool = DatabaseConnectionPool::builder()
         .build().await.unwrap();
// Instantiate a new transaction
let transaction = Transaction::new(&database_pool).await.unwrap();
// Retrieve the pool
let transaction_pool = transaction.pool();
// We use the transaction pool instead of the classic pool
DatabaseRecord::create(
    Dish {
        name: "Pizza".to_string(),
        price: 10,
    },
    transaction_pool
).await.unwrap();
// We commit the transaction
transaction.commit().await.unwrap();
````

The **create** operations is using the transaction, meaning it won't be written in ArangoDB until the transaction is committed.
The operation will simply be cancelled if the transaction is aborted.

> Make sure to always commit or abort a transaction !

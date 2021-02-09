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

## Safe execution

To avoid remembering to commit and maually handling when to abort a transaction, prefer using the safe execution. 

The safe execution allows to execute multiple operations in a block and make sure the transaction is either *committed* or *aborted*.

 ```rust
 
let database_pool = DatabaseConnectionPool::builder()
.build().await.unwrap();
// Instantiate a new transaction
let transaction = Transaction::new(&database_pool).await.unwrap();
// Safely execute operations:
let output = transaction.safe_execute(|transaction_pool| async move {
    // We use the provided `transaction_pool` instead of the classic pool
    DatabaseRecord::create(Dish {
        name: "Pizza".to_string(),
        price: 10,
    }, &transaction_pool).await?;
    DatabaseRecord::create(Dish {
        name: "Pasta".to_string(),
        price: 8,
    }, &transaction_pool).await?;
    DatabaseRecord::create(Dish {
        name: "Sandwich".to_string(),
        price: 5,
    }, &transaction_pool).await?;
    // You can return any type of data here
    Ok(())
}).await.unwrap();
// The output allows to check the transaction state: Committed or Aborted
assert!(output.is_committed());
```

If an operation fails in the `safe_execute` block the transaction will be aborted and every operation cancelled.

> Don't use `unwrap()` or any panicking functions in the block as the transaction won't be aborted.

The `safe_execute` method returns a `TransactionOutput` if everything went correctly (No Database or connection errors).
This output allows to check the state of the transaction, *Aborted* or *Committed* and retrieve the result of the block
stored as a generic.

> Note: Transactions can be committed multiple times, so feel free to use multiple safe execution blocks.

> Warning: An aborted transaction can no longer be committed ! Make sure to handle the `TransactionOuput` cases.

## Technical notes

The transactions use the ArangoDB [steam transaction API](https://www.arangodb.com/docs/stable/http/transaction-stream-transaction.html).

### Todo list

Nothng here yet.
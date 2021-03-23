
# Safe execution

To avoid remembering to commit and manually handling when to abort a transaction, prefer using the safe execution.

The safe execution allows to execute multiple operations in a block and make sure the transaction is either *committed* or *aborted*.

 ```rust
let database_connection = DatabaseConnection::builder()
.build().await.unwrap();
// Instantiate a new transaction
let transaction = Transaction::new(&database_connection).await.unwrap();
// Safely execute operations:
let output = transaction.safe_execute(|transaction_connection| async move {
    // We use the provided `transaction_connection` instead of the classic connection
    DatabaseRecord::create(Dish {
        name: "Pizza".to_string(),
        price: 10,
    }, &transaction_connection).await?;
    DatabaseRecord::create(Dish {
        name: "Pasta".to_string(),
        price: 8,
    }, &transaction_connection).await?;
    DatabaseRecord::create(Dish {
        name: "Sandwich".to_string(),
        price: 5,
    }, &transaction_connection).await?;
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

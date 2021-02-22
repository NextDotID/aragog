# Custom transactions

The `Transaction` object implements a builder pattern through `TransactionBuilder`

## Restricted transactions

The `Transaction::new` pattern build a valid transaction for *all collections* (defined in the schema).
You may want more restricted transactions, limited to a single **Collection**.

All structs deriving `Record`, here *User*, have access to:
- `User::transaction` building a transaction on this collection only.
- `User::transaction_builder` returning a builder for a transaction on this collection only.

Restricted transaction will fail if you try to **Write** on an other collection.

## Transaction options

The `TransactionBuilder` allows optional parameters:
- `wait_for_sync`, forcing the transaction to be synced to the database on **commit**.
- `lock_timeout`, specifying the transaction lock timeout set to **60000** by default
- `collections`, specifying the allowed collections (by default all collections are allowed)

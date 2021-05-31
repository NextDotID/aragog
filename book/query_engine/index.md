# The Query system

You can retrieve document from the database two ways:
- from the unique ArangoDB `_key` (see the [record](../record_trait/index.md) section)
- from an [AQL](https://www.arangodb.com/docs/stable/aql/index.html) query

`aragog` provides an AQL query builder system, allowing safer queries than direct string literals.

For a created object like the following:

 ```rust
#[derive(Serialize, Deserialize, Record, Clone)]
struct User {
    first_name: String,
    last_name: String,
    age: u16,
}

let user = User {
    first_name: "Robert".to_string(),
    last_name: "Surcouf".to_string(),
    age: 25,
};
DatabaseRecord::create(user, &database_connection).await.unwrap();
```

You can define a query:

```rust
let query = User::query()
    .filter(Filter::new(
        Comparison::field("last_name").equals_str("Surcouf"))
        .and(Comparison::field("age").greater_than(15))
    );
```

## Typed querying

Typed querying will allow only **one** type of document to be retrieved, in this case *User* collection documents.

> In case of corrupted documents they may not be returned, see safe querying for resilient querying

- Through `Record::get`:
```rust
 let result = User::get(query, &database_connection).await.unwrap();
```

- Through `DatabaseRecord::get` (requires type):
```rust
 let result :QueryResult<User> = DatabaseRecord::get(query, &database_connection).await.unwrap();
```

- Through `Query::call` (requires type):
```rust
 let result :QueryResult<User> = query.call(&database_connection).await.unwrap()
```

## Safe querying

safe querying will allow **multiple** types of document to be retrieved as json objects (`UndefinedRecord`) and then dynamically parsed.

> This version may be slightly slower, but you have a guarantee to retrieve all documents

- Through `Query::raw_call`:
```rust
 let result = query.raw_call(&database_connection).await.unwrap()
```

- Through `DatabaseAccess::query` (requires type):
```rust
 let result = database_connection.query(query).await.unwrap();
```

The `QueryResult<UndefinedRecord>` provides a `get_records` method to dynamically retrieve custom `Record` types.

## Batch calls

Each and every query variant shown above have a **batched** version:

- `Record::get` => `Record::get_in_batches`
- `DatabaseRecord::get` => `DatabaseRecord::get_in_batches`
- `Query::call` => `Query::call_in_batches`
- `Query::raw_call` => `Query::raw_call_in_batches`
- `DatabaseAccess::query` => `DatabaseAccess::query_in_batches`

They will return a `QueryCursor` instead of a `QueryResult` allowing to customize the number of returned document and easy iteration through the returned batches.

> If you use the `blocking` feature, `QueryCursor` has an `Iterator` implementation.
> Otherwise use the `next_batch` method
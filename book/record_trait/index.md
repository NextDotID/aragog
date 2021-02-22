# The `Record` trait

This trait defines the ODM (Object-Document mapper) of `aragog`.
Every structure implementing this trait becomes a **Model** that can be mapped to a ArangoDB [collection document](https://www.arangodb.com/docs/stable/data-modeling-documents-document-methods.html#document).

When declaring a model like the following:

```rust
use aragog::Record;

#[derive(Serialize, Deserialize, Clone, Record)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: usize
}
```

To derive `Record` your structure needs to derive or implement `Serialize`, `Deserialize` and  `Clone` which are needed
to store the document data.

We don't specify the `_key` field, as we describe the document's **data**;

> Note: An ArangoDB document is identified by its `_key` field which is its primary identifier and `_id` and `_rev` fields not yet used by `aragog`.

Now `User` can be written and retrieved for the database collection **"User"**

## Synced documents

To create a document in the database we need to use the `aragog` generic struct `DatabaseRecord<T>`.

`DatabaseRecord` describes a document synchronized with the database:

 ```rust
// The User document data
let user = User {
    username: String::from("LeRevenant1234"),
    first_name: String::from("Robert"),
    last_name: String::from("Surcouf"),
    age: 18
};
// We create the document on the database collection "User", returning a `DatabaseRecord<User>`
let mut user_record = DatabaseRecord::create(user, &database_pool).await.unwrap();
// We can now access the unique `_key` of the document
let document_key = user_record.key();
// The key can be used to retrieve documents, returning again a `DatabaseRecord<User>`
let found_user = User::find(document_key, &database_pool).await.unwrap();
// We can access the document data from the database record
assert_eq!(user.username, found_user.username);
 ```

- `key` is the primary identifier of the document, certifying it's written in the database collection
- `record` is the document data, a generic containing your struct implementing the `Record` trait

### Document operations

Documents can be:

- **created** with `DatabaseRecord::create`
- **retrieved** with `YourRecord::find` or `DatabaseRecord::find` (not recommended)
- **saved** with `DatabaseRecord::save`
- **deleted** with `DatabaseRecord::delete`

The `DatabaseRecord` structure wraps all ODM operations for any struct implementing `Record`

Complete Example:
 ```rust
 use aragog::{Record, DatabaseConnectionPool, DatabaseRecord, Validate, AuthMode};
 use serde::{Serialize, Deserialize};
 use tokio;

 #[derive(Serialize, Deserialize, Clone, Record)]
 pub struct User {
     pub username: String,
     pub first_name: String,
     pub last_name: String,
     pub age: usize
 }

 #[tokio::main]
 async fn main() {
     // Database connection Setup
     let database_pool = DatabaseConnectionPool::builder()
         .build()
         .await
         .unwrap();
     // Define a document
     let mut user = User {
         username: String::from("LeRevenant1234"),
         first_name: String::from("Robert"),
         last_name: String::from("Surcouf"),
         age: 18
     };
     // We create the user
     let mut user_record = DatabaseRecord::create(user, &database_pool).await.unwrap();
     // You can access and edit the document
     user_record.username = String::from("LeRevenant1524356");
     // And directly save it
     user_record.save(&database_pool).await.unwrap();
     // Or delete it
     user_record.delete(&database_pool).await.unwrap();
 }
 ```
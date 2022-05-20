# The `Record` trait

This trait defines `aragog` ODM (Object-Document mapper).
Every type implementing this trait becomes a **Model** that can be mapped to an ArangoDB [collection document](https://www.arangodb.com/docs/stable/data-modeling-documents-document-methods.html#document).

> Note: enums don't work as records but can be record fields

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

## Custom collection name

By default, the collection name associated with the model will be the same. A `User` struct deriving `Record` will be stored in a `User` collection (case sensitive).

In the case were your model and collection name don't match you can specify a `collection_name` attribute along with the *derive* macro:

```rust
use aragog::Record;

#[derive(Serialize, Deserialize, Clone, Record)]
#[collection_name = "Users"]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: usize
}
```

In this example, the `User` models will be synced with the `Users` collection.

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
let mut user_record = DatabaseRecord::create(user, &database_connection).await.unwrap();
// We can now access the unique `_key` of the document
let document_key = user_record.key();
// The key can be used to retrieve documents, returning again a `DatabaseRecord<User>`
let found_user = User::find(document_key, &database_connection).await.unwrap();
// We can access the document data from the database record
assert_eq!(user.username, found_user.username);
 ```

- `key` is the document primary identifier, certifying write action in the database collection
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
 use aragog::{Record, DatabaseConnection, DatabaseRecord, Validate, AuthMode};
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
     let database_connection = DatabaseConnection::builder()
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
     let mut user_record = DatabaseRecord::create(user, &database_connection).await.unwrap();
     // You can access and edit the document
     user_record.username = String::from("LeRevenant1524356");
     // And directly save it
     user_record.save(&database_connection).await.unwrap();
     // Or delete it
     user_record.delete(&database_connection).await.unwrap();
 }
 ```

#### Operation options

All the **write** operations (create, save and delete) provide a variant `_with_option`:
- `create_with_options`
- `save_with_options`
- `delete_with_options`

These methods allow to customize some aspects of the operation:
- `wait_for_sync`: Should aragog wait for the operations to be written on disk? (by default the collection behavior is kept)
- `ignore_revs`: Should ArangoDB ignore the revision conflict (`true` by default)
- `ignore_hooks`: Should the operation skip the related *Hooks* ?

These options are available but you should use them sparingly. Prefer defining a global option settings directly
in the [DatabaseConnection](../init/db_connection.md) if you find yourself in a situation where you want:
- To **always** or **never** wait for sync
- To **always** or **never** ignore the revision system
- To **always** skip the hooks

Keep in mind that all **write** operations also have `force_` variants which:
- explicitly ignore the revision system
- explicitly ignore the hooks

No matter what the global options are.
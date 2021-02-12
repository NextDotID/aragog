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
let document_key = &user_record.key;
// The key can be used to retrieve documents, returning again a `DatabaseRecord<User>`
let found_user = User::find(document_key, &database_pool).await.unwrap();
// We can access the document data from the database record
assert_eq!(user.username, found_user.record.username);
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
     user_record.record.username = String::from("LeRevenant1524356");
     // And directly save it
     user_record.save(&database_pool).await.unwrap();
     // Or delete it
     user_record.delete(&database_pool).await.unwrap();
 }
 ```

### Hooks

The `Record` trait provides the following hooks:
- `before_create` : executed before document creation (`DatabaseRecord::create`)
- `before_save` : executed before document save (`Record::save`)
- `before_all` : executed before both document creation **and** save.
- `after_create` : executed after the document creation (`DatabaseRecord::create`)
- `after_save` : executed after the document save (`Record::save`)
- `after_all` : executed after both documet creation **and** save.

You can register various methods in these hooks with the following syntax:
```rust
#[hook(before_create(func = "my_method"))]
```

The hooked methods can have follow various patterns using the following options:
- `is_async` the method is async
- `db_access` the method uses the db access

By default both these options are set to `false`.

You can combine the options to have an `async` hook with db access, allowing to execute document operations automatically.
If you combine a lot of operations, like creating documents in hooks or chaining operations make sure to:
- avoid **circular operations**
- use [Transaction](./transactions.md) for safety

#### Hook Patterns

##### The simple hook with no options
```rust
#[hook(before_create(func = "my_method"))]
```
*my_method* can be either:
- ```rust 
  fn my_method(&self) -> Result<(), aragog::ServiceError>
  ```
- ```rust 
  fn my_method(&mut self) -> Result<(), aragog::ServiceError>
  ```

##### The async hook
```rust
#[hook(before_create(func = "my_method", is_async = true))]
```
*my_method* can be either:
- ```rust 
  async fn my_method(&self) -> Result<(), aragog::ServiceError>
  ```
- ```rust 
  async fn my_method(&mut self) -> Result<(), aragog::ServiceError>
  ```

> If you use `aragog` with the `blocking` feature then this option will have no effect.


##### The hook with database access
```rust
#[hook(before_create(func = "my_method", db_access = true))]
```
*my_method* can be either:
- ```rust 
  fn my_method<D>(&self, db_access: &D) -> Result<(), aragog::ServiceError> where D: aragog::DatabaseAccess
  ```
- ```rust 
  fn my_method<D>(&mut self, db_access: &D) -> Result<(), aragog::ServiceError> where D: aragog::DatabaseAccess
  ```

> If you want to use the database access, using also `is_async = true` would be recommended


#### The `Validate` derive case

If you derive the [Validate](./validate.md) trait, you may want validations to be launched automatically in hooks.

```rust
#[derive(Serialize, Deserialize, Clone, Record, Validate)]
#[hook(before_all(func = "validate"))] // This hook will launch validations before `create` and `save`
 pub struct User {
     pub username: String,
     pub first_name: String,
     pub last_name: String,
     pub age: usize
 }
```

## Technical notes

### Direct implementation

You can implement `Record` directly instead of deriving it.

> We strongly suggest you derive the `Record` trait instead of implementing it, 
as in the future more hooks may be added to this trait without considering it **breaking changes**

You need to specify the `collection_name` method which, when deriving, takes the name of the structure.
You also need to implement directly the various hooks you need.

Example:
```rust
use aragog::Record;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: usize
}

impl Record for User {
    fn collection_name() -> &'static str { "User" }

    async fn before_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError> where
        D: DatabaseAccess {
        // Your implementation
        Ok(())
    }

    async fn after_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError> where
        D: DatabaseAccess {
        // Your implementation
        Ok(())
    }
}
```

### Unstable hooks state

The hook macros work pretty well but are difficult to test, especially the compilation errors:
Are the errors messages relevant? correctly spanned? etc.

So please report any bug or strange behaviour as this feature is still in its early stages.

### TODO list

- [X] Defining hooks (`before_save`, `before_create`, etc) and the equivalent macros
- [ ] Adding `before_delete` and `after_delete` hooks
- [ ] Allowing more modularity in return types and errors
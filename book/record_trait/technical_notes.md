# Technical notes

## The `Validate` derive case

If you derive the [Validate](../validate_trait/index.md) trait, you may want validations to be launched automatically in hooks.

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

## Direct implementation

You can implement `Record` directly instead of deriving it.

> We strongly suggest you derive the `Record` trait instead of implementing it,
as in the future more hooks may be added to this trait without considering it **breaking changes**

You need to specify the `collection_name` method which, when deriving, takes the name of the structure.
You also need to implement directly the various hooks.

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

    async fn before_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError> where
        D: DatabaseAccess {
        // Your implementation
        Ok(())
    }
    
    async fn before_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError> where
        D: DatabaseAccess {
        // Your implementation
        Ok(())
    }
    
    async fn after_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError> where
        D: DatabaseAccess {
        // Your implementation
        Ok(())
    }

    async fn after_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError> where
        D: DatabaseAccess {
        // Your implementation
        Ok(())
    }

    async fn after_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError> where
        D: DatabaseAccess {
        // Your implementation
        Ok(())
    }
}
```

## Unstable hooks state

The hook macros work pretty well but are difficult to test, especially the compilation errors:
Are the errors messages relevant? correctly spanned? etc.

So please report any bug or strange behaviour as this feature is still in its early stages.

## TODO list

- [X] Defining hooks (`before_save`, `before_create`, etc) and the equivalent macros
- [X] Adding `before_delete` and `after_delete` hooks
- [ ] Allowing more modularity in return types and errors
- [ ] Use the `_rev` check (**CRITICAL**)
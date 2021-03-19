# Technical notes

## The `Validate` derive case

If you derive the [Validate](../validate_trait/index.md) trait, you may want validations to be launched automatically in hooks.

```rust
#[derive(Serialize, Deserialize, Clone, Record, Validate)]
#[before_write(func = "validate")] // This hook will launch validations before `create` and `save`
 pub struct User {
     pub username: String,
     pub first_name: String,
     pub last_name: String,
     pub age: usize
 }
```

## Forbidden method names

When using a hook like the following:

`#[before_create(func("my_method"))]`

The called method names can't be: 

- `before_create_hook`
- `before_save_hook`
- `before_delete_hook`
- `after_create_hook`
- `after_save_hook`
- `after_delete_hook`

to avoid unexpected behaviors like unwanted recursive hooks.

> Try using explicit names for your hooks

## Direct implementation

You can implement `Record` directly instead of deriving it.

> We strongly suggest you derive the `Record` trait instead of implementing it,
as in the future more hooks may be added to this trait without considering it **breaking changes**

You need to specify the `collection_name` method which, when deriving, takes the name of the structure.
You also need to implement directly the different hooks.

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
        D: DatabaseAccess + ?Sized {
        // Your implementation
        Ok(())
    }

    async fn before_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError> where
        D: DatabaseAccess + ?Sized {
        // Your implementation
        Ok(())
    }
    
    async fn before_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError> where
        D: DatabaseAccess + ?Sized {
        // Your implementation
        Ok(())
    }
    
    async fn after_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError> where
        D: DatabaseAccess + ?Sized {
        // Your implementation
        Ok(())
    }

    async fn after_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError> where
        D: DatabaseAccess + ?Sized {
        // Your implementation
        Ok(())
    }

    async fn after_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError> where
        D: DatabaseAccess + ?Sized {
        // Your implementation
        Ok(())
    }
}
```

## Unstable hooks state

The hook macros work pretty well but are difficult to test, especially compilation errors:
Are the errors messages relevant? correctly spanned? etc.

So please report any bug or strange behaviour as this feature is still in its early stages.
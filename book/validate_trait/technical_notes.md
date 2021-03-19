# Technical notes

## `Validate` with `Record`

If you derive the [Record](../record_trait/index.md) trait, you may want validations to be launched automatically in record hooks.

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

## Enum validations

Enums can derive `Validate` but field validation attributes are not supported.

## Forbidden method name

When using a custom validation method like the following:

`#[validate(func("my_method"))]`

The called method names can't be `validations` to avoid unexpected behaviors like recursive validations.

This is caused by the `Validate` method `validations` already being built and called by the derive macro.

> Try using explicit names for your custom validation methods

If your objective is to call the validations of a compound object implementing `Validate` use the `call_validations` operation.

## Direct implementation

You can implement `Validate` directly instead of deriving it.

> We suggest you derive the `Validate` trait instead of implementing it unless you need specific operation order

You need to specify the `validations` method which, when deriving is filled with all the macro attributes

Example:
```rust
use aragog::Validate;

pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: usize
}

impl Validate for User {
    fn validations(&self, errors: &mut Vec<String>) {
        // Your validations
    }
}
```

## Unstable state

The validation macros work pretty well but are difficult to test, especially the compilation errors:
Are the errors messages relevant? correctly spanned? etc.

So please report any bug or strange behaviour as this feature is still in its early stages
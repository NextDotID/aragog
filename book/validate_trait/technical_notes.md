# Technical notes

## `Validate` with `Record`

If you derive the [Record](../record_trait/index.md) trait, you may want validations to be launched automatically in record hooks.

```rust
#[derive(Serialize, Deserialize, Clone, Record, Validate)]
#[hook(before_write(func = "validate"))] // This hook will launch validations before `create` and `save`
 pub struct User {
     pub username: String,
     pub first_name: String,
     pub last_name: String,
     pub age: usize
 }
```

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

## Todo list

- [X] make the validate trait easily compatible with `Record` hooks
- [ ] Add more field validation attributes
    - [ ] Array validations
    - [ ] `Option` validations
    - [ ] Boolean validations
    - [X] function validations of fields
- [ ] Interrupt compilation after a derive macro error
- [ ] Be able to propagate validations
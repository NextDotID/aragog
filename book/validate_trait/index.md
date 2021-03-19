# The `Validate` trait

As `aragog` is trying to be a complete ODM, being able to add validations to **models** is a useful feature.

## Macro validations

Let's take the [record](../record_trait/index.md) example model:

```rust
use aragog::{Record, Validate};

#[derive(Serialize, Deserialize, Clone, Record, Validate)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<String>,
    pub age: usize,
}
```
We added the `Validate` trait derive, making this **model** perform validations before being written to the database.

### Field validations

We can add quite some pre-made validation operations as following:
```rust
use aragog::{Record, Validate};

#[derive(Serialize, Deserialize, Clone, Record, Validate)]
pub struct User {
   // The *username* field must contain exactly 10 characters
   #[validate(length = 10)]
    pub username: String,
   // The *username* field must have at least 3 characters
   #[validate(min_length = 3)]
    pub first_name: String,
   // The *username* field must have its lenth between 3 and 30
   #[validate(min_length = 5, max_length = 30)]
    pub last_name: String,
   // Each role must have an exact length of 5
   #[validate_each(length = 5)]
   pub roles: Vec<String>,
   // The *age* field must be at least 18
   #[validate(greater_or_equal(18))]
    pub age: usize,
}
```

When trying to **create** or **save** a `User` document all validations must match 
or a `ServiceError::ValidationError` will be returned with an explicit message.

The current available field attribute validation operation macros (can be chained):
- `String` or string slice fields:
    - `length(VAL)` validates the field has exact length of *VAL*
    - `min_length(VAL)` validates the field has a minimum length of *VAL*
    - `max_length(VAL)` validates the field has a maximum length of *VAL*
    - `regex(REGEX)` validates the field matches the *REGEX* regexp
- Numeric fields or ordered types
    - `greater_than(VAL)` validated the field is greater than *VAL*
    - `greater_or_equal(VAL)` validated the field is greater or equal to *VAL*
    - `lesser_than(VAL)` validated the field is lesser than *VAL*
    - `lesser_or_equal(VAL)` validated the field is lesser or equal to*VAL*
- Other
    - `func(FUNC)` calls the *FUNC* method (see [Extra Validations](#extra-validations))
    - `call_validations` calls the validations of the field allowing to propagate the validation calls.
  The field must be an type implementing `Validate`
    
> Note: The order of the validations is not guaranteed

#### Validate comparison between custom types

The following validation operations:
  - `greater_than(VAL)`
  - `greater_or_equal(VAL)`
  - `lesser_than(VAL)`
  - `lesser_or_equal(VAL)`

Can allow custom types implementing `PartialOrd`

> Note: It also requires `Display` to generate the error, and `Copy` for the borrow checker

So you can use a custom struct or enum like this:

```rust
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
enum CustomOrd {
  A,
  B,
  C,
}

#[derive(Validate)]
struct Comparator {
  #[validate(lesser_than(CustomOrd::C), greater_than(CustomOrd::A))]
  pub field: CustomOrd,
}
```

### Extra validations

On more complex cases, simple field validations are not enough and you may want to add custom validations.

```diff
use aragog::{Record, Validate};

#[derive(Serialize, Deserialize, Clone, Record, Validate)]
+ #[validate(func("custom_validations"))] // We added this global validation attribute on top of the struct
pub struct User {
+   #[validate(length = 10, func("validate_username"))] // We added this field validation attribute
    pub username: String,
    #[validate(min_length = 3)]
    pub first_name: String,
    #[validate(min_length = 5, max_length = 30)]
    pub last_name: String,
    #[validate(greater_or_equal(18))]
    pub age: usize,
+   // These two fields require a more complex validation
+   pub phone: Option<String>,
+   pub phone_country_code: Option<String>,
}

+ impl User {
+     // We added the global custom validation method (uses multi-fields)
+     fn custom_validations(&self, errors: &mut Vec<String>) {
+         if self.phone.is_some() || self.phone_country_code.is_some() {
+             // We use built-in validation methods
+             Self::validate_field_presence("phone", &self.phone, errors);
+             Self::validate_field_presence("phone_country_code", &self.phone_country_code, erros);
+         }
+     }
+     
+     // We added the field custom validation method (field-specific)
+     fn validate_username(field_name: &str, value: &str, errors: &mut Vec<String>) {
+         if value == "SUPERADMIN" {
+             // We can push our own validation errors
+             errors.push(format!("{} can't be SUPERADMIN", field_name))
+         }   
+     }
+ }
```

The macro attribute `#[validate(func("METHOD"))]"` must link to an existing method of your struct.

This method can follow various patterns:

- global validation method (top of the struct)
```rust
fn my_method(&self, errors: &mut Vec<String>) {}
```
- field validation method
```rust
fn my_method(field_name: &str, field_value: &T, errors: &mut Vec<String>) {}
```
`T` being your field type

> Note: The method can have any visibility and can return whatever you want 

the `errors` argument is a mutable array of error messages it contains all current errors and you can push your own errors in it.
When the `validate()` method is called, this `errors` vector is used to build the error message.
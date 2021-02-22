# Aragog Book

This book is destined to cover every major feature of the `aragog` library.
Note that **everything** in the lib is documented, so don't forget to check the [technical documentation](https://docs.rs/aragog)
for more detailed information.

Missing sections:
- The relation traits (`Link`and `ForeignLink`)
- The optional traits (`New`, `Update` and `Authenticate`)


# Safe

Fully benefit Rust speed and safety with Aragog. Type safe queries, exhaustive errors and transactional operations

# Easy

Define and manipulate models, edges and graphs seamlessly. 

# Productive

Add validations, callbacks and hooked operations with zero boilerplate code.


## Models

Make a `struct` queryable with a simple `derive`

```rust
#[derive(Record, Clone, Serialize, Deserialize)]
pub struct User {
    username: String,
    name: String,
    age: u16
}
```

## Hooks

Define life cycle methods to your model

```rust
#[derive(Record, Clone, Serialize, Deserialize)]
#[hook(before_save(func = "my_method"))]
#[hook(after_delete(func = "my_other_method"))]
pub struct User {
    username: String,
    name: String,
    age: u16
}
```

## Validations

Integrate field and custom validations to your life cycle seamlessly

```rust
#[derive(Record, Clone, Serialize, Deserialize)]
#[hook(before_write(func = "validate"))]
pub struct User {
    #[validate(min_length = 5, max_length = 15)]
    username: String,
    #[validate(max_length = 50)]
    name: String,
    #[validate(greater_than(18))]
    age: u16
}
```

## Type safe queries

Enjoy type safety for your AQL queries

###  Rust
```rust
Query::new("Companies")
   .filter(Filter::new(Comparison::any("emails").like("%gmail.com")))
   .sort("company_name", None)
   .sort("company_age", Some(SortDirection::Desc))
   .limit(5, None)
   .distinct();
```

### AQL

```aql
FOR a in Companies
   FILTER a.emails ANY LIKE "%gmail.com"
   SORT a.company_name ASC, a.company_age DESC
   LIMIT 5
   return DISTINCT a
```

## Graph queries

Make complex edge queries

### Rust
```rust
Query::new("Companies")
   .filter(Filter::new(Comparison::any("emails").like("%gmail.com")))
   .sort("company_name", None)
   .join_outbound(1, 2, false,
       Query::new("MemberOf")
           .sort("_id", None)
           .prune(
                Comparison::statement("1").equals(1).into()
            ),
   );

```

### AQL

```aql
FOR b in Companies
    FILTER b.emails ANY LIKE "%gmail.com"
    SORT b.company_name ASC
        FOR a in 1..2 OUTBOUND b MemberOf
            SORT a._id ASC
            PRUNE 1 == 1
            return a
```
# The `EdgeRecord` trait

This trait defines the OGM (Object Graph mapper) of `aragog`.
Every structure implementing this trait becomes an **Edge Model** that can be mapped to a ArangoDB [edge document](https://www.arangodb.com/docs/stable/data-modeling-documents-document-methods.html#edges).

> A Edge document is part of an Edge Collection, and links regular Document together

An Eddge model must be declared as following:

```rust
use aragog::{Record, EdgeRecord};

#[derive(Serialize, Deserialize, Clone, Record, EdgeRecord)]
pub struct ChildOf {
    // This field is mandatory
    pub _from: String,
    // This field is mandatory
    pub _to: String,
    // You can still specify anything after `_from` and `_to` like in any `Record`
    pub notes: Option<String>,
    pub adopted: bool
}
```

To derive `EdgeRecord` your structure needs to derive or implement `Serialize`, `Deserialize` and  `Clone` which are needed
to store the document data and `Record` (see [previous section](./record.md))

An `EdgeRecord` is then also a `Record`, meaning we can use it exactly as one, with a few more aspects.

## Linking documents

To create an edge record we need to fill `_from` (the `_id` of the **from** document) and `_to` (the `_id` of the **to** document)

We can do this manually or use the safer built in method:

```rust
// We consider the `Person` struct to be declared and deriving `Record`
let parent = Person {
    first_name: String::from("Charles-Ange"),
    last_name: String::from("Surcouf"),
};
let parent_record= DatabaseRecord::create(parent, &db_pool).await.unwrap();
let child = Person {
    first_name: String::from("Robert"),
    last_name: String::from("Surcouf")
};
let child_record= DatabaseRecord::create(child, &db_pool).await.unwrap();

// This function will create the Edge Document liking the two person documents
let child_of_record = DatabaseRecord::link(&parent_record, &child_record, &db_pool, |_from, _to| {
    ChildOf {
        _from,
        _to,
        notes: None,
        adopted: false,
    }
}).await.unwrap();
```

We use a closure syntax to allow customized creation while safely giving the correct `_from` and `_to` value.
The returned value of `DatabaseRecord::link` is a `DatabaseRecord<T>` of the edge document that can now be used freely.

> See the ArangoDB documentation on Edge documents and Graphs for a better understanding of the uses

### Validating field formats

The `Validate` trait has a useful method to validate the Edge record `_from` and `_to` field formats.
You can use it this way:

```rust
use aragog::{Record, EdgeRecord, Validate};

#[derive(Serialize, Deserialize, Clone, Record, EdgeRecord, Validate)]
// Add the validate attribute
#[validate(func("validate_edge_fields"))]
pub struct ChildOf {
    pub _from: String,
    pub _to: String,
    pub notes: Option<String>,
    pub adopted: bool
}
```

## Technical notes

### Direct implementation

You can implement `EdgeRecord` directly instead of deriving it.

> We strongly suggest you derive the `EdgeRecord` trait instead of implementing it.
> you would loose the compiler check on the required field presence.

You need to specify the `_from()` and `_to()` methods which, when deriving, are automatically filled.
You also need to implement or derive `Record` (see [previous section](./record.md))

Example:
```rust
use aragog::{Record, EdgeRecord};

#[derive(Serialize, Deserialize, Clone, Record)]
pub struct ChildOf {
    pub _from: String,
    pub _to: String,
}

impl EdgeRecord for ChildOf {
    fn _from(&self) -> String { self._from.clone() }

    fn _to(&self) -> String { self._to.clone() }
}
```

### Todo list

Nothing here for the moment.

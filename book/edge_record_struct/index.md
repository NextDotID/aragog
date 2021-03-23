# The `EdgeRecord` struct

This struct defines the OGM (Object Graph mapper) aspect of `aragog`, an **Edge Model** that can be mapped to an ArangoDB [edge document](https://www.arangodb.com/docs/stable/data-modeling-documents-document-methods.html#edges).

> An Edge document is part of an Edge Collection, and links regular Documents together

To use edges you need to define a [Record](../record_trait/index.md) for your edge collection:

```rust
use aragog::Record;

#[derive(Serialize, Deserialize, Clone, Record)]
pub struct ChildOf {
    pub notes: Option<String>,
    pub adopted: bool
}
```

> The *ChildOf* collection must be an edge collection

And now `ChildOf` can be used as a `EdgeRecord<ChildOf>`.

## Linking documents

ArangoDB edge documents have two mandatory additional fields:
- `_from` containing the **id** of the target document
- `_to` containing the **id** of the target document

This fields are wrapped in the `EdgeRecord` struct, this is why you don't need to worry about specifyng it yourself

In order to link two `Person`:
```rust
let parent = Person {
    first_name: String::from("Charles-Ange"),
    last_name: String::from("Surcouf"),
};
let parent_record= DatabaseRecord::create(parent, &db_connection).await.unwrap();
let child = Person {
    first_name: String::from("Robert"),
    last_name: String::from("Surcouf")
};
let child_record= DatabaseRecord::create(child, &db_connection).await.unwrap();
```

We can do this manually: 

```rust
let edge_document = EdgeRecord::new(parent_record.id(), child_record.id(), ChildOf {
    notes: None,
    adopted: false,
}).unwrap();
let edge_record = DatabaseRecord::create(edge_document, db_connection).await.unwrap();
```
or use the safer built in method:

```rust
let edge_record = DatabaseRecord::link(&parent_record, &child_record, &db_connection,
    ChildOf {
        notes: None,
        adopted: false,
    }
}).await.unwrap();
```

In both cases we have `edge_record` of type `DatabaseRecord<EdgeRecord<ChildOf>>`.

> Both DatabaseRecord and EdgeRecord implement `Deref` and `DerefMut` towards the inner type so you can access inner values:
> ```rust
>  edge_record.adopted = true;
>  ```

### Validation and hooks

`EdgeRecord` validates the format of its `_from` and `_to` fields and calls the hooks of the inner document.

## Retrieval

If you wish to retrieve an edge document from its `key` or a query you need to specify the `EdgeRecord` wrapper to use the edge document features.

example:
```rust
// These will work but you won't have the `from` and `to` value
let edge = ChildOf::find("key", &db_access).await.unwrap();
let edge: DatabaseRecord<ChildOf> = DatabaseRecord::find("key", &db_access).await.unwrap();
// These will work and retrieve also the `from`and `to` values
let edge = EdgeRecord::<ChildOf>::find("key", &db_access).await.unwrap();
let edge: DatabaseRecord<EdgeRecord<ChildOf>> = DatabaseRecord::find("key", &db_access).await.unwrap();
```
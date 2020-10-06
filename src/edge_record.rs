/// Trait for structures that can be stored in Database as a ArangoDB EdgeCollection.
/// The trait must be implemented to be used as a edge record in [`DatabaseRecord`].
///
/// # How to use
///
/// ## Declaration
///
/// A structure deriving from `EdgeRecord` **MUST** contain two string fields:
/// * `_from` - The id of the source document
/// * `_to` - The id of the target document
/// Or the compilation will fail.
///
/// On creation or save these two fields must be valid ArangoDB `Object ID` formatted as:
/// > `CollectionName/DocumentKey`
///
/// (Example: "User/123")
/// >
/// ## Creation
///
/// To create a edge between two existing [`DatabaseRecord`] you can use the following process:
///
/// ```rust ignore
/// # use aragog::{DatabaseRecord, EdgeRecord, Record, Validate};
/// # use serde::{Serialize, Deserialize, de::DeserializeOwned}
/// #
/// #[derive(Clone, EdgeRecord, Validate, Serialize, Deserialize)]
/// struct Edge {
///     _from: String,
///     _to: String,
///     description: String,
/// }
///
/// let record_a = Character::find("123", &database_connection_pool).await.unwrap();
/// let record_b = Character::find("234", &database_connection_pool).await.unwrap();
///
/// let edge_record = DatabaseRecord::link(record_a, record_b, &database_connection_pool, |_from, _to| {
///     Edge { _from, _to, description: "description".to_string() }
/// }).await.unwrap();
/// ```
///
/// [`DatabaseRecord`]: db/database_record/struct.DatabaseRecord.html
pub trait EdgeRecord {

    /// Retrieves the struct `_from` field
    fn _from(&self) -> String;

    /// Retrieves the struct `_to` field
    fn _to(&self) -> String;

    /// Parses the `_from()` returned `id` and returns the document `key`
    ///
    /// # Panic
    ///
    /// The method will panic if the stored `id` is not formatted correctly (`String`/`String`)
    ///
    /// # Example
    ///```rust
    /// # use serde::{Serialize, Deserialize, de::DeserializeOwned};
    /// use aragog::{EdgeRecord, Record, Validate, DatabaseRecord};
    ///
    /// #[derive(EdgeRecord, Clone, Serialize, Deserialize, Validate)]
    /// struct EdgeModel {
    ///     pub _from: String,
    ///     pub _to: String,
    /// }
    ///
    /// let edge = EdgeModel {
    ///    _from: "User/123".to_string(),
    ///    _to: "Client/345".to_string(),
    /// };
    /// assert_eq!(edge._from_key(), "123".to_string());
    /// ```
    fn _from_key(&self) -> String {
        self._from().split('/').last().unwrap().to_string()
    }

    /// Parses the `_to()` returned `id` and returns the document `key`
    ///
    /// # Panic
    ///
    /// The method will panic if the stored `id` is not formatted correctly (`String`/`String`)
    ///
    /// # Example
    ///```rust
    /// # use serde::{Serialize, Deserialize, de::DeserializeOwned};
    /// use aragog::{EdgeRecord, Record, Validate, DatabaseRecord};
    ///
    /// #[derive(EdgeRecord, Clone, Serialize, Deserialize, Validate)]
    /// struct EdgeModel {
    ///     pub _from: String,
    ///     pub _to: String,
    /// }
    ///
    /// let edge = EdgeModel {
    ///    _from: "User/123".to_string(),
    ///    _to: "Client/345".to_string(),
    /// };
    /// assert_eq!(edge._to_key(), "345".to_string());
    /// ```
    fn _to_key(&self) -> String {
        self._to().split('/').last().unwrap().to_string()
    }

    /// Parses the `_to()` returned `id` and returns the document collection name
    ///
    /// # Panic
    ///
    /// The method will panic if the stored `id` is not formatted correctly (`String`/`String`)
    ///
    /// # Example
    ///```rust
    /// # use serde::{Serialize, Deserialize, de::DeserializeOwned};
    /// use aragog::{EdgeRecord, Record, Validate, DatabaseRecord};
    ///
    /// #[derive(EdgeRecord, Clone, Serialize, Deserialize, Validate)]
    /// struct EdgeModel {
    ///     pub _from: String,
    ///     pub _to: String,
    /// }
    ///
    /// let edge = EdgeModel {
    ///    _from: "User/123".to_string(),
    ///    _to: "Client/345".to_string(),
    /// };
    /// assert_eq!(edge._to_collection_name(), "Client".to_string());
    /// ```
    fn _to_collection_name(&self) -> String {
        self._to().split('/').next().unwrap().to_string()
    }

    /// Parses the `_from()` returned `id` and returns the document collection name
    ///
    /// # Panic
    ///
    /// The method will panic if the stored `id` is not formatted correctly (`String`/`String`)
    ///
    /// # Example
    ///```rust
    /// # use serde::{Serialize, Deserialize, de::DeserializeOwned};
    /// use aragog::{EdgeRecord, Record, Validate, DatabaseRecord};
    ///
    /// #[derive(EdgeRecord, Clone, Serialize, Deserialize, Validate)]
    /// struct EdgeModel {
    ///     pub _from: String,
    ///     pub _to: String,
    /// }
    ///
    /// let edge = EdgeModel {
    ///    _from: "User/123".to_string(),
    ///    _to: "Client/345".to_string(),
    /// };
    /// assert_eq!(edge._from_collection_name(), "User".to_string());
    /// ```
    fn _from_collection_name(&self) -> String {
        self._from().split('/').next().unwrap().to_string()
    }

    /// Validation method for `EdgeRecord` to use on [`Validate`] implementation.
    /// Verifies that both `_from` and `_to` fields have correct format.
    fn validate_edge_fields(&self, errors: &mut Vec<String>) {
        let array = [("_from", self._from()), ("_to", self._to())];
        for (name, field) in array.iter() {
            let split = field.split('/');
            if split.count() != 2 {
                errors.push(format!(r#"{} "{}" has wrong format"#, name, field));
            }
        }
    }
}
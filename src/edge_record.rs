use crate::{DatabaseAccess, DatabaseRecord, Error, Record, Validate};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// Struct wrapping an edge document, with the `from` and `to` fields correctly set.
///
/// The document of type `T` mut implement [`Record`] and `EdgeRecord` also implements it.
///
/// # Note
///
/// `EdgeRecord` implements `Deref` and `DerefMut` into `T`
///
/// [`Record`]: trait.Record.html
#[derive(Serialize, Deserialize, Clone)]
pub struct EdgeRecord<T> {
    /// The `_from` field of `ArangoDB` edge documents
    #[serde(rename(serialize = "_from", deserialize = "_from"))]
    from: String,
    /// The `to` field of `ArangoDB` edge documents
    #[serde(rename(serialize = "_to", deserialize = "_to"))]
    to: String,
    /// The main document data, must implement [`Record`].
    ///
    /// Note: The data is flattened on save, so you won't have any field named `data` in your database.
    ///
    /// [`Record`]: trait.Record.html
    #[serde(flatten)]
    pub data: T,
}

impl<T: Record> EdgeRecord<T> {
    /// Manually instantiates an Edge record
    ///
    /// # Arguments
    ///
    /// * `id_from` - The **from** document `id`
    /// * `id_to` - The **to** document `id`
    /// * `data` - The main document data
    ///
    /// # Errors
    ///
    /// This function validates the format of the id fields which can result in an error.
    pub fn new(id_from: String, id_to: String, data: T) -> Result<Self, Error> {
        let res = Self {
            from: id_from,
            to: id_to,
            data,
        };
        res.validate()?;
        Ok(res)
    }

    /// Retrieves the `from` document from the database
    #[maybe_async::maybe_async]
    pub async fn from_record<D, R>(&self, db_access: &D) -> Result<DatabaseRecord<R>, Error>
    where
        D: DatabaseAccess + ?Sized,
        R: Record,
    {
        DatabaseRecord::find(self.key_from().as_str(), db_access).await
    }

    /// Retrieves the `to` document from the database
    #[maybe_async::maybe_async]
    pub async fn to_record<D, R>(&self, db_access: &D) -> Result<DatabaseRecord<R>, Error>
    where
        D: DatabaseAccess + ?Sized,
        R: Record,
    {
        DatabaseRecord::find(self.key_to().as_str(), db_access).await
    }

    /// Retrieves the document `_from` field, storing the target documents `id`.
    pub fn id_from(&self) -> &String {
        &self.from
    }

    /// Retrieves the document `_to` field, storing the target documents `id`.
    pub fn id_to(&self) -> &String {
        &self.to
    }

    /// Parses the `from` value to retrieve only the `_key` part.
    ///
    /// # Panics
    ///
    /// This method may panic if the `from` value is not formatted correctly.
    pub fn key_from(&self) -> String {
        self.id_from().split('/').last().unwrap().to_string()
    }

    /// Parses the `to` value to retrieve only the `_key` part.
    ///
    /// # Panics
    ///
    /// This method may panic if the `to` value is not formatted correctly.
    pub fn key_to(&self) -> String {
        self.id_to().split('/').last().unwrap().to_string()
    }

    /// Parses the `from` value to retrieve only the collection name part.
    ///
    /// # Panics
    ///
    /// This method may panic if the `to` value is not formatted correctly.
    pub fn to_collection_name(&self) -> String {
        self.id_to().split('/').next().unwrap().to_string()
    }

    /// Parses the `to` value to retrieve only the collection name part.
    ///
    /// # Panics
    ///
    /// This method may panic if the `from` value is not formatted correctly.
    pub fn from_collection_name(&self) -> String {
        self.id_from().split('/').next().unwrap().to_string()
    }

    fn validate_edge_fields(&self, errors: &mut Vec<String>) {
        let array = [("from", self.id_from()), ("to", self.id_to())];
        for (name, field) in array {
            let split = field.split('/').collect::<Vec<&str>>();
            if split.len() != 2 {
                errors.push(format!(r#"{} "{}" has wrong format"#, name, field));
            }
            let left_part = split.first().unwrap();
            Self::validate_min_len(name, left_part, 2, errors);
            let right_part = split.last().unwrap();
            Self::validate_min_len(name, right_part, 2, errors);
            Self::validate_numeric_string(name, right_part, errors);
        }
    }
}

impl<T: Record> Validate for EdgeRecord<T> {
    fn validations(&self, errors: &mut Vec<String>) {
        self.validate_edge_fields(errors);
    }
}

#[maybe_async::maybe_async]
impl<T: Record + Send> Record for EdgeRecord<T> {
    const COLLECTION_NAME: &'static str = T::COLLECTION_NAME;

    async fn before_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), Error>
    where
        D: DatabaseAccess + ?Sized,
    {
        self.validate()?;
        self.data.before_create_hook(db_accessor).await
    }

    async fn before_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), Error>
    where
        D: DatabaseAccess + ?Sized,
    {
        self.data.before_save_hook(db_accessor).await
    }

    async fn before_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), Error>
    where
        D: DatabaseAccess + ?Sized,
    {
        self.data.before_delete_hook(db_accessor).await
    }

    async fn after_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), Error>
    where
        D: DatabaseAccess + ?Sized,
    {
        self.data.after_create_hook(db_accessor).await
    }

    async fn after_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), Error>
    where
        D: DatabaseAccess + ?Sized,
    {
        self.validate()?;
        self.data.after_save_hook(db_accessor).await
    }

    async fn after_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), Error>
    where
        D: DatabaseAccess + ?Sized,
    {
        self.data.after_delete_hook(db_accessor).await
    }
}

impl<T: Record> Deref for EdgeRecord<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Record> DerefMut for EdgeRecord<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

use crate::{DatabaseAccess, DatabaseRecord, Record, ServiceError};

/// The `ForeignLink` trait of the Aragog library.
/// It allows to define foreign_key relations between different models.
///
/// # Example
///
/// ```rust
/// # use aragog::{Record, Validate, ForeignLink, DatabaseConnectionPool, DatabaseRecord, AuthMode};
/// # use serde::{Deserialize, Serialize};
/// # use std::borrow::Borrow;
/// #
/// #[derive(Clone, Serialize, Deserialize, Record, Validate)]
/// pub struct Order {
///     pub content: String,
///     pub user_id: String,
/// }
///
/// #[derive(Clone, Serialize, Deserialize, Record, Validate)]
/// pub struct User {}
///
/// impl ForeignLink<User> for Order {
///     fn foreign_key(&self) -> &str {
///         self.user_id.borrow()
///     }
/// }
///
/// # #[tokio::main]
/// # async fn main() {
/// # let database_pool = DatabaseConnectionPool::builder()
/// #     .with_credentials(
/// #       &std::env::var("DB_HOST").unwrap_or("http://localhost:8529".to_string()),
/// #       &std::env::var("DB_NAME").unwrap_or("aragog_test".to_string()),
/// #       &std::env::var("DB_USER").unwrap_or("test".to_string()),
/// #       &std::env::var("DB_PWD").unwrap_or("test".to_string())
/// #     )
/// #    .with_schema_path("tests/schema.yaml")
/// #    .build()
/// #    .await
/// #    .unwrap();
/// # database_pool.truncate().await;
/// let user = DatabaseRecord::create(User {}, &database_pool).await.unwrap();
/// let order = Order {
///     content: "content".to_string(),
///     user_id: user.key().clone()
/// };
/// let linked_user = order.linked_model(&database_pool).await.unwrap();
/// assert_eq!(user.id(), linked_user.id());
/// # }
/// ```
#[maybe_async::maybe_async]
pub trait ForeignLink<T: Record>: Sized {
    /// Defines the foreign key field to the linked `T` model.
    ///
    /// # Example
    /// ```rust
    /// # use aragog::{Record, Validate, ForeignLink, DatabaseConnectionPool, DatabaseRecord};
    /// # use serde::{Deserialize, Serialize};
    /// # use std::borrow::Borrow;
    /// #
    /// #[derive(Clone, Serialize, Deserialize, Record, Validate)]
    /// pub struct Order {
    ///     pub content: String,
    ///     pub user_id: String,
    /// }
    ///
    /// #[derive(Clone, Serialize, Deserialize, Record, Validate)]
    /// pub struct User {}
    ///
    /// impl ForeignLink<User> for Order {
    ///     fn foreign_key(&self) -> &str {
    ///         self.user_id.borrow()
    ///     }
    /// }
    /// ```
    fn foreign_key(&self) -> &str;

    /// Retrieves the record matching the defined `foreign_key`. Type inference may be required.
    #[cfg(feature = "async")]
    async fn linked_model<D>(&self, db_pool: &D) -> Result<DatabaseRecord<T>, ServiceError>
    where
        Self: Sized,
        T: 'async_trait,
        D: DatabaseAccess + ?Sized,
    {
        DatabaseRecord::find(self.foreign_key(), db_pool).await
    }

    /// Retrieves the record matching the defined `foreign_key`. Type inference may be required.
    #[cfg(not(feature = "async"))]
    fn linked_model<D>(&self, db_pool: &D) -> Result<DatabaseRecord<T>, ServiceError>
    where
        D: DatabaseAccess + ?Sized,
    {
        DatabaseRecord::find(self.foreign_key(), db_pool)
    }
}

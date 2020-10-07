use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{DatabaseConnectionPool, DatabaseRecord, Record, ServiceError};

/// The `ForeignLink` trait of the Aragog library.
/// It allows to define foreign_key relations between different models.
///
/// # Example
///
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
///
/// # #[tokio::main]
/// # async fn main() {
/// # std::env::set_var("SCHEMA_PATH", "tests/schema.json");
/// # let database_pool = DatabaseConnectionPool::new(
/// #       &std::env::var("DB_HOST").unwrap_or("http://localhost:8529".to_string()),
/// #       &std::env::var("DB_NAME").unwrap_or("aragog_test".to_string()),
/// #       &std::env::var("DB_USER").unwrap_or("test".to_string()),
/// #       &std::env::var("DB_PWD").unwrap_or("test".to_string())).await;
/// # database_pool.truncate().await;
/// let user = DatabaseRecord::create(User {}, &database_pool).await.unwrap();
/// let order = Order {
///     content: "content".to_string(),
///     user_id: user.key.clone()
/// };
/// let linked_user = order.linked_model(&database_pool).await.unwrap();
/// assert_eq!(&user.id, &linked_user.id);
/// # }
/// ```
#[async_trait]
pub trait ForeignLink<T: Record + Serialize + DeserializeOwned + Clone> {
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
    async fn linked_model(&self, db_pool: &DatabaseConnectionPool) -> Result<DatabaseRecord<T>, ServiceError>
        where Self: Sized,
              T: 'async_trait
    {
        DatabaseRecord::find(self.foreign_key(), db_pool).await
    }
}
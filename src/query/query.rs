use std::fmt::{self, Display};

use serde::de::DeserializeOwned;
use serde::export::Formatter;
use serde::{Serialize, Deserialize};

use crate::{DatabaseConnectionPool, DatabaseRecord, Record, ServiceError};
use crate::query::{Filter, QueryResult};

/// Macro to simplify the [`Query`] construction:
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate aragog;
/// # use aragog::query::Query;
///
/// # fn main() {
/// // The following are equivalent:
/// let query = Query::new("Users");
/// let query = query!("Users");
/// # }
/// ```
///
/// [`Query`]: query/struct.Query.html
#[macro_export]
macro_rules! query {
    ($collection:expr) => {
        $crate::query::Query::new($collection)
    }
}

/// The direction for [`Query`] [`sort`] method
///
/// [`Query`]: struct.Query.html
/// [`sort`]: struct.Query.html#method.sort
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SortDirection {
    /// Ascending
    Asc,
    /// Descending
    Desc,
}

impl Display for SortDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC"
        })
    }
}

/// A query utility for ArangoDB to avoid writing simple AQL strings. After building can be rendered
/// as an AQL string with the [`render`] method.
///
/// # Examples
///
/// ```rust
/// # use aragog::Record;
/// # use aragog::query::Query;
/// # use serde::{Serialize, Deserialize};
/// # #[macro_use] extern crate aragog;
/// #
/// #[derive(Clone, Serialize, Deserialize)]
/// pub struct User {
///     pub username: String
/// }
///
/// impl Record for User {
///     fn collection_name() -> &'static str { "Users" }
/// }
///
/// # fn main() {
/// // You can init a query in three ways, the following lines do the exact same thing
/// let query = Query::new("Users");
/// let query2 = User::query(); // `User` needs to implement `Record`
/// let query3 = query!("Users");
/// # }
/// ```
///
/// [`render`]: struct.Query.html#method.render
#[derive(Clone, Debug)]
pub struct Query {
    collection: String,
    filter: Option<String>,
    sort: Option<String>,
    skip: Option<u32>,
    limit: Option<u32>,
    distinct: bool,
}

impl Query {
    /// Creates a new empty `Query`.
    /// You can call `filter`, `sort`, `limit` and `distinct` to customize the query afterwards
    ///
    /// # Arguments
    ///
    /// * `collection_name`- The name of the queried collection
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::Query;
    /// let query = Query::new("User");
    /// ```
    pub fn new(collection_name: &str) -> Self {
        Self {
            collection: String::from(collection_name),
            filter: None,
            sort: None,
            skip: None,
            limit: None,
            distinct: false,
        }
    }

    /// Allows to sort a current `Query` by different field names. The fields must exist or the query won't work.
    /// Every time the method is called, a new sorting condition is added.
    ///
    /// # Arguments
    ///
    /// * `field`: The field name, must exist in the collection
    /// * `direction`: Optional sorting direction for that field.
    /// The direction is optional because `ArangoDB` uses `ASC` sorting by default
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Query, SortDirection};
    /// let query = Query::new("User")
    ///     .sort("username", Some(SortDirection::Desc))
    ///     .sort("age", Some(SortDirection::Asc)
    /// );
    /// ```
    pub fn sort(mut self, field: &str, direction: Option<SortDirection>) -> Self {
        let mut sort_str = format!("i.{}", field);
        if direction.is_some() {
            sort_str = format!("{} {}", sort_str, direction.unwrap());
        }
        if self.sort.is_none() {
            self.sort = Some(sort_str);
        } else {
            self.sort = Some(format!("{}, {}", self.sort.unwrap(), sort_str));
        }
        self
    }

    /// Allows to filter a current `Query` by different comparisons.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Query, Filter, Comparison};
    /// let query = Query::new("User").filter(Filter::new(Comparison::field("age").greater_than(18)));
    /// ```
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter.render());
        self
    }

    /// Allows to paginate a current `Query`.
    ///
    /// # Arguments
    ///
    /// * `limit` - the maximum returned elements
    /// * `skip`- optional number of skipped elements
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::Query;
    /// // We want maximum 10 elements but skip the first 5
    /// let query = Query::new("User").limit(10, Some(5));
    /// ```
    pub fn limit(mut self, limit: u32, skip: Option<u32>) -> Self {
        self.limit = Some(limit);
        self.skip = skip;
        self
    }

    /// Allows to avoid duplicate elements for a `Query`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Query, Filter, Comparison};
    /// let query = Query::new("User")
    ///     .filter(Filter::new(Comparison::field("age").greater_than(18)))
    ///     .distinct();
    /// ```
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }

    /// Renders the AQL string corresponding to the current `Query`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    /// let mut query = Query::new("User").filter(Filter::new(Comparison::field("age").greater_than(10)).
    ///     or(Comparison::field("username").in_str_array(&["felix", "felixm"]))).distinct();
    /// assert_eq!(query.render(), String::from(r#"FOR i in User FILTER i.age > 10 || i.username IN ["felix", "felixm"] DISTINCT return i"#));
    /// ```
    pub fn render(&self) -> String {
        let mut res = format!("FOR i in {}", &self.collection);
        if self.filter.is_some() {
            res = format!("{} {}", res, self.filter.clone().unwrap());
        }
        if self.sort.is_some() {
            res = format!("{} SORT {}", res, &self.sort.clone().unwrap());
        }
        if self.limit.is_some() {
            let skip_str = match self.skip {
                None => String::new(),
                Some(val) => format!("{}, ", val)
            };
            res = format!("{} LIMIT {}{}", res, skip_str, self.limit.unwrap());
        }
        if self.distinct {
            res += " DISTINCT";
        }
        res += " return i";
        res
    }

    /// Finds all documents in database matching the current `Query`.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`get`]
    ///
    /// [`DatabaseRecord`]: ../struct.DatabaseRecord.html
    /// [`get`]: ../struct.DatabaseRecord.html#method.get
    pub async fn call<T>(self, db_pool: &DatabaseConnectionPool) -> Result<QueryResult<T>, ServiceError>
        where T: Record + Clone + Serialize + DeserializeOwned
    {
        DatabaseRecord::get(self, db_pool).await
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}
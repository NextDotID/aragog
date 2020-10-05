use std::fmt::{self, Display};

#[cfg(feature = "open-api")]
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use serde::export::Formatter;

use crate::{DatabaseConnectionPool, ServiceError};
use crate::query::Filter;
use crate::query::graph_query::{GraphQueryData, GraphQueryDirection};
use crate::query::operations::{AqlOperation, OperationContainer};
use crate::query::query_id_helper::get_str_identifier;
use crate::query::query_result::JsonQueryResult;

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
#[cfg_attr(feature = "open-api", derive(Apiv2Schema))]
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
/// as an AQL string with the [`to_aql`] method.
///
/// # Examples
///
/// ```rust
/// # use aragog::Record;
/// # use aragog::query::Query;
/// # use serde::{Serialize, Deserialize};
/// # #[macro_use] extern crate aragog;
/// #
/// #[derive(Clone, Serialize, Deserialize, Record)]
/// pub struct User {
///     pub username: String
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
/// [`to_aql`]: struct.Query.html#method.to_aql
#[derive(Clone, Debug)]
pub struct Query {
    collection: String,
    graph_data: Option<GraphQueryData>,
    operations: OperationContainer,
    distinct: bool,
    sub_query: Option<String>,
    item_identifier: usize,
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
            graph_data: None,
            operations: OperationContainer(vec![]),
            distinct: false,
            sub_query: None,
            item_identifier: 0,
        }
    }

    /// Creates a new outbound traversing `Query` though a `edge_collection`.
    /// You can call `filter`, `sort`, `limit` and `distinct` to customize the query afterwards
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum depth of the graph request
    /// * `max` - The maximum depth of the graph request
    /// * `edge_collection`- The name of the traversed edge collection
    /// * `vertex` - The `_id` of the starting document (`User/123` for example)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::Query;
    /// let query = Query::outbound(1, 2, "ChildOf", "User/123");
    /// ```
    pub fn outbound(min: u16, max: u16, edge_collection: &str, vertex: &str) -> Self {
        Self {
            graph_data: Some(GraphQueryData {
                direction: GraphQueryDirection::Outbound,
                start_vertex: format!(r#"'{}'"#, vertex),
                min,
                max,
                named_graph: false,
            }),
            ..Self::new(edge_collection)
        }
    }

    /// Creates a new outbound traversing `Query` though a `named_grah`.
    /// You can call `filter`, `sort`, `limit` and `distinct` to customize the query afterwards
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum depth of the graph request
    /// * `max` - The maximum depth of the graph request
    /// * `named_graph`- The named graph to traverse
    /// * `vertex` - The `_id` of the starting document (`User/123` for example)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::Query;
    /// let query = Query::outbound_graph(1, 2, "SomeGraph", "User/123");
    /// ```
    pub fn outbound_graph(min: u16, max: u16, named_graph: &str, vertex: &str) -> Self {
        Self {
            graph_data: Some(GraphQueryData {
                direction: GraphQueryDirection::Outbound,
                start_vertex: format!(r#"'{}'"#, vertex),
                min,
                max,
                named_graph: true,
            }),
            ..Self::new(named_graph)
        }
    }

    /// Creates a new `ANY` traversing `Query` though a `edge_collection`.
    /// You can call `filter`, `sort`, `limit` and `distinct` to customize the query afterwards
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum depth of the graph request
    /// * `max` - The maximum depth of the graph request
    /// * `edge_collection`- The name of the traversed edge collection
    /// * `vertex` - The `_id` of the starting document (`User/123` for example)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::Query;
    /// let query = Query::outbound(1, 2, "ChildOf", "User/123");
    /// ```
    pub fn any(min: u16, max: u16, edge_collection: &str, vertex: &str) -> Self {
        Self {
            graph_data: Some(GraphQueryData {
                direction: GraphQueryDirection::Any,
                start_vertex: format!(r#"'{}'"#, vertex),
                min,
                max,
                named_graph: false,
            }),
            ..Self::new(edge_collection)
        }
    }

    /// Creates a new `ANY` traversing `Query` though a `named_grah`.
    /// You can call `filter`, `sort`, `limit` and `distinct` to customize the query afterwards
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum depth of the graph request
    /// * `max` - The maximum depth of the graph request
    /// * `named_graph`- The named graph to traverse
    /// * `vertex` - The `_id` of the starting document (`User/123` for example)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::Query;
    /// let query = Query::outbound_graph(1, 2, "SomeGraph", "User/123");
    /// ```
    pub fn any_graph(min: u16, max: u16, named_graph: &str, vertex: &str) -> Self {
        Self {
            graph_data: Some(GraphQueryData {
                direction: GraphQueryDirection::Any,
                start_vertex: format!(r#"'{}'"#, vertex),
                min,
                max,
                named_graph: true,
            }),
            ..Self::new(named_graph)
        }
    }

    /// Creates a new inbound traversing `Query` though a `edge_collection`.
    /// You can call `filter`, `sort`, `limit` and `distinct` to customize the query afterwards
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum depth of the graph request
    /// * `max` - The maximum depth of the graph request
    /// * `edge_collection`- The name of the traversed edge collection
    /// * `vertex` - The `_id` of the starting document (`User/123` for example)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::Query;
    /// let query = Query::inbound(1, 2, "ChildOf", "User/123");
    /// ```
    pub fn inbound(min: u16, max: u16, edge_collection: &str, vertex: &str) -> Self {
        Self {
            graph_data: Some(GraphQueryData {
                direction: GraphQueryDirection::Inbound,
                start_vertex: format!(r#"'{}'"#, vertex),
                min,
                max,
                named_graph: false,
            }),
            ..Self::new(edge_collection)
        }
    }

    /// Creates a new inbound traversing `Query` though a `named_grah`.
    /// You can call `filter`, `sort`, `limit` and `distinct` to customize the query afterwards
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum depth of the graph request
    /// * `max` - The maximum depth of the graph request
    /// * `named_graph`- The named graph to traverse
    /// * `vertex` - The `_id` of the starting document (`User/123` for example)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::Query;
    /// let query = Query::inbound_graph(1, 2, "SomeGraph", "User/123");
    /// ```
    pub fn inbound_graph(min: u16, max: u16, named_graph: &str, vertex: &str) -> Self {
        Self {
            graph_data: Some(GraphQueryData {
                direction: GraphQueryDirection::Inbound,
                start_vertex: format!(r#"'{}'"#, vertex),
                min,
                max,
                named_graph: true,
            }),
            ..Self::new(named_graph)
        }
    }

    fn join(mut self, min: u16, max: u16, mut query: Query, direction: GraphQueryDirection, named_graph: bool) -> Self {
        self.item_identifier = query.item_identifier + 1;
        query.graph_data = Some(GraphQueryData {
            direction,
            start_vertex: get_str_identifier(self.item_identifier),
            min,
            max,
            named_graph,
        });
        self.sub_query = Some(query.to_aql());
        self
    }

    /// Adds an outbound traversing query to the current `Query`.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum depth of the graph request
    /// * `max` - The maximum depth of the graph request
    /// * `named_graph` - Is the following query on a Named graph?
    /// * `query` - The sub query to add
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::Query;
    /// let query = Query::new("User").join_outbound(1, 2, false, Query::new("ChildOf"));
    /// assert_eq!(query.to_aql(), String::from("\
    ///     FOR b in User \
    ///         FOR a in 1..2 OUTBOUND b ChildOf \
    ///         return a\
    /// "));
    /// let query = Query::new("User").join_outbound(1, 2, true, Query::new("NamedGraph"));
    /// assert_eq!(query.to_aql(), String::from("\
    ///     FOR b in User \
    ///         FOR a in 1..2 OUTBOUND b GRAPH NamedGraph \
    ///         return a\
    /// "));
    /// ```
    pub fn join_outbound(self, min: u16, max: u16, named_graph: bool, query: Query) -> Self {
        self.join(min, max, query, GraphQueryDirection::Outbound, named_graph)
    }

    /// Adds an inbound traversing query to the current `Query`.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum depth of the graph request
    /// * `max` - The maximum depth of the graph request
    /// * `named_graph` - Is the following query on a Named graph?
    /// * `query` - The sub query to add
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::Query;
    /// let query = Query::new("User").join_inbound(1, 2, false, Query::new("ChildOf"));
    /// assert_eq!(query.to_aql(), String::from("\
    ///     FOR b in User \
    ///         FOR a in 1..2 INBOUND b ChildOf \
    ///         return a\
    /// "));
    /// let query = Query::new("User").join_inbound(1, 2, true, Query::new("NamedGraph"));
    /// assert_eq!(query.to_aql(), String::from("\
    ///     FOR b in User \
    ///         FOR a in 1..2 INBOUND b GRAPH NamedGraph \
    ///         return a\
    /// "));
    /// ```
    pub fn join_inbound(self, min: u16, max: u16, named_graph: bool, query: Query) -> Self {
        self.join(min, max, query, GraphQueryDirection::Inbound, named_graph)
    }

    /// Adds an `ANY` traversing query to the current `Query`.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum depth of the graph request
    /// * `max` - The maximum depth of the graph request
    /// * `named_graph` - Is the following query on a Named graph?
    /// * `query` - The sub query to add
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::Query;
    /// let query = Query::new("User").join_any(1, 2, false, Query::new("ChildOf"));
    /// assert_eq!(query.to_aql(), String::from("\
    ///     FOR b in User \
    ///         FOR a in 1..2 ANY b ChildOf \
    ///         return a\
    /// "));
    /// let query = Query::new("User").join_any(1, 2, true, Query::new("NamedGraph"));
    /// assert_eq!(query.to_aql(), String::from("\
    ///     FOR b in User \
    ///         FOR a in 1..2 ANY b GRAPH NamedGraph \
    ///         return a\
    /// "));
    /// ```
    pub fn join_any(self, min: u16, max: u16, named_graph: bool, query: Query) -> Self {
        self.join(min, max, query, GraphQueryDirection::Any, named_graph)
    }

    /// Allows to sort a current `Query` by different field names. The fields must exist or the query won't work.
    /// Every time the method is called, a new sorting condition is added.
    ///
    /// # Note
    ///
    /// If you add mutliple `sort` calls it will result in something like `SORT a.field, b.field, c.field`.
    /// If you separate the calls by a `limit` or other operation, the order will be respected and the resulting query
    /// will look like `SORT a.field LIMIT 10 SORT b.field, c.field
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
        self.operations.0.push(AqlOperation::Sort {
            field: field.to_string(),
            direction: direction.unwrap_or(SortDirection::Asc),
        });
        self
    }

    /// Allows to filter a current `Query` by different comparisons.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Query, Filter, Comparison};
    /// let query = Query::new("User").filter(Filter::new(Comparison::field("age").greater_than(18)));
    /// // or
    /// let query = Query::new("User").filter(Comparison::field("age").greater_than(18).into());
    /// ```
    pub fn filter(mut self, filter: Filter) -> Self {
        self.operations.0.push(AqlOperation::Filter(filter));
        self
    }

    /// Allows to filter a current `Query` by different comparisons but using the `PRUNE` keyword.
    ///
    /// # Note
    ///
    /// The `prune` operation only works for graph queries (See ArangoDB documentation)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Query, Filter, Comparison};
    /// let query = Query::outbound(1, 2, "ChildOf", "User/123").prune(Filter::new(Comparison::field("age").greater_than(18)));
    /// // or
    /// let query = Query::outbound(1, 2, "ChildOf", "User/123").prune(Comparison::field("age").greater_than(18).into());
    /// ```
    pub fn prune(mut self, filter: Filter) -> Self {
        self.operations.0.push(AqlOperation::Prune(filter));
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
        self.operations.0.push(AqlOperation::Limit { skip, limit });
        self
    }

    /// Allows to avoid duplicate elements for a `Query`.
    ///
    /// # Note
    ///
    /// If you use sub-queries, only the `distinct` on the last sub query will be used.
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
    ///     or(Comparison::field("username").in_str_array(&["Felix", "Bianca"]))).distinct();
    /// assert_eq!(query.to_aql(), String::from("\
    ///     FOR a in User \
    ///         FILTER a.age > 10 || a.username IN [\"Felix\", \"Bianca\"] \
    ///         return DISTINCT a\
    /// "));
    /// ```
    pub fn to_aql(&self) -> String {
        let mut res;
        let collection_id = get_str_identifier(self.item_identifier);
        if self.graph_data.is_some() {
            let graph_data = self.graph_data.as_ref().unwrap();
            res = format!(
                "FOR {} in {}..{} {} {} {}{}",
                collection_id,
                graph_data.min,
                graph_data.max,
                graph_data.direction,
                &graph_data.start_vertex,
                if graph_data.named_graph { "GRAPH " } else { "" },
                &self.collection
            );
        } else {
            res = format!("FOR {} in {}", collection_id, &self.collection);
        }
        if self.operations.0.len() > 0 {
            res = format!("{} {}", res, self.operations.to_aql(&collection_id));
        }
        if self.sub_query.is_some() {
            res = format!("{} {}", res, self.sub_query.as_ref().unwrap())
        } else {
            res = format!("{} return {}{}", res, if self.distinct {
                "DISTINCT "
            } else {
                ""
            }, &collection_id);
        }
        res
    }

    /// Finds all documents in database matching the current `Query`.
    /// This will return a wrapper for `serde_json`::`Value`
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`get`]
    ///
    /// [`DatabaseRecord`]: ../struct.DatabaseRecord.html
    /// [`get`]: ../struct.DatabaseRecord.html#method.get
    pub async fn call(self, db_pool: &DatabaseConnectionPool) -> Result<JsonQueryResult, ServiceError>
    {
        db_pool.aql_get(&self.to_aql()).await
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_aql())
    }
}
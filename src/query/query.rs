use std::fmt::{Display, Result};
use serde::export::Formatter;
use crate::query::QueryItem;

/// A query utility for ArangoDB to avoid writing simple AQL strings. After building can be rendered
/// as an AQL string with the [`render`] method.
///
/// [`render`]: struct.Query.html#method.render
#[derive(Clone, Debug)]
pub struct Query {
    aql_string: String,
}

impl Query {

    /// Instantiates a new query from a comparison item
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    /// let query = Query::new(QueryItem::field("age").greater_than(10));
    /// ```
    pub fn new(comparison: QueryItem) -> Self {
        Self {
            aql_string: format!("FILTER i.{}", comparison)
        }
    }

    /// Appends the query current condition(s) with a new one with a `AND` logic.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    /// let mut query = Query::new(QueryItem::field("age").greater_than(10));
    /// query = query.and(QueryItem::field("username").in_str_array(&["felix", "felixm"]));
    /// ```
    ///
    pub fn and(mut self, comparison: QueryItem) -> Self {
        self.aql_string = format!("{} && i.{}", self.aql_string, comparison);
        self
    }

    /// Appends the query current condition(s) with a new one with a `OR` logic.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    /// let mut query = Query::new(QueryItem::field("age").greater_than(10));
    /// query = query.or(QueryItem::field("username").in_str_array(&["felix", "felixm"]));
    /// ```
    ///
    pub fn or(mut self, comparison: QueryItem) -> Self {
        self.aql_string = format!("{} || i.{}", self.aql_string, comparison);
        self
    }

    /// Renders the AQL string corresponding to the current `Query`. The query will go out of scope.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    /// let mut query = Query::new(QueryItem::field("age").greater_than(10)).
    ///     or(QueryItem::field("username").in_str_array(&["felix", "felixm"]));
    /// assert_eq!(query.render(), String::from(r#"FILTER i.age > 10 || i.username IN ["felix", "felixm"]"#));
    /// ```
    pub fn render(self) -> String { self.aql_string }

    /// Renders a new AQL string corresponding to the current `Query`. The query won't go out of scope.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    /// let mut query = Query::new(QueryItem::field("age").greater_than(10)).
    ///     or(QueryItem::field("username").in_str_array(&["felix", "felixm"]));
    /// assert_eq!(query.render_borrow(), String::from(r#"FILTER i.age > 10 || i.username IN ["felix", "felixm"]"#));
    /// ```
    pub fn render_borrow(&self) -> String { self.aql_string.clone() }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.render_borrow())
    }
}
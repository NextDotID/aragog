use std::fmt::{Display, Result};
use serde::export::Formatter;
use crate::query::Comparison;

/// Allows to filter a query according to different [`Comparison`].
/// as an AQL string with the [`render`] method.
///
/// [`render`]: struct.Filter.html#method.render
/// [`Comparison`]: struct.Comparison.html
#[derive(Clone, Debug)]
pub struct Filter {
    filter_string: String,
}

impl Filter {
    /// Instantiates a new query filter from a comparison item
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Filter};
    /// let filter = Filter::new(Comparison::field("age").greater_than(10));
    /// ```
    pub fn new(comparison: Comparison) -> Self {
        Self {
            filter_string: format!("FILTER {}", comparison)
        }
    }

    /// Appends the filter current condition(s) with a new one with a `AND` logic.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Filter};
    /// let mut filter = Filter::new(Comparison::field("age").greater_than(10));
    /// filter = filter.and(Comparison::field("username").in_str_array(&["felix", "felixm"]));
    /// ```
    ///
    pub fn and(mut self, comparison: Comparison) -> Self {
        self.filter_string = format!("{} && {}", self.filter_string, comparison);
        self
    }

    /// Appends the filter current condition(s) with a new one with a `OR` logic.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Filter};
    /// let mut filter = Filter::new(Comparison::field("age").greater_than(10));
    /// filter = filter.or(Comparison::field("username").in_str_array(&["felix", "felixm"]));
    /// ```
    ///
    pub fn or(mut self, comparison: Comparison) -> Self {
        self.filter_string = format!("{} || {}", self.filter_string, comparison);
        self
    }

    /// Renders the AQL string corresponding to the current `Filter`. The query will go out of scope.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Filter};
    /// let mut filter = Filter::new(Comparison::field("age").greater_than(10)).
    ///     or(Comparison::field("username").in_str_array(&["felix", "felixm"]));
    /// assert_eq!(filter.render(), String::from(r#"FILTER i.age > 10 || i.username IN ["felix", "felixm"]"#));
    /// ```
    pub fn render(self) -> String { self.filter_string }
}

impl Display for Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.filter_string.clone())
    }
}
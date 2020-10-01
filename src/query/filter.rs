use std::fmt::{Display, Result};

use serde::export::Formatter;

use crate::query::{Comparison, SortDirection};

#[derive(Clone, Debug)]
enum Operator {
    AND,
    OR,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Self::AND => "&&",
            Self::OR => "||",
        })
    }
}

#[derive(Debug, Clone)]
pub enum AqlOperation {
    Filter(Filter),
    Limit { skip: Option<u32>, limit: u32 },
    Sort { field: String, direction: SortDirection },
}

/// Allows to filter a query according to different [`Comparison`].
/// as an AQL string with the [`to_aql`] method.
///
/// [`to_aql`]: struct.Filter.html#method.to_aql
/// [`Comparison`]: struct.Comparison.html
#[derive(Clone, Debug)]
pub struct Filter {
    comparisons: Vec<Comparison>,
    operators: Vec<Operator>,
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
            comparisons: vec![comparison],
            operators: vec![],
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
        self.comparisons.push(comparison);
        self.operators.push(Operator::AND);
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
        self.comparisons.push(comparison);
        self.operators.push(Operator::OR);
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
    /// assert_eq!(filter.to_aql("i"), String::from(r#"FILTER i.age > 10 || i.username IN ["felix", "felixm"]"#));
    /// ```
    pub fn to_aql(&self, collection_id: &str) -> String {
        let mut res = String::from("FILTER");
        for (i, comparison) in self.comparisons.iter().enumerate() {
            let operator_str = if i >= self.operators.len() {
                String::new()
            } else {
                format!(" {}", self.operators[i].to_string())
            };
            res = format!("{} {}{}", res, comparison.to_aql(collection_id), operator_str)
        }
        res
    }
}
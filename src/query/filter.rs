use std::fmt::{Display, Formatter, Result};

use crate::query::Comparison;

#[derive(Clone, Debug)]
enum Operator {
    And,
    Or,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::And => "&&",
                Self::Or => "||",
            }
        )
    }
}

/// Allows to filter a query according to different [`Comparison`].
///
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
    #[must_use]
    #[inline]
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
    #[must_use]
    #[inline]
    pub fn and(mut self, comparison: Comparison) -> Self {
        self.comparisons.push(comparison);
        self.operators.push(Operator::And);
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
    #[must_use]
    #[inline]
    pub fn or(mut self, comparison: Comparison) -> Self {
        self.comparisons.push(comparison);
        self.operators.push(Operator::Or);
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
    /// assert_eq!(filter.to_aql("i"), String::from(r#"i.age > 10 || i.username IN ["felix", "felixm"]"#));
    /// ```
    #[must_use]
    #[deprecated(note = "use `aql_str` instead")]
    pub fn to_aql(&self, collection_id: &str) -> String {
        self.aql_str(collection_id)
    }

    /// Renders the AQL string corresponding to the current `Filter`. The query will go out of scope.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Filter};
    /// let mut filter = Filter::new(Comparison::field("age").greater_than(10)).
    ///     or(Comparison::field("username").in_str_array(&["felix", "felixm"]));
    /// assert_eq!(filter.aql_str("i"), String::from(r#"i.age > 10 || i.username IN ["felix", "felixm"]"#));
    /// ```
    #[must_use]
    pub fn aql_str(&self, collection_id: &str) -> String {
        let mut res = String::new();
        for (i, comparison) in self.comparisons.iter().enumerate() {
            let operator_str = if i >= self.operators.len() {
                String::new()
            } else {
                format!(" {}", self.operators[i])
            };
            res = format!(
                "{} {}{}",
                res,
                comparison.aql_str(collection_id),
                operator_str
            );
        }
        String::from(res.trim_start())
    }
}

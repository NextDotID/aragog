use std::fmt::Display;
use num::Num;
use crate::query::{Filter, string_array_from_array, string_array_from_array_str};

/// Macro to simplify the [`Comparison`] construction:
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate aragog;
/// # use aragog::query::Comparison;
///
/// # fn main() {
/// // The following are equivalent:
/// let comparison = Comparison::field("field_name");
/// let comparison = compare!(field "field_name");
/// // The following are equivalent:
/// let comparison = Comparison::all("field_name");
/// let comparison = compare!(all "field_name");
/// // The following are equivalent:
/// let comparison = Comparison::any("field_name");
/// let comparison = compare!(any "field_name");
/// // The following are equivalent:
/// let comparison = Comparison::none("field_name");
/// let comparison = compare!(none "field_name");
/// // The following are equivalent:
/// let comparison = Comparison::statement("statement");
/// let comparison = compare!("statement");
/// # }
/// ```
///
/// [`Comparison`]: query/struct.Comparison.html
#[macro_export]
macro_rules! compare {
    ($value:expr) => {
        $crate::query::Comparison::statement($value)
    };
    (field $field_name:expr) => {
        $crate::query::Comparison::field($field_name)
    };
    (all $field_name:expr) => {
        $crate::query::Comparison::all($field_name)
    };
    (any $field_name:expr) => {
        $crate::query::Comparison::any($field_name)
    };
    (none $field_name:expr) => {
        $crate::query::Comparison::none($field_name)
    };
}

/// Builder for [`Comparison`]
///
/// [`Comparison`]: struct.Comparison.html
#[derive(Clone, Debug)]
pub struct ComparisonBuilder {
    is_field: bool,
    statement: String
}

/// Struct representing one AQL comparison in a [`Query`].
///
/// [`Query`]: struct.Query.html
#[derive(Clone, Debug)]
pub struct Comparison {
    is_field: bool,
    left_value: String,
    comparator: String,
    right_value: String
}

impl ComparisonBuilder {

    /// Finalizes the current query item builder with a string equality comparison.
    /// The field to be matched should be a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("username").equals_str("felix");
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn equals_str(self, value: &str) -> Comparison {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "==".to_string(),
            right_value: format!(r#""{}""#, value)
        }
    }

    /// Finalizes the current query item builder with a string inequality comparison.
    /// The field to be matched should be a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("username").different_than_str("felix");
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn different_than_str(self, value: &str) -> Comparison {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "!=".to_string(),
            right_value: format!(r#""{}""#, value)
        }
    }

    /// Finalizes the current query item builder with a regular expression matching.
    /// The field to be matched should be a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("username").matches(r#"^[0.9](0.6)$"#);
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn matches(self, regular_expression: &str) -> Comparison {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "=~".to_string(),
            right_value: format!(r#""{}""#, regular_expression)
        }
    }

    /// Finalizes the current query item builder with an inverse regular expression matching.
    /// The field to be matched should be a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("username").does_not_match(r#"^[0.9](0.6)$"#);
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn does_not_match(self, regular_expression: &str) -> Comparison {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "!~".to_string(),
            right_value: format!(r#""{}""#, regular_expression)
        }
    }

    /// Finalizes the current query item builder with string comparison.
    /// The field to be matched should be a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("username").like("%felix%");
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn like(self, pattern: &str) -> Comparison {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "LIKE".to_string(),
            right_value: format!(r#""{}""#, pattern)
        }
    }

    /// Finalizes the current query item builder with string comparison.
    /// The field to be matched should be a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("username").not_like("%felix%");
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn not_like(self, pattern: &str) -> Comparison {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "NOT LIKE".to_string(),
            right_value: format!(r#""{}""#, pattern)
        }
    }
    
    /// Finalizes the current query item builder with numeric equality comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("age").equals(18);
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn equals<T>(self, value: T) -> Comparison where T: Num + Display {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "==".to_string(),
            right_value: format!(r#"{}"#, value)
        }
    }

    /// Finalizes the current query item builder with numeric inquality comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("age").different_than(18);
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn different_than<T>(self, value: T) -> Comparison where T: Num + Display {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "!=".to_string(),
            right_value: format!(r#"{}"#, value)
        }
    }

    /// Finalizes the current query item builder with numeric comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("age").greater_than(18);
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn greater_than<T>(self, value: T) -> Comparison where T: Num + Display {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: ">".to_string(),
            right_value: format!(r#"{}"#, value)
        }
    }

    /// Finalizes the current query item builder with numeric comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("age").greater_or_equal(18);
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn greater_or_equal<T>(self, value: T) -> Comparison where T: Num + Display {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: ">=".to_string(),
            right_value: format!(r#"{}"#, value)
        }
    }

    /// Finalizes the current query item builder with numeric comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("age").lesser_than(18);
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn lesser_than<T>(self, value: T) -> Comparison where T: Num + Display {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "<".to_string(),
            right_value: format!(r#"{}"#, value)
        }
    }

    /// Finalizes the current query item builder with numeric comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("age").lesser_or_equal(18);
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn lesser_or_equal<T>(self, value: T) -> Comparison where T: Num + Display {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "<=".to_string(),
            right_value: format!(r#"{}"#, value)
        }
    }

    /// Finalizes the current query item builder with an inclusion in a numeric array comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("age").in_array(&[1, 11, 16, 18]);
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn in_array<T>(self, array: &[T]) -> Comparison where T: Num + Display {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "IN".to_string(),
            right_value: format!(r#"{}"#, string_array_from_array(array))
        }
    }

    /// Finalizes the current query item builder with an inclusion in a numeric array comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("age").not_in_array(&[1, 11, 16, 18]);
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn not_in_array<T>(self, array: &[T]) -> Comparison where T: Num + Display {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "NOT IN".to_string(),
            right_value: format!(r#"{}"#, string_array_from_array(array))
        }
    }

    /// Finalizes the current query item builder with an inclusion in a string array comparison.
    /// The field to be matched should be a string type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("username").in_str_array(&["felix", "123felix"]);
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn in_str_array(self, array: &[&str]) -> Comparison {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "IN".to_string(),
            right_value: format!(r#"{}"#, string_array_from_array_str(array))
        }
    }

    /// Finalizes the current query item builder with an inclusion in a string array comparison.
    /// The field to be matched should be a string type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("username").not_in_str_array(&["felix", "123felix"]);
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn not_in_str_array(self, array: &[&str]) -> Comparison {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "NOT IN".to_string(),
            right_value: format!(r#"{}"#, string_array_from_array_str(array))
        }
    }

    /// Finalizes the current query item builder with a `null` comparison.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("username").is_null();
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn is_null(self) -> Comparison {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "==".to_string(),
            right_value: "null".to_string()
        }
    }

    /// Finalizes the current query item builder with a not `null` comparison.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("username").not_null();
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn not_null(self) -> Comparison {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "!=".to_string(),
            right_value: "null".to_string()
        }
    }

    /// Finalizes the current query item builder with a boolean comparison.
    /// The field to be matched should be a boolean type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("is_authorized").is_true();
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn is_true(self) -> Comparison {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "==".to_string(),
            right_value: "true".to_string()
        }
    }

    /// Finalizes the current query item builder with a boolean comparison.
    /// The field to be matched should be a boolean type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    ///
    /// let query_item = Comparison::field("is_authorized").is_false();
    /// let query = Query::new("Users").filter(Filter::new(query_item));
    /// ```
    pub fn is_false(self) -> Comparison {
        Comparison {
            is_field: self.is_field,
            left_value: self.statement,
            comparator: "==".to_string(),
            right_value: "false".to_string()
        }
    }
}

impl Comparison {
    /// Instantiates a new builder for a `Comparison` with the specified `field_name`.
    /// The field will be used as the left value of the comparison.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Filter, Query};
    /// Query::new("Users").filter(Filter::new(Comparison::field("name").equals_str("felix")));
    /// // or
    /// Query::new("Users").filter(Comparison::field("name").equals_str("felix").into());
    /// ```
    pub fn field(field_name: &str) -> ComparisonBuilder {
        ComparisonBuilder {
            is_field: true,
            statement: format!("{}", field_name.to_string())
        }
    }

    /// Instantiates a new builder for a `Comparison` with the specified `array_field_name`.
    /// The field should be an array, as all items in the array will have to match the comparison
    /// to succeed.
    ///
    /// # Example
    ///
    /// In this example the query will render all documents where every price is above 10.
    /// ```rust
    /// # use aragog::query::{Comparison, Filter, Query};
    /// Query::new("Products").filter(Filter::new(Comparison::all("prices").greater_or_equal(10)));
    /// // or
    /// Query::new("Users").filter(Comparison::all("prices").greater_or_equal(10).into());
    /// ```
    pub fn all(array_field_name: &str) -> ComparisonBuilder {
        ComparisonBuilder {
            is_field: true,
            statement: format!("{} ALL", array_field_name)
        }
    }

    /// Instantiates a new builder for a `Comparison` with the specified `array_field_name`.
    /// The field should be an array, none of the items in the array can match the comparison to succeed.
    ///
    /// # Example
    ///
    /// In this example the query will render all documents where every price is not above 10.
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    /// Query::new("Products").filter(Filter::new(Comparison::none("prices").greater_or_equal(10)));
    /// // or
    /// Query::new("Users").filter(Comparison::none("prices").greater_or_equal(10).into());
    /// ```
    pub fn none(array_field_name: &str) -> ComparisonBuilder {
        ComparisonBuilder {
            is_field: true,
            statement: format!("{} NONE", array_field_name)
        }
    }
    /// Instantiates a new builder for a `Comparison` with the specified `array_field_name`.
    /// The field should be an array, at least one of the items in the array must match the
    /// comparison to succeed.
    ///
    /// # Example
    ///
    /// In this example the query will render all documents where at least one price is above 10.
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    /// Query::new("Products").filter(Filter::new(Comparison::any("prices").greater_or_equal(10)));
    /// // or
    /// Query::new("Users").filter(Comparison::any("prices").greater_or_equal(10).into());
    /// ```
    pub fn any(array_field_name: &str) -> ComparisonBuilder {
        ComparisonBuilder {
            is_field: true,
            statement: format!("{} ANY", array_field_name)
        }
    }

    /// Instantiates a new builder for a `Comparison` with the specified `statement`.
    /// The field will be used as the left value of the comparison.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    /// Query::new("Products").filter(Filter::new(Comparison::statement("10 * 3").greater_or_equal(10)));
    /// // or
    /// Query::new("Products").filter(Comparison::statement("10 * 3").greater_or_equal(10).into());
    /// ```
    pub fn statement(statement: &str) -> ComparisonBuilder {
        ComparisonBuilder {
            is_field: false,
            statement: statement.to_string()
        }
    }

    /// Appends the filter current condition(s) with a new one with a `AND` logic.
    /// `self` will be treated as a `Filter`.
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    /// /// Both are equivalent
    /// let a = Filter::new(Comparison::field("age").greater_than(10)).and(Comparison::field("age").lesser_or_equal(18));
    /// let b = Comparison::field("age").greater_than(10).and(Comparison::field("age").lesser_or_equal(18));
    /// assert_eq!(a.to_aql("i"), b.to_aql("i"));
    /// ```
    pub fn and(self, comparison: Comparison) -> Filter {
        Filter::new(self).and(comparison)
    }

    /// Appends the filter current condition(s) with a new one with a `OR` logic.
    /// `self` will be treated as a `Filter`.
    /// ```rust
    /// # use aragog::query::{Comparison, Query, Filter};
    /// /// Both are equivalent
    /// let a = Filter::new(Comparison::field("age").greater_than(10)).or(Comparison::field("age").lesser_or_equal(18));
    /// let b = Comparison::field("age").greater_than(10).or(Comparison::field("age").lesser_or_equal(18));
    /// assert_eq!(a.to_aql("i"), b.to_aql("i"));
    /// ```
    pub fn or(self, comparison: Comparison) -> Filter {
        Filter::new(self).or(comparison)
    }

    /// Renders `self` in a valid AQL format.
    /// `collection_id` is simply the collection identifier, it can be any string.
    ///
    /// # Note
    ///
    /// You should not use this method except for debug purpose, use [`Query`]::[`to_aql`] instead
    ///
    /// # Example
    ///
    /// ```rust
    /// use aragog::query::Comparison;
    /// let comparison = Comparison::field("age").greater_than(18);
    ///
    /// assert_eq!(comparison.to_aql("i").as_str(), "i.age > 18")
    /// ```
    ///
    /// [`Query`]: struct.Query.html
    /// [`to_aql`]: struct.Query.html#method.to_aql
    pub fn to_aql(&self, collection_id: &str) -> String {
        let id = if self.is_field {
            format!("{}.", collection_id)
        } else {
            String::new()
        };
        format!("{}{} {} {}", id, &self.left_value, &self.comparator, &self.right_value)
    }
}

impl From<Comparison> for Filter {
    fn from(comparison: Comparison) -> Self {
        Self::new(comparison)
    }
}
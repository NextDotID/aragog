use std::fmt::{Display, Formatter, Result};
use num::Num;

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
    field: String
}

/// Struct representing one AQL comparison in a [`Query`].
///
/// [`Query`]: struct.Query.html
#[derive(Clone, Debug)]
pub struct Comparison {
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
            left_value: self.field,
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
            left_value: self.field,
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
            left_value: self.field,
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
            left_value: self.field,
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
            left_value: self.field,
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
            left_value: self.field,
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
            left_value: self.field,
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
            left_value: self.field,
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
            left_value: self.field,
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
            left_value: self.field,
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
            left_value: self.field,
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
            left_value: self.field,
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
            left_value: self.field,
            comparator: "IN".to_string(),
            right_value: format!(r#"{}"#, Self::string_from_array(array))
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
            left_value: self.field,
            comparator: "NOT IN".to_string(),
            right_value: format!(r#"{}"#, Self::string_from_array(array))
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
            left_value: self.field,
            comparator: "IN".to_string(),
            right_value: format!(r#"{}"#, Self::string_from_array_str(array))
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
            left_value: self.field,
            comparator: "NOT IN".to_string(),
            right_value: format!(r#"{}"#, Self::string_from_array_str(array))
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
            left_value: self.field,
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
            left_value: self.field,
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
            left_value: self.field,
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
            left_value: self.field,
            comparator: "==".to_string(),
            right_value: "false".to_string()
        }
    }

    fn string_from_array<T>(array: &[T]) -> String where T: Num + Display {
        let mut array_str = String::from("[");
        for (i, element) in array.iter().enumerate() {
            array_str = format!("{}{}", array_str, element);
            if i < array.len() - 1 { array_str += ", " }
        }
        array_str += "]";
        array_str
    }

    fn string_from_array_str(array: &[&str]) -> String where {
        let mut array_str = String::from("[");
        for (i, element) in array.iter().enumerate() {
            array_str = format!(r#"{}"{}""#, array_str, element);
            if i < array.len() - 1 { array_str += ", " }
        }
        array_str += "]";
        array_str
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
    /// ```
    pub fn field(field_name: &str) -> ComparisonBuilder {
        ComparisonBuilder {
            field: format!("i.{}", field_name.to_string())
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
    /// ```
    pub fn all(array_field_name: &str) -> ComparisonBuilder {
        ComparisonBuilder {
            field: format!("i.{} ALL", array_field_name)
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
    /// ```
    pub fn none(array_field_name: &str) -> ComparisonBuilder {
        ComparisonBuilder {
            field: format!("i.{} NONE", array_field_name)
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
    /// ```
    pub fn any(array_field_name: &str) -> ComparisonBuilder {
        ComparisonBuilder {
            field: format!("i.{} ANY", array_field_name)
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
    /// ```
    pub fn statement(statement: &str) -> ComparisonBuilder {
        ComparisonBuilder {
            field: statement.to_string()
        }
    }

}

impl Display for Comparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.left_value, self.comparator, self.right_value)
    }
}
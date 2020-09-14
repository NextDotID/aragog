use std::fmt::{Display, Formatter, Result};
use num::Num;

/// Builder for [`QueryItem`]
///
/// [`QueryItem`]: struct.QueryItem.html
#[derive(Clone, Debug)]
pub struct QueryItemBuilder {
    field: String
}

/// Struct representing one AQL comparison in a [`Query`].
///
/// [`Query`]: struct.Query.html
#[derive(Clone, Debug)]
pub struct QueryItem {
    field: String,
    comparator: String,
    value: String
}

impl QueryItemBuilder {
    /// Instantiates a new builder for [`QueryItem`] with the specified `field_name`
    ///
    /// [`QueryItem`]: struct.QueryItem.html
    pub fn field(field_name: &str) -> Self {
        Self {
            field: String::from(field_name),
        }
    }

    /// Finalizes the current query item builder with a string equality comparison.
    /// The field to be matched should be a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// println!("we are in the test");
    /// let query_item = QueryItem::field("username").equals_str("felix");
    /// let query = Query::new(query_item);
    /// ```
    pub fn equals_str(self, value: &str) -> QueryItem {
        QueryItem {
            field: self.field,
            comparator: "==".to_string(),
            value: format!(r#""{}""#, value)
        }
    }

    /// Finalizes the current query item builder with a string inequality comparison.
    /// The field to be matched should be a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("username").different_than_str("felix");
    /// let query = Query::new(query_item);
    /// ```
    pub fn different_than_str(self, value: &str) -> QueryItem {
        QueryItem {
            field: self.field,
            comparator: "!=".to_string(),
            value: format!(r#""{}""#, value)
        }
    }

    /// Finalizes the current query item builder with a regular expression matching.
    /// The field to be matched should be a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("username").matches(r#"^[0.9](0.6)$"#);
    /// let query = Query::new(query_item);
    /// ```
    pub fn matches(self, regular_expression: &str) -> QueryItem {
        QueryItem {
            field: self.field,
            comparator: "=~".to_string(),
            value: format!(r#""{}""#, regular_expression)
        }
    }

    /// Finalizes the current query item builder with an inverse regular expression matching.
    /// The field to be matched should be a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("username").does_not_match(r#"^[0.9](0.6)$"#);
    /// let query = Query::new(query_item);
    /// ```
    pub fn does_not_match(self, regular_expression: &str) -> QueryItem {
        QueryItem {
            field: self.field,
            comparator: "!~".to_string(),
            value: format!(r#""{}""#, regular_expression)
        }
    }

    /// Finalizes the current query item builder with string comparison.
    /// The field to be matched should be a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("username").like("%felix%");
    /// let query = Query::new(query_item);
    /// ```
    pub fn like(self, pattern: &str) -> QueryItem {
        QueryItem {
            field: self.field,
            comparator: "LIKE".to_string(),
            value: format!(r#""{}""#, pattern)
        }
    }

    /// Finalizes the current query item builder with string comparison.
    /// The field to be matched should be a string.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("username").not_like("%felix%");
    /// let query = Query::new(query_item);
    /// ```
    pub fn not_like(self, pattern: &str) -> QueryItem {
        QueryItem {
            field: self.field,
            comparator: "NOT LIKE".to_string(),
            value: format!(r#""{}""#, pattern)
        }
    }
    
    /// Finalizes the current query item builder with numeric equality comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("age").equals(18);
    /// let query = Query::new(query_item);
    /// ```
    pub fn equals<T>(self, value: T) -> QueryItem where T: Num + Display {
        QueryItem {
            field: self.field,
            comparator: "==".to_string(),
            value: format!(r#"{}"#, value)
        }
    }

    /// Finalizes the current query item builder with numeric inquality comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("age").different_than(18);
    /// let query = Query::new(query_item);
    /// ```
    pub fn different_than<T>(self, value: T) -> QueryItem where T: Num + Display {
        QueryItem {
            field: self.field,
            comparator: "!=".to_string(),
            value: format!(r#"{}"#, value)
        }
    }

    /// Finalizes the current query item builder with numeric comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("age").greater_than(18);
    /// let query = Query::new(query_item);
    /// ```
    pub fn greater_than<T>(self, value: T) -> QueryItem where T: Num + Display {
        QueryItem {
            field: self.field,
            comparator: ">".to_string(),
            value: format!(r#"{}"#, value)
        }
    }

    /// Finalizes the current query item builder with numeric comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("age").greater_or_equal(18);
    /// let query = Query::new(query_item);
    /// ```
    pub fn greater_or_equal<T>(self, value: T) -> QueryItem where T: Num + Display {
        QueryItem {
            field: self.field,
            comparator: ">=".to_string(),
            value: format!(r#"{}"#, value)
        }
    }

    /// Finalizes the current query item builder with numeric comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("age").lesser_than(18);
    /// let query = Query::new(query_item);
    /// ```
    pub fn lesser_than<T>(self, value: T) -> QueryItem where T: Num + Display {
        QueryItem {
            field: self.field,
            comparator: "<".to_string(),
            value: format!(r#"{}"#, value)
        }
    }

    /// Finalizes the current query item builder with numeric comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("age").lesser_or_equal(18);
    /// let query = Query::new(query_item);
    /// ```
    pub fn lesser_or_equal<T>(self, value: T) -> QueryItem where T: Num + Display {
        QueryItem {
            field: self.field,
            comparator: "<=".to_string(),
            value: format!(r#"{}"#, value)
        }
    }

    /// Finalizes the current query item builder with an inclusion in a numeric array comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("age").in_array(&[1, 11, 16, 18]);
    /// let query = Query::new(query_item);
    /// ```
    pub fn in_array<T>(self, array: &[T]) -> QueryItem where T: Num + Display {
        QueryItem {
            field: self.field,
            comparator: "IN".to_string(),
            value: format!(r#"{}"#, Self::string_from_array(array))
        }
    }

    /// Finalizes the current query item builder with an inclusion in a numeric array comparison.
    /// The field to be matched should be a numeric type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("age").not_in_array(&[1, 11, 16, 18]);
    /// let query = Query::new(query_item);
    /// ```
    pub fn not_in_array<T>(self, array: &[T]) -> QueryItem where T: Num + Display {
        QueryItem {
            field: self.field,
            comparator: "NOT IN".to_string(),
            value: format!(r#"{}"#, Self::string_from_array(array))
        }
    }

    /// Finalizes the current query item builder with an inclusion in a string array comparison.
    /// The field to be matched should be a string type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("username").in_str_array(&["felix", "123felix"]);
    /// let query = Query::new(query_item);
    /// ```
    pub fn in_str_array(self, array: &[&str]) -> QueryItem {
        QueryItem {
            field: self.field,
            comparator: "IN".to_string(),
            value: format!(r#"{}"#, Self::string_from_array_str(array))
        }
    }

    /// Finalizes the current query item builder with an inclusion in a string array comparison.
    /// The field to be matched should be a string type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::{QueryItem, Query};
    ///
    /// let query_item = QueryItem::field("username").not_in_str_array(&["felix", "123felix"]);
    /// let query = Query::new(query_item);
    /// ```
    pub fn not_in_str_array(self, array: &[&str]) -> QueryItem {
        QueryItem {
            field: self.field,
            comparator: "NOT IN".to_string(),
            value: format!(r#"{}"#, Self::string_from_array_str(array))
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

impl QueryItem {
    /// Instantiates a new builder for a `QueryItem` with the specified `field_name`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::query::QueryItem;
    /// let query_item_builder = QueryItem::field("username");
    /// ```
    pub fn field(field_name: &str) -> QueryItemBuilder {
        QueryItemBuilder::field(field_name)
    }
}

impl Display for QueryItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.field, self.comparator, self.value)
    }
}
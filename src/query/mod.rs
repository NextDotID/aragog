use std::fmt::Display;

pub use {
    comparison::Comparison,
    comparison::ComparisonBuilder,
    filter::Filter,
    query::Query,
    query::SortDirection,
    query_result::JsonQueryResult,
    query_result::RecordQueryResult,
};

mod query;
mod comparison;
mod filter;
mod query_result;
mod graph_query;
mod query_id_helper;
mod operations;

#[derive(Clone, Debug)]
struct OptionalQueryString(Option<String>);

fn string_array_from_array<T>(array: &[T]) -> String where T: Display {
    format!("[{}]", string_from_array(array))
}

fn string_from_array<T>(array: &[T]) -> String where T: Display {
    let mut array_str = String::new();
    for (i, element) in array.iter().enumerate() {
        array_str = format!("{}{}", array_str, element);
        if i < array.len() - 1 { array_str += ", " }
    }
    array_str
}

fn string_array_from_array_str(array: &[&str]) -> String where {
    let mut array_str = String::from("[");
    for (i, element) in array.iter().enumerate() {
        array_str = format!(r#"{}"{}""#, array_str, element);
        if i < array.len() - 1 { array_str += ", " }
    }
    array_str += "]";
    array_str
}

impl ToString for OptionalQueryString {
    fn to_string(&self) -> String {
        match &self.0 {
            Some(str) => str.clone(),
            None => String::new()
        }
    }
}
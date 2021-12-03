use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct OptionalQueryString(pub Option<String>);

pub fn string_array_from_array<T>(array: &[T]) -> String
where
    T: Display,
{
    format!("[{}]", string_from_array(array))
}

pub fn string_from_array<T>(array: &[T]) -> String
where
    T: Display,
{
    let mut array_str = String::new();
    for (i, element) in array.iter().enumerate() {
        array_str = format!("{}{}", array_str, element);
        if i < array.len() - 1 {
            array_str += ", ";
        }
    }
    array_str
}

pub fn string_array_from_array_str<T>(array: &[T]) -> String
where
    T: Display,
{
    let mut array_str = String::from("[");
    for (i, element) in array.iter().enumerate() {
        array_str = format!(r#"{}"{}""#, array_str, element);
        if i < array.len() - 1 {
            array_str += ", ";
        }
    }
    array_str += "]";
    array_str
}

impl ToString for OptionalQueryString {
    fn to_string(&self) -> String {
        match &self.0 {
            Some(str) => str.clone(),
            None => String::new(),
        }
    }
}

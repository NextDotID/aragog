use serde_json::Value;

/// Retrieves a string from a json value
///
/// # Arguments:
///
/// * `element` - json value containing the `key`
/// * `key` - string slice representing the json key
///
/// # Returns
///
/// Returns the json value as a String on success, returns a String error message on failure.
pub fn load_json_string_key(element: &Value, key: &str) -> Result<String, String> {
    if let Value::String(value) = &element[key] {
        return Ok(value.clone());
    }
    Err(format!("Failed to load key {}", key))
}

/// Converts a json value to string without format issues
///
/// # Arguments:
///
/// * `element` - json value to convert to string
///
/// # Returns
///
/// Returns the json value as a String on success, returns a String error message on failure.
pub fn load_json_string(element: &Value) -> Result<String, String> {
    if let Value::String(value) = &element {
        return Ok(value.clone());
    }
    Err(format!("Failed to load value {}", element))
}

#[allow(dead_code)]
/// Retrieves an environment varirable from a json string
///
/// The function uses [`load_json_string_key`] to retrieve a key string then [`get_env_var`] for the
/// environment variable value.
///
/// # Arguments:
///
/// * `element` - json value containing the `key`
/// * `key` - string slice representing the json key
///
/// # Panics
///
/// The function panics if the env var doesn't exist. see [`get_env_var`] for more information.
///
/// # Returns
///
/// Returns the env var value on success,  returns a String error message on failure.
///
/// [`load_json_string_key`]: ./fn.load_json_string_key.html
/// [`get_env_var`]: ../toolbox/fn.get_env_var.html
pub fn load_json_string_as_env(element :&Value, key: &str) -> Result<String, String> {
    let str = load_json_string_key(&element, &key)?;
    Ok(std::env::var(&str).unwrap_or(format!("{} is not set in environment", key)))
}
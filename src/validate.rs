use crate::ServiceError;

/// The `Validate` trait of the Aragog library.
/// This trait provides the possibility to validate an instance or its fields formats or logic. Its main use
/// it to validate a new or updated [`Record`] model instance before saving it.
///
/// # Example
///
/// ```rust
/// # use aragog::{Record, Validate};
/// # use serde::{Deserialize, Serialize};
/// use aragog::helpers::string_validators::*;
///
/// #[derive(Record, Clone, Deserialize, Serialize)]
/// pub struct User {
///     pub name: String,
///     pub age: u32,
///     pub email: String,
///     pub job: Option<String>,
///     pub phone: String,
/// }
///
/// impl Validate for User {
///     fn validations(&self, errors: &mut Vec<String>) {
///         if self.age > 18 {
///             Self::validate_field_presence("job", &self.job, errors);
///         }
///         validate_min_len("name", &self.name, 6, errors);
///         validate_len("phone", &self.phone, 10, errors);
///         validate_numeric_string("phone", &self.phone, errors);
///         if self.age < 13 {
///             errors.push("You are too young to use this website".to_string());
///         }
///     }
/// }
/// ```
/// [`Record`]: record/trait.Record.html
pub trait Validate {
    /// Validates the object field formats, logic or anything. Calls the [`validations`] method
    /// and will render a complete [`ServiceError`]::[`ValidationError`] on validation failure.
    /// On success returns `()`
    ///
    /// [`validations`]: trait.Validate.html#tymethod.validations
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`ValidationError`]: enum.ServiceError.html#variant.ValidationError
    fn validate(&self) -> Result<(), ServiceError>
    {
        let mut errors: Vec<String> = Vec::new();

        self.validations(&mut errors);

        if errors.is_empty() { Ok(()) }
        else {
            let error_str = errors.join(", ");
            log::error!("{}", &error_str);
            Err(ServiceError::ValidationError(error_str))
        }
    }

    /// Runs all the defined validation on fields and fills the `errors` string vector with custom error messages
    fn validations(&self, errors: &mut Vec<String>);

    /// Runs all validations and returns a `false` if they failed, on success `true` is returned
    fn is_valid(&self) -> bool {
        match self.validate() {
            Ok(()) => true,
            Err(_err) => false
        }
    }

    /// Helper function to simply check the presence of a field. This function is usually used inside the
    /// [`validations`] method since it will fill the `errors` with a message if the `field` is missing.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The string slice name of the field, will be used in the error message on failure
    /// * `field` - Optional value, if `field` is `Some<T>` the function will succeed
    /// * `errors` - the mutable reference of the error message vector like provided in [`validations`]
    ///
    /// # Returns
    ///
    /// `true` if `field` is `Some<T>` on failure, `false` is returned and `errors` stored a new message
    ///
    /// [`validations`]: trait.Validate.html#tymethod.validations
    fn validate_field_presence<T>(field_name: &str, field: &Option<T>, errors: &mut Vec<String>) -> bool {
        match field {
            Some(_value) => { true }
            None => {
                errors.push(format!("{} is missing", field_name));
                false
            }
        }
    }

    /// Helper function to simply check the absence of a field. This function is usually used inside the
    /// [`validations`] method since it will fill the `errors` with a message if the `field` is missing.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The string slice name of the field, will be used in the error message on failure
    /// * `field` - Optional value, if `field` is `None` the function will succeed
    /// * `errors` - the mutable reference of the error message vector like provided in [`validations`]
    ///
    /// # Returns
    ///
    /// `true` if `field` is `None` on failure, `false` is returned and `errors` stored a new message
    ///
    /// [`validations`]: trait.Validate.html#tymethod.validations
    fn validate_field_absence<T>(field_name: &str, field: &Option<T>, errors: &mut Vec<String>) -> bool {
        match field {
            Some(_value) => {
                errors.push(format!("{} should not be set", field_name));
                false
            }
            None => { true }
        }
    }
}
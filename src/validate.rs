use crate::AragogServiceError;

/// The `Validate` trait of the Aragog library.
/// This trait provides the possibility to validate an instance or its fields formats or logic. Its main use
/// it to validate a new or updated [`Record`] model instance before saving it.
///
/// [`Record`]: record/trait.Record.html
pub trait Validate {
    /// Validates the object field formats, logic or anything. Calls the [`validations`] method
    /// and will render a complete [`AragogServiceError`]::[`ValidationError`] on validation failure.
    /// On success returns `()`
    ///
    /// [`validations`]: trait.Validate.html#tymethod.validations
    /// [`AragogServiceError`]: enum.AragogServiceError.html
    /// [`ValidationError`]: enum.AragogServiceError.html#variant.ValidationError
    fn validate(&self) -> Result<(), AragogServiceError>
    {
        let mut errors: Vec<String> = Vec::new();

        self.validations(&mut errors);

        if errors.is_empty() {
            Ok(())
        }
        else {
            let error_str = errors.join(", ");
            log::error!("{}", &error_str);
            Err(AragogServiceError::ValidationError(error_str))
        }
    }

    /// Runs all the defined validation on fields and fills the `errors` string vector with custom error messages
    fn validations(&self, errors: &mut Vec<String>);

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
}
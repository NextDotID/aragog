use regex::Regex;

use crate::{EdgeRecord, ServiceError};
use std::fmt::Display;

/// The `Validate` trait of the Aragog library.
/// This trait provides the possibility to validate an instance or its fields formats or logic. Its main use
/// it to validate a new or updated [`Record`] model instance before saving it.
///
/// # Example
///
/// ```rust
/// # use aragog::{Record, Validate};
/// # use serde::{Deserialize, Serialize};
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
///         Self::validate_min_len("name", &self.name, 6, errors);
///         Self::validate_len("phone", &self.phone, 10, errors);
///         Self::validate_numeric_string("phone", &self.phone, errors);
///         if self.age < 13 {
///             errors.push("You are too young to use this website".to_string());
///         }
///     }
/// }
/// ```
/// [`Record`]: trait.Record.html
pub trait Validate {
    /// Validates the object field formats, logic or anything. Calls the [`validations`] method
    /// and will render a complete [`ServiceError`]::[`ValidationError`] on validation failure.
    /// On success returns `()`
    ///
    /// [`validations`]: trait.Validate.html#tymethod.validations
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`ValidationError`]: enum.ServiceError.html#variant.ValidationError
    fn validate(&self) -> Result<(), ServiceError> {
        let mut errors: Vec<String> = Vec::new();

        self.validations(&mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
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
            Err(_err) => false,
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
    fn validate_field_presence<T>(
        field_name: &str,
        field: &Option<T>,
        errors: &mut Vec<String>,
    ) -> bool {
        match field {
            Some(_value) => true,
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
    fn validate_field_absence<T>(
        field_name: &str,
        field: &Option<T>,
        errors: &mut Vec<String>,
    ) -> bool {
        match field {
            Some(_value) => {
                errors.push(format!("{} should not be set", field_name));
                false
            }
            None => true,
        }
    }

    /// Validates that `str` is numeric. Usually used as a helper function for implementations of
    /// [`Validate`] trait.
    ///
    /// # Arguments
    ///
    /// * `field_path` - the string slice representing the field name or path for clear errors
    /// * `str` - the field value to validate
    /// * `errors` - a mutable reference to a vector of String to be filled with error messages like provided
    /// in [`Validate`]::[`validations`]
    ///
    /// # Returns
    ///
    /// On success `true` is returned and `errors` stays unchanged. On failure `false` is returned and a
    /// new error message is added to `errors`
    ///
    /// [`Validate`]: trait.Validate.html
    /// [`validations`]: trait.Validate.html#tymethod.validations
    #[allow(dead_code)]
    fn validate_numeric_string(field_path: &str, str: &str, errors: &mut Vec<String>) -> bool {
        for char in str.chars() {
            if !char.is_ascii_digit() {
                errors.push(format!("{} '{}' is not numeric", field_path, str));
                return false;
            }
        }
        true
    }

    /// Validates that `str` is not longer than expected. Usually used as a helper function for implementations of
    /// [`Validate`] trait.
    ///
    /// # Arguments
    ///
    /// * `field_path` - the string slice representing the field name or path for clear errors
    /// * `str` - the field value to validate
    /// * `max_len` - The maximum length of `str`
    /// * `errors` - a mutable reference to a vector of String to be filled with error messages like provided
    /// in [`Validate`]::[`validations`]
    ///
    /// # Returns
    ///
    /// On success `true` is returned and `errors` stays unchanged. On failure `false` is returned and a
    /// new error message is added to `errors`
    ///
    /// [`Validate`]: trait.Validate.html
    /// [`validations`]: trait.Validate.html#tymethod.validations
    #[allow(dead_code)]
    fn validate_max_len(
        field_path: &str,
        str: &str,
        max_len: usize,
        errors: &mut Vec<String>,
    ) -> bool {
        if str.len() > max_len {
            errors.push(format!(
                "{} '{}' is too long, max length: {}",
                field_path, str, max_len
            ));
            return false;
        }
        true
    }

    /// Validates that `str` is not shorter than expected. Usually used as a helper function for implementations of
    /// [`Validate`] trait.
    ///
    /// # Arguments
    ///
    /// * `field_path` - the string slice representing the field name or path for clear errors
    /// * `str` - the field value to validate
    /// * `min_len` - The minimum length of `str`
    /// * `errors` - a mutable reference to a vector of String to be filled with error messages like provided
    /// in [`Validate`]::[`validations`]
    ///
    /// # Returns
    ///
    /// On success `true` is returned and `errors` stays unchanged. On failure `false` is returned and a
    /// new error message is added to `errors`
    ///
    /// [`Validate`]: trait.Validate.html
    /// [`validations`]: trait.Validate.html#tymethod.validations
    #[allow(dead_code)]
    fn validate_min_len(
        field_path: &str,
        str: &str,
        min_len: usize,
        errors: &mut Vec<String>,
    ) -> bool {
        if str.len() < min_len {
            errors.push(format!(
                "{} '{}' is too short, min length: {}",
                field_path, str, min_len
            ));
            return false;
        }
        true
    }

    /// Validates that `str` has the exact expected length. Usually used as a helper function for implementations of
    /// [`Validate`] trait.
    ///
    /// # Arguments
    ///
    /// * `field_path` - the string slice representing the field name or path for clear errors
    /// * `str` - the field value to validate
    /// * `len` - The expected length of `str`
    /// * `errors` - a mutable reference to a vector of String to be filled with error messages like provided
    /// in [`Validate`]::[`validations`]
    ///
    /// # Returns
    ///
    /// On success `true` is returned and `errors` stays unchanged. On failure `false` is returned and a
    /// new error message is added to `errors`
    ///
    /// [`Validate`]: trait.Validate.html
    /// [`validations`]: trait.Validate.html#tymethod.validations
    #[allow(dead_code)]
    fn validate_len(field_path: &str, str: &str, len: usize, errors: &mut Vec<String>) -> bool {
        if str.len() != len {
            errors.push(format!(
                "{} '{}' has wrong length, please specify {} characters",
                field_path, str, len
            ));
            return false;
        }
        true
    }

    /// Validates that `str` matches a regexp. Usually used as a helper function for implementations of
    /// [`Validate`] trait.
    ///
    /// # Arguments
    ///
    /// * `field_path` - the string slice representing the field name or path for clear errors
    /// * `str` - the field value to validate
    /// * `regex` - The regular expression `str` must match
    /// * `errors` - a mutable reference to a vector of String to be filled with error messages like provided
    /// in [`Validate`]::[`validations`]
    ///
    /// # Returns
    ///
    /// On success `true` is returned and `errors` stays unchanged. On failure `false` is returned and a
    /// new error message is added to `errors`
    ///
    /// [`Validate`]: trait.Validate.html
    /// [`validations`]: trait.Validate.html#tymethod.validations
    #[allow(dead_code)]
    fn validate_regex(field_path: &str, str: &str, regex: &str, errors: &mut Vec<String>) -> bool {
        let reg = match Regex::new(regex) {
            Ok(value) => value,
            Err(error) => {
                log::error!("FATAL: Wrong regex: {}", error);
                return false;
            }
        };
        let result = reg.is_match(str);
        if result {
            return true;
        }
        errors.push(format!("{} '{}' has incorrect format", field_path, str));
        false
    }

    /// Validates that `value` is greater than `min_value`. Usually used as a helper function for implementations of
    /// [`Validate`] trait.
    ///
    /// # Arguments
    ///
    /// * `field_path` - the string slice representing the field name or path for clear errors
    /// * `value` - the field value to validate
    /// * `min_value` - The comparison value
    /// * `errors` - a mutable reference to a vector of String to be filled with error messages like provided
    /// in [`Validate`]::[`validations`]
    ///
    /// # Returns
    ///
    /// On success `true` is returned and `errors` stays unchanged. On failure `false` is returned and a
    /// new error message is added to `errors`
    ///
    /// [`Validate`]: trait.Validate.html
    /// [`validations`]: trait.Validate.html#tymethod.validations
    #[allow(dead_code)]
    fn validate_greater_than<T: PartialOrd + Display>(
        field_path: &str,
        value: T,
        min_value: T,
        errors: &mut Vec<String>,
    ) -> bool {
        if value <= min_value {
            errors.push(format!(
                "{} '{}' must be greater than {}",
                field_path, value, min_value
            ));
            return false;
        }
        true
    }

    /// Validates that `value` is greater or equal to `min_value`. Usually used as a helper function for implementations of
    /// [`Validate`] trait.
    ///
    /// # Arguments
    ///
    /// * `field_path` - the string slice representing the field name or path for clear errors
    /// * `value` - the field value to validate
    /// * `min_value` - The comparison value
    /// * `errors` - a mutable reference to a vector of String to be filled with error messages like provided
    /// in [`Validate`]::[`validations`]
    ///
    /// # Returns
    ///
    /// On success `true` is returned and `errors` stays unchanged. On failure `false` is returned and a
    /// new error message is added to `errors`
    ///
    /// [`Validate`]: trait.Validate.html
    /// [`validations`]: trait.Validate.html#tymethod.validations
    #[allow(dead_code)]
    fn validate_greater_or_equal_to<T: PartialOrd + Display>(
        field_path: &str,
        value: T,
        min_value: T,
        errors: &mut Vec<String>,
    ) -> bool {
        if value < min_value {
            errors.push(format!(
                "{} '{}' must be greater or equal to {}",
                field_path, value, min_value
            ));
            return false;
        }
        true
    }

    /// Validates that `value` is lower than `max_value`. Usually used as a helper function for implementations of
    /// [`Validate`] trait.
    ///
    /// # Arguments
    ///
    /// * `field_path` - the string slice representing the field name or path for clear errors
    /// * `value` - the field value to validate
    /// * `max_value` - The comparison value
    /// * `errors` - a mutable reference to a vector of String to be filled with error messages like provided
    /// in [`Validate`]::[`validations`]
    ///
    /// # Returns
    ///
    /// On success `true` is returned and `errors` stays unchanged. On failure `false` is returned and a
    /// new error message is added to `errors`
    ///
    /// [`Validate`]: trait.Validate.html
    /// [`validations`]: trait.Validate.html#tymethod.validations
    #[allow(dead_code)]
    fn validate_lesser_than<T: PartialOrd + Display>(
        field_path: &str,
        value: T,
        max_value: T,
        errors: &mut Vec<String>,
    ) -> bool {
        if value >= max_value {
            errors.push(format!(
                "{} '{}' must be lower than {}",
                field_path, value, max_value
            ));
            return false;
        }
        true
    }

    /// Validates that `value` is lower or equal to `max_value`. Usually used as a helper function for implementations of
    /// [`Validate`] trait.
    ///
    /// # Arguments
    ///
    /// * `field_path` - the string slice representing the field name or path for clear errors
    /// * `value` - the field value to validate
    /// * `max_value` - The comparison value
    /// * `errors` - a mutable reference to a vector of String to be filled with error messages like provided
    /// in [`Validate`]::[`validations`]
    ///
    /// # Returns
    ///
    /// On success `true` is returned and `errors` stays unchanged. On failure `false` is returned and a
    /// new error message is added to `errors`
    ///
    /// [`Validate`]: trait.Validate.html
    /// [`validations`]: trait.Validate.html#tymethod.validations
    #[allow(dead_code)]
    fn validate_lesser_or_equal_to<T: PartialOrd + Display>(
        field_path: &str,
        value: T,
        max_value: T,
        errors: &mut Vec<String>,
    ) -> bool {
        if value > max_value {
            errors.push(format!(
                "{} '{}' must be lower or equal to {}",
                field_path, value, max_value
            ));
            return false;
        }
        true
    }

    /// Validation method for `EdgeRecord` to use on [`Validate`] implementation.
    /// Verifies that both `_from` and `_to` fields have correct format.
    fn validate_edge_fields(&self, errors: &mut Vec<String>)
    where
        Self: EdgeRecord,
    {
        let array = [("_from", self._from()), ("_to", self._to())];
        for (name, field) in array.iter() {
            let split = field.split('/').collect::<Vec<&str>>();
            if split.len() != 2 {
                errors.push(format!(r#"{} "{}" has wrong format"#, name, field));
            }
            let left_part = split.first().unwrap();
            Self::validate_min_len(name, left_part, 2, errors);
            let right_part = split.last().unwrap();
            Self::validate_min_len(name, right_part, 2, errors);
            Self::validate_numeric_string(name, right_part, errors);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;

    const STRING_EMPTY: &str = "";

    struct TestElem;

    impl Validate for TestElem {
        fn validations(&self, _errors: &mut Vec<String>) {}
    }

    mod string_validators {
        use super::*;

        mod numeric_string {
            use super::*;

            #[test]
            fn validates_only_numeric_characters() {
                let mut errors = Vec::new();
                let correct_str = "0123456789";
                let wrong_strs = ["+33122334", "01 02 03", "1.23"];

                assert_eq!(
                    TestElem::validate_numeric_string(STRING_EMPTY, correct_str, &mut errors),
                    true
                );
                assert_eq!(errors.is_empty(), true);
                for wrong_str in wrong_strs.iter() {
                    assert_eq!(
                        TestElem::validate_numeric_string(STRING_EMPTY, wrong_str, &mut errors),
                        false
                    );
                    assert_eq!(errors.is_empty(), false);
                    errors.pop().unwrap();
                    assert_eq!(errors.is_empty(), true);
                }
            }
        }

        mod max_length {
            use super::*;

            #[test]
            fn validates_only_correct_strings() {
                let mut errors = Vec::new();
                let max = 10;
                let correct_strs = ["hello", "foo1", "bar-++", "0102030405"];
                let wrong_strs = [
                    "hello678911",
                    "foobarfoobar1",
                    "bar-+123lkdzacdeee+",
                    "010203040005",
                ];

                for correct_str in correct_strs.iter() {
                    assert_eq!(
                        TestElem::validate_max_len(
                            STRING_EMPTY,
                            &String::from(correct_str.deref()),
                            max,
                            &mut errors,
                        ),
                        true
                    );
                    assert_eq!(errors.is_empty(), true);
                }
                for wrong_str in wrong_strs.iter() {
                    assert_eq!(
                        TestElem::validate_max_len(
                            STRING_EMPTY,
                            &String::from(wrong_str.deref()),
                            max,
                            &mut errors,
                        ),
                        false
                    );
                    assert_eq!(errors.is_empty(), false);
                    errors.pop().unwrap();
                    assert_eq!(errors.is_empty(), true);
                }
            }
        }

        mod min_length {
            use super::*;

            #[test]
            fn validates_only_correct_strings() {
                let mut errors = Vec::new();
                let min = 10;
                let correct_strs = [
                    "hello678911",
                    "foobarfoobar1",
                    "bar-+123lkdzacdeee+",
                    "010203040005",
                ];
                let wrong_strs = ["hello", "foo1", "bar-++", "010203040"];

                for correct_str in correct_strs.iter() {
                    assert_eq!(
                        TestElem::validate_min_len(
                            STRING_EMPTY,
                            &String::from(correct_str.deref()),
                            min,
                            &mut errors,
                        ),
                        true
                    );
                    assert_eq!(errors.is_empty(), true);
                }
                for wrong_str in wrong_strs.iter() {
                    assert_eq!(
                        TestElem::validate_min_len(
                            STRING_EMPTY,
                            &String::from(wrong_str.deref()),
                            min,
                            &mut errors,
                        ),
                        false
                    );
                    assert_eq!(errors.is_empty(), false);
                    errors.pop().unwrap();
                    assert_eq!(errors.is_empty(), true);
                }
            }
        }

        mod exact_length {
            use super::*;

            #[test]
            fn validates_only_correct_strings() {
                let mut errors = Vec::new();
                let length = 10;
                let correct_strs = ["0102030405", "a-bct09a=(", "felix faur"];
                let wrong_strs = ["hello", "foo1barbarbar", "bar-++", "01020304051123"];

                for correct_str in correct_strs.iter() {
                    assert_eq!(
                        TestElem::validate_len(
                            STRING_EMPTY,
                            &String::from(correct_str.deref()),
                            length,
                            &mut errors,
                        ),
                        true
                    );
                    assert_eq!(errors.is_empty(), true);
                }
                for wrong_str in wrong_strs.iter() {
                    assert_eq!(
                        TestElem::validate_len(
                            STRING_EMPTY,
                            &String::from(wrong_str.deref()),
                            length,
                            &mut errors,
                        ),
                        false
                    );
                    assert_eq!(errors.is_empty(), false);
                    errors.pop().unwrap();
                    assert_eq!(errors.is_empty(), true);
                }
            }
        }

        mod regex {
            use super::*;

            const REGEX: &str = "^[a-z]{1,10}$";

            #[test]
            fn validate_regex_works() {
                let mut errors = Vec::new();
                let valid_strs = ["abc", "hellothere", "felix"];
                let wrong_strs = ["abc1", "hellotherebro", "felix de_manevi11e", "a bc"];

                for valid_str in valid_strs.iter() {
                    assert_eq!(
                        TestElem::validate_regex(
                            STRING_EMPTY,
                            valid_str.deref(),
                            REGEX,
                            &mut errors
                        ),
                        true
                    );
                    assert!(errors.is_empty());
                }
                for wrong_str in wrong_strs.iter() {
                    assert_eq!(
                        TestElem::validate_regex(
                            STRING_EMPTY,
                            wrong_str.deref(),
                            REGEX,
                            &mut errors
                        ),
                        false
                    );
                    assert!(!errors.is_empty());
                    errors.pop();
                    assert!(errors.is_empty());
                }
            }
        }
    }
}

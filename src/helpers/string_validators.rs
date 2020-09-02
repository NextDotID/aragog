use regex::Regex;

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
/// [`Validate`]: ../../validate/trait.Validate.html
/// [`validations`]: ../../validate/trait.Validate.html#tymethod.validations
#[allow(dead_code)]
pub fn validate_numeric_string(field_path: &str, str: &str, errors: &mut Vec<String>) -> bool {
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
/// [`Validate`]: ../../validate/trait.Validate.html
/// [`validations`]: ../../validate/trait.Validate.html#tymethod.validations
#[allow(dead_code)]
pub fn validate_max_len(field_path: &str, str: &str, max_len: usize, errors: &mut Vec<String>) -> bool {
    if str.len() > max_len {
        errors.push(format!("{} '{}' is too long, max length: {}", field_path, str, max_len));
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
/// [`Validate`]: ../../validate/trait.Validate.html
/// [`validations`]: ../../validate/trait.Validate.html#tymethod.validations
#[allow(dead_code)]
pub fn validate_min_len(field_path: &str, str: &str, min_len: usize, errors: &mut Vec<String>) -> bool {
    if str.len() < min_len {
        errors.push(format!("{} '{}' is too short, min length: {}", field_path, str, min_len));
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
/// [`Validate`]: ../../validate/trait.Validate.html
/// [`validations`]: ../../validate/trait.Validate.html#tymethod.validations
#[allow(dead_code)]
pub fn validate_len(field_path: &str, str: &str, len: usize, errors: &mut Vec<String>) -> bool {
    if str.len() != len {
        errors.push(format!("{} '{}' has wrong length, please specify {} characters", field_path, str, len));
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
/// [`Validate`]: ../../validate/trait.Validate.html
/// [`validations`]: ../../validate/trait.Validate.html#tymethod.validations
#[allow(dead_code)]
pub fn validate_regex(field_path: &str, str :&str, regex :&str, errors: &mut Vec<String>) -> bool {
    let reg = match Regex::new(regex) {
        Ok(value) => { value },
        Err(error) => {
            log::error!("FATAL: Wrong regex: {}", error);
            return false;
        }
    };
    let result = reg.is_match(str);
    if result { return true; }
    errors.push(format!("{} '{}' has incorrect format", field_path, str));
    false
}

/// Validates that `str` has a correct RFC 3339 date format. Usually used as a helper function for implementations of
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
/// [`Validate`]: ../../validate/trait.Validate.html
/// [`validations`]: ../../validate/trait.Validate.html#tymethod.validations
//#[allow(dead_code)]
//pub fn validate_date_format(field_path :&str, str :&str, errors: &mut Vec<String>) -> bool {
//    match DateTime::parse_from_rfc3339(str) {
//        Ok(_value) => { true },
//        Err(_error) => {
//            errors.push(format!("Invalid date format on {}", field_path));
//            false
//        }
//    }
//}

#[cfg(test)]
mod tests {
    const STRING_EMPTY: &str = "";
    use super::*;
    use std::ops::Deref;

    mod numeric_string {
        use super::*;

        #[test]
        fn validates_only_numeric_characters() {
            let mut errors = Vec::new();
            let correct_str = "0123456789";
            let wrong_strs = ["+33122334", "01 02 03", "1.23"];

            assert_eq!(validate_numeric_string(STRING_EMPTY, correct_str, &mut errors), true);
            assert_eq!(errors.is_empty(), true);
            for wrong_str in wrong_strs.iter() {
                assert_eq!(validate_numeric_string(STRING_EMPTY, wrong_str, &mut errors), false);
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
            let wrong_strs = ["hello678911", "foobarfoobar1", "bar-+123lkdzacdeee+", "010203040005"];

            for correct_str in correct_strs.iter() {
                assert_eq!(validate_max_len(STRING_EMPTY, &String::from(correct_str.deref()), max, &mut errors), true);
                assert_eq!(errors.is_empty(), true);
            }
            for wrong_str in wrong_strs.iter() {
                assert_eq!(validate_max_len(STRING_EMPTY, &String::from(wrong_str.deref()), max, &mut errors), false);
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
            let correct_strs = ["hello678911", "foobarfoobar1", "bar-+123lkdzacdeee+", "010203040005"];
            let wrong_strs = ["hello", "foo1", "bar-++", "010203040"];

            for correct_str in correct_strs.iter() {
                assert_eq!(validate_min_len(STRING_EMPTY, &String::from(correct_str.deref()), min, &mut errors), true);
                assert_eq!(errors.is_empty(), true);
            }
            for wrong_str in wrong_strs.iter() {
                assert_eq!(validate_min_len(STRING_EMPTY, &String::from(wrong_str.deref()), min, &mut errors), false);
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
                assert_eq!(validate_len(STRING_EMPTY, &String::from(correct_str.deref()), length, &mut errors), true);
                assert_eq!(errors.is_empty(), true);
            }
            for wrong_str in wrong_strs.iter() {
                assert_eq!(validate_len(STRING_EMPTY, &String::from(wrong_str.deref()), length, &mut errors), false);
                assert_eq!(errors.is_empty(), false);
                errors.pop().unwrap();
                assert_eq!(errors.is_empty(), true);
            }
        }
    }

    mod regex {
        use super::*;

        const REGEX :&str = "^[a-z]{1,10}$";

        #[test]
        fn validate_regex_works() {
            let mut errors = Vec::new();
            let valid_strs = ["abc", "hellothere", "felix"];
            let wrong_strs = ["abc1", "hellotherebro", "felix de_manevi11e", "a bc"];

            for valid_str in valid_strs.iter() {
                assert_eq!(validate_regex(STRING_EMPTY, valid_str.deref(), REGEX, &mut errors), true);
                assert!(errors.is_empty());
            }
            for wrong_str in wrong_strs.iter() {
                assert_eq!(validate_regex(STRING_EMPTY, wrong_str.deref(), REGEX, &mut errors), false);
                assert!(!errors.is_empty());
                errors.pop();
                assert!(errors.is_empty());
            }
        }
    }
}
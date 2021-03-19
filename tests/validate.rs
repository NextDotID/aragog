use aragog::{ServiceError, Validate};
use serde::{Deserialize, Serialize};

pub mod common;

#[derive(Serialize, Deserialize, Clone, Validate)]
#[validate(func("custom_validations"))]
pub struct Dish {
    #[validate(min_length = 5)]
    pub name: String,
    pub description: Option<String>,
    #[validate(length = 10)]
    pub reference: String,
    #[validate(greater_than(0))]
    pub price: u16,
}

impl Dish {
    fn custom_validations(&self, errors: &mut Vec<String>) {
        Self::validate_field_presence("description", &self.description, errors);
        if self.description.is_some() {
            Self::validate_min_len(
                "description",
                self.description.as_ref().unwrap(),
                15,
                errors,
            );
        }
        Self::validate_numeric_string("reference", &self.reference, errors);
    }
}

#[test]
fn can_succeed() {
    let dish = Dish {
        name: "Pizza Regina".to_string(),
        description: Some("Tomate, Jambon, Oeuf, Mozzarella".to_string()),
        reference: "0102030405".to_string(),
        price: 5,
    };
    dish.validate().unwrap();
}

#[should_panic(expected = "ValidationError")]
#[test]
fn can_fail() {
    let dish = Dish {
        name: "Piza".to_string(),
        description: Some("wrong".to_string()),
        reference: "ABC".to_string(),
        price: 0,
    };
    dish.validate().unwrap();
}

#[test]
fn can_fail_and_provide_message() -> Result<(), String> {
    let dish = Dish {
        name: "Piza".to_string(),
        description: Some("wrong".to_string()),
        reference: "ABC".to_string(),
        price: 0,
    };
    match dish.validate() {
        Ok(()) => Err(String::from("Should have failed validations")),
        Err(error) => match error {
            ServiceError::ValidationError(str) => {
                println!("{}", str);
                common::expect_assert(str.contains(r#"name 'Piza' is too short, min length: 5"#))?;
                common::expect_assert(
                    str.contains(r#"description 'wrong' is too short, min length: 15"#),
                )?;
                common::expect_assert(str.contains(r#"reference 'ABC' is not numeric"#))?;
                common::expect_assert(str.contains(
                    r#"reference 'ABC' has wrong length, please specify 10 characters"#,
                ))?;
                common::expect_assert(str.contains(r#"price '0' must be greater than 0"#))?;
                Ok(())
            }
            _ => Err(String::from("Validations failed but wrong error returned")),
        },
    }
}

mod macros {
    use super::*;

    mod complete_operations {
        use super::*;

        // Everything compiling is already a test
        #[derive(Validate)]
        pub struct CompleteValidator {
            // Floats
            #[validate(greater_than(- 10.0), greater_or_equal(- 9.0), lesser_than(10.0), lesser_or_equal(9.0))]
            pub float32: f32,
            #[validate(greater_than(- 10.0), greater_or_equal(- 9.0), lesser_than(10.0), lesser_or_equal(9.0))]
            pub float64: f64,

            // Float vectors
            #[validate_each(greater_than(- 10.0), greater_or_equal(- 9.0), lesser_than(10.0), lesser_or_equal(9.0))]
            pub vec_float32: Vec<f32>,
            #[validate_each(greater_than(- 10.0), greater_or_equal(- 9.0), lesser_than(10.0), lesser_or_equal(9.0))]
            pub vec_float64: Vec<f64>,

            // Ints
            #[validate(greater_than(- 10), greater_or_equal(- 9), lesser_than(10), lesser_or_equal(9))]
            pub int8: i8,
            #[validate(greater_than(- 10), greater_or_equal(- 9), lesser_than(10), lesser_or_equal(9))]
            pub int_size: isize,
            #[validate(greater_than(- 10), greater_or_equal(- 9), lesser_than(10), lesser_or_equal(9))]
            pub int16: i16,
            #[validate(greater_than(- 10), greater_or_equal(- 9), lesser_than(10), lesser_or_equal(9))]
            pub int32: i32,
            #[validate(greater_than(- 10), greater_or_equal(- 9), lesser_than(10), lesser_or_equal(9))]
            pub int64: i64,
            #[validate(greater_than(- 10), greater_or_equal(- 9), lesser_than(10), lesser_or_equal(9))]
            pub int128: i128,

            // Int vectors
            #[validate_each(greater_than(- 10), greater_or_equal(- 9), lesser_than(10), lesser_or_equal(9))]
            pub vec_int8: Vec<i8>,
            #[validate_each(greater_than(- 10), greater_or_equal(- 9), lesser_than(10), lesser_or_equal(9))]
            pub vec_int_size: Vec<isize>,
            #[validate_each(greater_than(- 10), greater_or_equal(- 9), lesser_than(10), lesser_or_equal(9))]
            pub vec_int16: Vec<i16>,
            #[validate_each(greater_than(- 10), greater_or_equal(- 9), lesser_than(10), lesser_or_equal(9))]
            pub vec_int32: Vec<i32>,
            #[validate_each(greater_than(- 10), greater_or_equal(- 9), lesser_than(10), lesser_or_equal(9))]
            pub vec_int64: Vec<i64>,
            #[validate_each(greater_than(- 10), greater_or_equal(- 9), lesser_than(10), lesser_or_equal(9))]
            pub vec_int128: Vec<i128>,

            // UInts
            #[validate(
                greater_than(0),
                greater_or_equal(1),
                lesser_than(10),
                lesser_or_equal(9)
            )]
            pub uint8: u8,
            #[validate(
                greater_than(0),
                greater_or_equal(1),
                lesser_than(10),
                lesser_or_equal(9)
            )]
            pub uint_size: usize,
            #[validate(
                greater_than(0),
                greater_or_equal(1),
                lesser_than(10),
                lesser_or_equal(9)
            )]
            pub uint16: u16,
            #[validate(
                greater_than(0),
                greater_or_equal(1),
                lesser_than(10),
                lesser_or_equal(9)
            )]
            pub uint32: u32,
            #[validate(
                greater_than(0),
                greater_or_equal(1),
                lesser_than(10),
                lesser_or_equal(9)
            )]
            pub uint64: u64,
            #[validate(
                greater_than(0),
                greater_or_equal(1),
                lesser_than(10),
                lesser_or_equal(9)
            )]
            pub uint128: u128,

            // UInt vectors
            #[validate_each(
                greater_than(0),
                greater_or_equal(1),
                lesser_than(10),
                lesser_or_equal(9)
            )]
            pub vec_uint8: Vec<u8>,
            #[validate_each(
                greater_than(0),
                greater_or_equal(1),
                lesser_than(10),
                lesser_or_equal(9)
            )]
            pub vec_uint_size: Vec<usize>,
            #[validate_each(
                greater_than(0),
                greater_or_equal(1),
                lesser_than(10),
                lesser_or_equal(9)
            )]
            pub vec_uint16: Vec<u16>,
            #[validate_each(
                greater_than(0),
                greater_or_equal(1),
                lesser_than(10),
                lesser_or_equal(9)
            )]
            pub vec_uint32: Vec<u32>,
            #[validate_each(
                greater_than(0),
                greater_or_equal(1),
                lesser_than(10),
                lesser_or_equal(9)
            )]
            pub vec_uint64: Vec<u64>,
            #[validate_each(
                greater_than(0),
                greater_or_equal(1),
                lesser_than(10),
                lesser_or_equal(9)
            )]
            pub vec_uint128: Vec<u128>,

            // Strings
            #[validate(min_length = 1, max_length = 10, length = 5, regex("//"))]
            pub string: String,
            #[validate(min_length = 1, max_length = 10, length = 5, regex("//"))]
            pub str: &'static str,

            // String vectors
            #[validate_each(min_length = 1, max_length = 10, length = 5, regex("//"))]
            pub vec_string: Vec<String>,
            #[validate_each(min_length = 1, max_length = 10, length = 5, regex("//"))]
            pub vec_str: Vec<&'static str>,
        }
    }

    mod custom_comparator {
        use super::*;
        use std::fmt::{self, Display, Formatter};

        #[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
        enum CustomOrd {
            A,
            B,
            C,
        }

        impl Display for CustomOrd {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        #[derive(Validate)]
        struct Comparator {
            #[validate(lesser_than(CustomOrd::C), greater_than(CustomOrd::A))]
            pub field: CustomOrd,
        }

        #[test]
        fn comparison_works() {
            assert!(Comparator {
                field: CustomOrd::B
            }
            .is_valid());
            assert!(!Comparator {
                field: CustomOrd::A
            }
            .is_valid());
            assert!(!Comparator {
                field: CustomOrd::C
            }
            .is_valid());
        }
    }

    mod enum_validations {
        use super::*;

        #[derive(Validate)]
        #[validate(func = "validate_cases")]
        pub enum EnumValidator {
            Case1,
            Case2,
        }

        impl EnumValidator {
            fn validate_cases(&self, errors: &mut Vec<String>) {
                match self {
                    EnumValidator::Case1 => errors.push("Case1 is invalid".to_string()),
                    EnumValidator::Case2 => errors.push("Case2 is invalid".to_string()),
                }
            }
        }

        #[test]
        fn enums_validation() {
            assert!(!EnumValidator::Case1.is_valid());
            match EnumValidator::Case1.validate() {
                Ok(()) => panic!("Should fail for Case1"),
                Err(e) => match e {
                    ServiceError::ValidationError(msg) => {
                        assert_eq!(msg, "Case1 is invalid".to_string())
                    }
                    _ => panic!("Wrong error returned for Case1"),
                },
            }
            assert!(!EnumValidator::Case2.is_valid());
            match EnumValidator::Case2.validate() {
                Ok(()) => panic!("Should fail for Case2"),
                Err(e) => match e {
                    ServiceError::ValidationError(msg) => {
                        assert_eq!(msg, "Case2 is invalid".to_string())
                    }
                    _ => panic!("Wrong error returned for Case2"),
                },
            }
        }
    }
}

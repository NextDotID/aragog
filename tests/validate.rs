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

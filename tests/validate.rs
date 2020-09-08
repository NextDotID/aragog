use serde::{Deserialize, Serialize};

use aragog::{ServiceError, Validate};
use aragog::helpers::string_validators;

pub mod common;

#[derive(Serialize, Deserialize, Clone)]
pub struct Dish {
    pub name: String,
    pub description: Option<String>,
    pub reference: String,
    pub price: u16,
}

impl Validate for Dish {
    fn validations(&self, errors: &mut Vec<String>) {
        string_validators::validate_min_len("name", &self.name, 5, errors);

        Self::validate_field_presence("description", &self.description, errors);
        if self.description.is_some() {
            string_validators::validate_min_len("description", self.description.as_ref().unwrap(), 15, errors);
        }
        string_validators::validate_numeric_string("reference", &self.reference, errors);
        string_validators::validate_len("reference", &self.reference, 10, errors);
        if self.price == 0 {
            errors.push("Price can't be zero".to_string())
        }
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
                common::expect_assert(str.contains(r#"name 'Piza' is too short, min length: 5"#))?;
                common::expect_assert(str.contains(r#"description 'wrong' is too short, min length: 15"#))?;
                common::expect_assert(str.contains(r#"reference 'ABC' is not numeric"#))?;
                common::expect_assert(str.contains(r#"reference 'ABC' has wrong length, please specify 10 characters"#))?;
                common::expect_assert(str.contains(r#"Price can't be zero"#))?;
                Ok(())
            }
            _ => Err(String::from("Validations failed but wrong error returned"))
        }
    }
}
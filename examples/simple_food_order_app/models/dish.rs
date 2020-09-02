use chrono::Utc;
use serde::{Deserialize, Serialize};

use aragog::{AragogServiceError, New, Record, Validate, Update};
use aragog::helpers::string_validators;

#[derive(Serialize, Deserialize, Clone)]
pub struct Dish {
    pub name: String,
    pub description: String,
    pub price: u16,
    created_at: u64,
    updated_at: u64,
}

pub struct DishDTO {
    pub name: String,
    pub description: String,
    pub price: u16,
}

impl Record for Dish {
    fn collection_name() -> String {
        String::from("Dishes")
    }
}

impl Validate for Dish {
    fn validations(&self, errors: &mut Vec<String>) {
        string_validators::validate_min_len("name", &self.name, 5, errors);
        string_validators::validate_min_len("name", &self.description, 15, errors);
        if self.price == 0 {
            errors.push("price should be above zero".to_string())
        }
    }
}

impl New<DishDTO> for Dish {
    fn new(form: DishDTO) -> Result<Self, AragogServiceError> {
        Ok(Self {
            name: form.name,
            description: form.description,
            price: form.price,
            created_at: Utc::now().timestamp() as u64,
            updated_at: Utc::now().timestamp() as u64,
        })
    }
}

impl Update<DishDTO> for Dish {
    fn update(&mut self, form: &DishDTO) -> Result<(), AragogServiceError> {
        self.name = form.name.clone();
        self.description = form.description.clone();
        self.price = form.price;
        self.updated_at = Utc::now().timestamp() as u64;
        Ok(())
    }
}
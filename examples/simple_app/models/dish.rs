use chrono::Utc;
use serde::{Deserialize, Serialize};

use aragog::{ServiceError, New, Record, Validate, Update};
use aragog::helpers::string_validators;

#[derive(Serialize, Deserialize, Clone)]
pub struct Dish {
    pub name: String,
    pub description: String,
    pub price: u16,
    pub is_alcohol: bool,
    created_at: u64,
    updated_at: u64,
}

pub struct DishDTO {
    pub name: String,
    pub description: String,
    pub price: u16,
    pub is_alcohol: bool,
}

impl Record for Dish {
    fn collection_name() -> &'static str { "Dishes" }
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
    fn new(form: DishDTO) -> Result<Self, ServiceError> {
        Ok(Self {
            name: form.name,
            description: form.description,
            price: form.price,
            is_alcohol: form.is_alcohol,
            created_at: Utc::now().timestamp() as u64,
            updated_at: Utc::now().timestamp() as u64,
        })
    }
}

impl Update<DishDTO> for Dish {
    fn update(&mut self, form: &DishDTO) -> Result<(), ServiceError> {
        self.name = form.name.clone();
        self.description = form.description.clone();
        self.price = form.price;
        self.is_alcohol = form.is_alcohol;
        self.updated_at = Utc::now().timestamp() as u64;
        Ok(())
    }
}
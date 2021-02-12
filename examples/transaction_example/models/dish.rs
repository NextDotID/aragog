use chrono::Utc;
use serde::{Deserialize, Serialize};

use aragog::{New, Record, ServiceError, Update, Validate};

#[derive(Serialize, Deserialize, Clone, Record, Validate)]
#[hook(before_all(func = "validate"))]
pub struct Dish {
    #[validate(min_length = 5)]
    pub name: String,
    #[validate(min_length = 15)]
    pub description: String,
    #[validate(greater_than(0))]
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

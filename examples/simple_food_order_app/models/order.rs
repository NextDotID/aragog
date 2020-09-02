use serde::{Deserialize, Serialize};
use crate::models::dish::Dish;
use aragog::{Record, Validate};
use chrono::Utc;

#[derive(Serialize, Deserialize, Clone)]
pub struct Order {
    pub dishes: Vec<Dish>,
    pub total_price: u16,
    created_at: u64,
    updated_at: u64
}

impl Order {
    pub fn new() -> Self {
        Self {
            dishes: vec![],
            total_price: 0,
            created_at: Utc::now().timestamp() as u64,
            updated_at: Utc::now().timestamp() as u64
        }
    }

    pub fn add(&mut self, dish: &Dish) {
        self.total_price += dish.price;
        self.dishes.push(dish.clone());
    }
}

impl Record for Order {
    fn collection_name() -> String {
        String::from("Orders")
    }
}

impl Validate for Order {
    fn validations(&self, errors: &mut Vec<String>) {
        if self.dishes.is_empty() {
            errors.push(String::from("Should have at least one dish"));
        }
        let mut computed_price = 0;
        for dish in self.dishes.iter() {
            computed_price += dish.price;
        }
        if computed_price != self.total_price {
            errors.push(String::from("Wrong total price"))
        }
    }
}
#![cfg(not(feature = "minimal_traits"))]

use aragog::{ServiceError, Update};
use serde::{Deserialize, Serialize};

pub mod common;

#[derive(Serialize, Deserialize, Clone)]
pub struct Dish {
    pub name: String,
    pub description: String,
    pub price: u16,
}

pub struct DishForm {
    form_name: String,
    form_description: String,
    price: u16,
}

impl Update<DishForm> for Dish {
    fn update(&mut self, form: &DishForm) -> Result<(), ServiceError> {
        if form.price == 0 {
            return Err(ServiceError::ValidationError(String::from("Wrong price")));
        }
        self.name = form.form_name.clone();
        self.description = form.form_description.clone();
        self.price = form.price;
        Ok(())
    }
}

#[test]
fn can_succeed() {
    let mut dish = Dish {
        name: "Pizza Regina".to_string(),
        description: "Tomate, Jambon, Oeuf, Mozzarella".to_string(),
        price: 5,
    };
    let form = DishForm {
        form_name: String::from("Pizza Savoyarde"),
        form_description: String::from("Base crème, oignons, champignons, reublochon, poitrine"),
        price: 13,
    };
    dish.update(&form).unwrap();
    assert_eq!(&dish.name, &form.form_name);
    assert_eq!(&dish.description, &form.form_description);
    assert_eq!(dish.price, form.price);
}

#[should_panic(expected = "ValidationError")]
#[test]
fn can_fail() {
    let mut dish = Dish {
        name: "Pizza Regina".to_string(),
        description: "Tomate, Jambon, Oeuf, Mozzarella".to_string(),
        price: 5,
    };
    let form = DishForm {
        form_name: String::from("Pizza Savoyarde"),
        form_description: String::from("Base crème, oignons, champignons, reublochon, poitrine"),
        price: 0,
    };
    dish.update(&form).unwrap();
}

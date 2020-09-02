use serde::{Deserialize, Serialize};
use aragorn::{New, AragornServiceError};

pub mod common;

#[derive(Serialize, Deserialize, Clone)]
pub struct Dish {
    pub name: String,
    pub description: String,
    price: u16,
    created_at: u64
}

#[derive(Clone)]
pub struct DishForm {
    form_name: String,
    form_description: String,
    price: u16,
}

impl New<DishForm> for Dish {
    fn new(form: DishForm) -> Result<Self, AragornServiceError> {
        let res = Dish {
            name: form.form_name,
            description: form.form_description,
            price: form.price,
            created_at: 1000
        };
        if res.price == 0 {
            return Err(AragornServiceError::ValidationError(String::from("Wrong price")));
        }
        Ok(res)
    }
}

#[test]
fn can_succeed() {
    let form = DishForm {
        form_name: String::from("Pizza Savoyarde"),
        form_description: String::from("Base crème, oignons, champignons, reublochon, poitrine"),
        price: 13
    };
    let dish = Dish::new(form.clone()).unwrap();
    assert_eq!(&dish.name, &form.form_name);
    assert_eq!(&dish.description, &form.form_description);
    assert_eq!(dish.price, form.price);
}

#[should_panic(expected = "ValidationError")]
#[test]
fn can_fail() {
    let form = DishForm {
        form_name: String::from("Pizza Regina"),
        form_description: String::from("Base tomate, jambon, Mozzarella, Jambon"),
        price: 0
    };
    Dish::new(form).unwrap();
}
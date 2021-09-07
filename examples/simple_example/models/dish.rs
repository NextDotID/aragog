use chrono::Utc;
use serde::{Deserialize, Serialize};

use aragog::{DatabaseAccess, Error, New, Record, Update, Validate};

/// this is a Dish
#[derive(Serialize, Deserialize, Clone, Record, Validate)]
#[before_create(func = "hook_before_create", is_async = true, db_access = true)]
#[before_save(func = "hook_before_save")]
pub struct Dish {
    #[validate(min_length = 5, max_length(20))]
    pub name: String,
    #[validate(min_length = 15)]
    pub description: String,
    #[validate(greater_than(0), lesser_or_equal(35))]
    pub price: u16,
    pub is_alcohol: bool,
    #[validate(greater_than(0))]
    created_at: u64,
    #[validate(greater_than(0))]
    updated_at: u64,
}

impl Dish {
    async fn hook_before_create<D>(&mut self, _db_accessor: &D) -> Result<(), Error>
    where
        D: DatabaseAccess + ?Sized,
    {
        self.created_at = Utc::now().timestamp() as u64;
        self.updated_at = Utc::now().timestamp() as u64;
        self.validate()?;
        Ok(())
    }

    fn hook_before_save(&mut self) -> Result<(), Error> {
        self.updated_at = Utc::now().timestamp() as u64;
        self.validate()?;
        Ok(())
    }
}

pub struct DishDTO {
    pub name: String,
    pub description: String,
    pub price: u16,
    pub is_alcohol: bool,
}

impl New<DishDTO> for Dish {
    fn new(form: DishDTO) -> Result<Self, Error> {
        Ok(Self {
            name: form.name,
            description: form.description,
            price: form.price,
            is_alcohol: form.is_alcohol,
            created_at: 0,
            updated_at: 0,
        })
    }
}

impl Update<DishDTO> for Dish {
    fn update(&mut self, form: &DishDTO) -> Result<(), Error> {
        self.name = form.name.clone();
        self.description = form.description.clone();
        self.price = form.price;
        self.is_alcohol = form.is_alcohol;
        Ok(())
    }
}

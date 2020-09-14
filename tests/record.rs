use serde::{Deserialize, Serialize};

use aragog::{DatabaseRecord, Record, Validate};
use aragog::DatabaseConnectionPool;
use common::with_db;

pub mod common;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Dish {
    pub name: String,
    pub description: String,
    pub price: u16,
}

impl Record for Dish {
    fn collection_name() -> &'static str { "Dishes" }
}

impl Validate for Dish {
    fn validations(&self, _errors: &mut Vec<String>) {}
}

fn init_dish() -> Dish {
    Dish {
        name: "Quiche".to_string(),
        description: "Part de quiche aux oeufs, lardons et fromage".to_string(),
        price: 7,
    }
}

mod write {
    use super::*;

    #[test]
    fn can_be_recorded_and_retrieved() -> Result<(), String> {
        with_db(|pool| {
            let dish = init_dish();
            let dish_record = tokio_test::block_on(DatabaseRecord::create(dish, pool)).unwrap();
            let found_record = tokio_test::block_on(Dish::find(&dish_record.key, pool)).unwrap();
            common::expect_assert_eq(dish_record.record, found_record.record)?;
            Ok(())
        })
    }

    #[should_panic(expected = "Conflict")]
    #[test]
    fn can_fail() {
        with_db(|pool| {
            let dish = init_dish();
            tokio_test::block_on(DatabaseRecord::create(dish.clone(), pool)).unwrap();
            tokio_test::block_on(DatabaseRecord::create(dish, pool)).unwrap();
            Ok(())
        }).unwrap();
    }
}

mod read {
    use super::*;
    use aragog::ServiceError;
    use aragog::query::{Query, QueryItem};

    fn create_dishes(pool: &DatabaseConnectionPool) -> DatabaseRecord<Dish> {
        tokio_test::block_on(DatabaseRecord::create(Dish {
            name: "Pizza".to_string(),
            description: "Tomato and Mozarella".to_string(),
            price: 10
        }, pool)).unwrap();
        tokio_test::block_on(DatabaseRecord::create(Dish {
            name: "Pasta".to_string(),
            description: "Ham and cheese".to_string(),
            price: 6
        }, pool)).unwrap();
        tokio_test::block_on(DatabaseRecord::create(Dish {
            name: "Steak".to_string(),
            description: "Served with fries".to_string(),
            price: 10
        }, pool)).unwrap();
        tokio_test::block_on(DatabaseRecord::create(init_dish(), pool)).unwrap()
    }

    #[test]
    fn find() -> Result<(), String> {
        with_db(|pool| {
            let dish_record = create_dishes(&pool);

            let found_record = tokio_test::block_on(Dish::find(&dish_record.key, pool)).unwrap();
            common::expect_assert_eq(dish_record.record, found_record.record)?;
            Ok(())
        })
    }

    #[should_panic(expected = "NotFound")]
    #[test]
    fn find_can_fail() {
        with_db(|pool| {
            create_dishes(&pool);
            tokio_test::block_on(Dish::find("wrong_key", pool)).unwrap();
            Ok(())
        }).unwrap();
    }

    #[test]
    fn find_can_fail_with_correct_error() -> Result<(), String>  {
        with_db(|pool| {
            create_dishes(&pool);
            let res = tokio_test::block_on(Dish::find("wrong_key", pool));
            if let Err(error) = res {
                if let ServiceError::NotFound(message) = error {
                    assert_eq!(message, "Dishes document not found".to_string());
                    Ok(())
                }
                else {
                    Err(format!("The find should return a NotFound"))
                }
            }
            else {
                Err(format!("The find should return an error"))
            }
        })
    }

    #[test]
    fn find_where() -> Result<(), String> {
        with_db(|pool| {
            let dish_record = create_dishes(&pool);
            let query = Query::new(QueryItem::field("name").equals_str("Quiche"))
                .and(QueryItem::field("price").equals(7));

            let found_record = tokio_test::block_on(Dish::find_where(query, pool)).unwrap();
            common::expect_assert_eq(dish_record.record, found_record.record)?;
            Ok(())
        })
    }

    #[should_panic(expected = "NotFound")]
    #[test]
    fn find_where_can_fail() {
        with_db(|pool| {
            let query = Query::new(QueryItem::field("name").equals_str("Quiche"));

            tokio_test::block_on(Dish::find_where(query, pool)).unwrap();
            Ok(())
        }).unwrap();
    }


    #[should_panic(expected = "NotFound")]
    #[test]
    fn find_where_can_fail_on_multiple_found() {
        with_db(|pool| {
            create_dishes(&pool);
            let query = Query::new(QueryItem::field("price").equals(10));

            tokio_test::block_on(Dish::find_where(query, pool)).unwrap();
            Ok(())
        }).unwrap();
    }

    #[test]
    fn get_where() -> Result<(), String> {
        with_db(|pool| {
            let dish_record = create_dishes(&pool);
            let query = Query::new(QueryItem::field("name").equals_str("Quiche"))
                .and(QueryItem::field("price").equals(7));

            let found_records = tokio_test::block_on(Dish::get_where(query, pool)).unwrap();
            common::expect_assert_eq(found_records.len(), 1)?;
            common::expect_assert_eq(dish_record.record, found_records[0].record.clone())?;
            Ok(())
        })
    }

    #[test]
    fn get_where_can_return_empty_vec() {
        with_db(|pool| {
            let query = Query::new(QueryItem::field("name").equals_str("Quiche"));
            let found_records = tokio_test::block_on(Dish::get_where(query, pool)).unwrap();
            common::expect_assert_eq(found_records.len(), 0)?;
            Ok(())
        }).unwrap();
    }


    #[test]
    fn get_where_on_multiple_found() -> Result<(), String> {
        with_db(|pool| {
            create_dishes(&pool);
            let query = Query::new(QueryItem::field("price").equals(10));

            let found_records = tokio_test::block_on(Dish::get_where(query, pool)).unwrap();
            common::expect_assert_eq(found_records.len(), 2)?;
            Ok(())
        })
    }

    #[test]
    fn exists_where() -> Result<(), String> {
        with_db(|pool| {
            create_dishes(&pool);
            let query = Query::new(QueryItem::field("name").equals_str("Quiche"))
                .and(QueryItem::field("price").equals(7));
            let res = tokio_test::block_on(Dish::exists_where(query, pool));
            common::expect_assert_eq(res, true)?;
            Ok(())
        })
    }
}
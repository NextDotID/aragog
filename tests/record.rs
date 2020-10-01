#[macro_use]
extern crate aragog;

use serde::{Deserialize, Serialize};

use aragog::{DatabaseRecord, Record, Validate};
use aragog::DatabaseConnectionPool;
use common::with_db;

pub mod common;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Record)]
pub struct Dish {
    pub name: String,
    pub description: String,
    pub price: u16,
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

#[test]
fn macro_works() {
    assert_eq!(Dish::collection_name(), "Dish");
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
    use aragog::query::{Comparison, Filter};
    use aragog::ServiceError;

    use super::*;

    fn create_dishes(pool: &DatabaseConnectionPool) -> DatabaseRecord<Dish> {
        tokio_test::block_on(DatabaseRecord::create(Dish {
            name: "Pizza".to_string(),
            description: "Tomato and Mozarella".to_string(),
            price: 10,
        }, pool)).unwrap();
        tokio_test::block_on(DatabaseRecord::create(Dish {
            name: "Pasta".to_string(),
            description: "Ham and cheese".to_string(),
            price: 6,
        }, pool)).unwrap();
        tokio_test::block_on(DatabaseRecord::create(Dish {
            name: "Steak".to_string(),
            description: "Served with fries".to_string(),
            price: 10,
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
    fn find_can_fail_with_correct_error() -> Result<(), String> {
        with_db(|pool| {
            create_dishes(&pool);
            let res = tokio_test::block_on(Dish::find("wrong_key", pool));
            if let Err(error) = res {
                if let ServiceError::NotFound(message) = error {
                    assert_eq!(message, "Dish document not found".to_string());
                    Ok(())
                } else {
                    Err(format!("The find should return a NotFound"))
                }
            } else {
                Err(format!("The find should return an error"))
            }
        })
    }

    #[test]
    fn query_uniq() -> Result<(), String> {
        with_db(|pool| {
            let dish_record = create_dishes(&pool);
            let query = Dish::query().filter(Filter::new(Comparison::field("name").equals_str("Quiche"))
                .and(Comparison::field("price").equals(7)));

            let found_record = tokio_test::block_on(Dish::get(query, pool)).unwrap().uniq().unwrap();
            common::expect_assert_eq(dish_record.record, found_record.record)?;
            Ok(())
        })
    }

    #[should_panic(expected = "NotFound")]
    #[test]
    fn query_uniq_can_fail() {
        with_db(|pool| {
            let query = Dish::query().filter(Filter::new(Comparison::field("name").equals_str("Quiche")));

            tokio_test::block_on(Dish::get(query, pool)).unwrap().uniq().unwrap();
            Ok(())
        }).unwrap();
    }

    #[should_panic(expected = "NotFound")]
    #[test]
    fn query_uniq_can_fail_on_multiple_found() {
        with_db(|pool| {
            create_dishes(&pool);
            let query = Dish::query().filter(Filter::new(Comparison::field("price").equals(10)));

            tokio_test::block_on(Dish::get(query, pool)).unwrap().uniq().unwrap();
            Ok(())
        }).unwrap();
    }

    #[test]
    fn query() -> Result<(), String> {
        with_db(|pool| {
            let dish_record = create_dishes(&pool);
            let query = Dish::query().filter(Filter::new(Comparison::field("name").equals_str("Quiche"))
                .and(Comparison::field("price").equals(7)));

            let found_records = tokio_test::block_on(Dish::get(query, pool)).unwrap().documents;
            common::expect_assert_eq(found_records.len(), 1)?;
            common::expect_assert_eq(dish_record.record, found_records[0].record.clone())?;
            Ok(())
        })
    }

    #[test]
    fn query_can_return_empty_vec() {
        with_db(|pool| {
            let query = Dish::query().filter(Filter::new(Comparison::field("name").equals_str("Quiche")));
            let found_records = tokio_test::block_on(Dish::get(query, pool)).unwrap();
            common::expect_assert_eq(found_records.len(), 0)?;
            Ok(())
        }).unwrap();
    }

    #[test]
    fn query_on_multiple_found() -> Result<(), String> {
        with_db(|pool| {
            create_dishes(&pool);
            // Can return multiple
            let query = Dish::query().filter(Filter::new(Comparison::field("price").equals(10)));
            let found_records = tokio_test::block_on(Dish::get(query, pool)).unwrap();
            common::expect_assert_eq(found_records.len(), 2)?;

            // Limit features
            let query = Dish::query()
                .filter(Filter::new(Comparison::field("price").equals(10)))
                .limit(1, None);
            let found_records = tokio_test::block_on(Dish::get(query, pool)).unwrap();
            common::expect_assert_eq(found_records.len(), 1)?;

            let query = Dish::query();
            let found_records = tokio_test::block_on(Dish::get(query, pool)).unwrap();
            common::expect_assert_eq(found_records.len(), 4)?;

            let query = Dish::query().limit(2, Some(3));
            let found_records = tokio_test::block_on(Dish::get(query, pool)).unwrap();
            common::expect_assert_eq(found_records.len(), 1)?;

            // Sorting
            let query = Dish::query().sort("name", None);
            let found_records = tokio_test::block_on(Dish::get(query, pool)).unwrap().documents;
            for (i, value) in ["Pasta", "Pizza", "Quiche", "Steak"].iter().enumerate() {
                common::expect_assert_eq(*value, &found_records[i].record.name)?;
            }
            let query = Dish::query().sort("price", None).sort("name", None);
            let found_records = tokio_test::block_on(Dish::get(query, pool)).unwrap().documents;
            for (i, value) in ["Pasta", "Quiche", "Pizza", "Steak"].iter().enumerate() {
                common::expect_assert_eq(*value, &found_records[i].record.name)?;
            }
            Ok(())
        })
    }

    #[test]
    fn exists() -> Result<(), String> {
        with_db(|pool| {
            create_dishes(&pool);
            let query = Dish::query().filter(Filter::new(Comparison::field("name").equals_str("Quiche"))
                .and(Comparison::field("price").equals(7)));

            let res = tokio_test::block_on(Dish::exists(query, pool));
            common::expect_assert_eq(res, true)?;
            Ok(())
        })
    }

    mod graph_querying {
        use aragog::query::Query;

        use super::*;

        mod aql {
            use super::*;

            #[test]
            fn from_record() -> Result<(), String> {
                with_db(|pool| {
                    let dish = create_dishes(&pool);
                    let query = dish.outbound_query(2, 5, "edges");
                    common::expect_assert_eq(query.to_aql(), format!("\
                        FOR a in 2..5 OUTBOUND \'{}\' edges \
                        return a\
                    ", &dish.id))?;
                    Ok(())
                })
            }

            #[test]
            fn explicit() -> Result<(), String> {
                with_db(|pool| {
                    let dish = create_dishes(&pool);
                    let query = Query::outbound(2, 5, "edges", &dish.id);
                    common::expect_assert_eq(query.to_aql(), format!("\
                        FOR a in 2..5 OUTBOUND \'{}\' edges \
                        return a\
                    ", &dish.id))?;
                    Ok(())
                })
            }

            #[test]
            fn complex_query() -> Result<(), String> {
                with_db(|pool| {
                    let dish = create_dishes(&pool);
                    let query = dish.outbound_query(2, 5, "edges")
                        .filter(compare!(field "price").greater_than(10).into())
                        .sort("_id", None);
                    common::expect_assert_eq(query.to_aql(), format!("\
                        FOR a in 2..5 OUTBOUND \'{}\' edges \
                            FILTER a.price > 10 \
                            SORT a._id ASC \
                            return a\
                    ", &dish.id))?;
                    Ok(())
                })
            }
        }
    }
}
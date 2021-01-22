#[macro_use]
extern crate aragog;

use serde::{Deserialize, Serialize};

use aragog::DatabaseConnectionPool;
use aragog::{DatabaseRecord, Record, Validate};

pub mod common;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Record, Validate)]
pub struct Dish {
    pub name: String,
    pub description: String,
    pub price: u16,
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

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn can_be_recorded_and_retrieved() -> Result<(), String> {
        let pool = common::setup_db().await;
        let dish = init_dish();
        let dish_record = DatabaseRecord::create(dish, &pool).await.unwrap();
        let found_record = Dish::find(&dish_record.key, &pool).await.unwrap();
        common::expect_assert_eq(dish_record.record, found_record.record)?;
        Ok(())
    }

    #[should_panic(expected = "Conflict")]
    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn can_fail() {
        let pool = common::setup_db().await;
        let dish = init_dish();
        DatabaseRecord::create(dish.clone(), &pool).await.unwrap();
        DatabaseRecord::create(dish, &pool).await.unwrap();
    }
}

mod fmt {
    use super::*;

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn database_record_can_be_displayed() {
        let pool = common::setup_db().await;
        let db_record = DatabaseRecord::create(
            Dish {
                name: "Pizza".to_string(),
                description: "Tomato and Mozarella".to_string(),
                price: 10,
            },
            &pool,
        )
        .await
        .unwrap();
        assert_eq!(
            format!("{}", db_record),
            format!("Dish {} Database Record", db_record.key)
        );
    }
}

mod read {
    use aragog::query::{Comparison, Filter};
    use aragog::ServiceError;

    use super::*;

    #[maybe_async::maybe_async]
    async fn create_dishes(pool: &DatabaseConnectionPool) -> DatabaseRecord<Dish> {
        DatabaseRecord::create(
            Dish {
                name: "Pizza".to_string(),
                description: "Tomato and Mozarella".to_string(),
                price: 10,
            },
            pool,
        )
        .await
        .unwrap();
        DatabaseRecord::create(
            Dish {
                name: "Pasta".to_string(),
                description: "Ham and cheese".to_string(),
                price: 6,
            },
            pool,
        )
        .await
        .unwrap();
        DatabaseRecord::create(
            Dish {
                name: "Steak".to_string(),
                description: "Served with fries".to_string(),
                price: 10,
            },
            pool,
        )
        .await
        .unwrap();
        DatabaseRecord::create(init_dish(), pool).await.unwrap()
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn find() -> Result<(), String> {
        let pool = common::setup_db().await;
        let dish_record = create_dishes(&pool).await;

        let found_record = Dish::find(&dish_record.key, &pool).await.unwrap();
        common::expect_assert_eq(dish_record.record, found_record.record)?;
        Ok(())
    }

    #[should_panic(expected = "NotFound")]
    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn find_can_fail() {
        let pool = common::setup_db().await;
        create_dishes(&pool).await;
        Dish::find("wrong_key", &pool).await.unwrap();
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn find_can_fail_with_correct_error() -> Result<(), String> {
        let pool = common::setup_db().await;
        create_dishes(&pool).await;
        let res = Dish::find("wrong_key", &pool).await;
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
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn query_uniq() -> Result<(), String> {
        let pool = common::setup_db().await;
        let dish_record = create_dishes(&pool).await;
        let query = Dish::query().filter(
            Filter::new(Comparison::field("name").equals_str("Quiche"))
                .and(Comparison::field("price").equals(7)),
        );

        let found_record = Dish::get(query, &pool).await.unwrap().uniq().unwrap();
        common::expect_assert_eq(dish_record.record, found_record.record)?;
        Ok(())
    }

    #[should_panic(expected = "NotFound")]
    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn query_uniq_can_fail() {
        let pool = common::setup_db().await;
        let query =
            Dish::query().filter(Filter::new(Comparison::field("name").equals_str("Quiche")));

        Dish::get(query, &pool).await.unwrap().uniq().unwrap();
    }

    #[should_panic(expected = "NotFound")]
    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn query_uniq_can_fail_on_multiple_found() {
        let pool = common::setup_db().await;
        create_dishes(&pool).await;
        let query = Dish::query().filter(Filter::new(Comparison::field("price").equals(10)));

        Dish::get(query, &pool).await.unwrap().uniq().unwrap();
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn query() -> Result<(), String> {
        let pool = common::setup_db().await;
        let dish_record = create_dishes(&pool).await;
        let query = Dish::query().filter(
            Filter::new(Comparison::field("name").equals_str("Quiche"))
                .and(Comparison::field("price").equals(7)),
        );

        let found_records = Dish::get(query, &pool).await.unwrap().documents;
        common::expect_assert_eq(found_records.len(), 1)?;
        common::expect_assert_eq(dish_record.record, found_records[0].record.clone())?;
        Ok(())
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn query_can_return_empty_vec() {
        let pool = common::setup_db().await;
        let query =
            Dish::query().filter(Filter::new(Comparison::field("name").equals_str("Quiche")));
        let found_records = Dish::get(query, &pool).await.unwrap();
        common::expect_assert_eq(found_records.len(), 0).unwrap();
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn query_on_multiple_found() -> Result<(), String> {
        let pool = common::setup_db().await;
        create_dishes(&pool).await;
        // Can return multiple
        let query = Dish::query().filter(Filter::new(Comparison::field("price").equals(10)));
        let found_records = Dish::get(query, &pool).await.unwrap();
        common::expect_assert_eq(found_records.len(), 2)?;

        // Limit features
        let query = Dish::query()
            .filter(Filter::new(Comparison::field("price").equals(10)))
            .limit(1, None);
        let found_records = Dish::get(query, &pool).await.unwrap();
        common::expect_assert_eq(found_records.len(), 1)?;

        let query = Dish::query();
        let found_records = Dish::get(query, &pool).await.unwrap();
        common::expect_assert_eq(found_records.len(), 4)?;

        let query = Dish::query().limit(2, Some(3));
        let found_records = Dish::get(query, &pool).await.unwrap();
        common::expect_assert_eq(found_records.len(), 1)?;

        // Sorting
        let query = Dish::query().sort("name", None);
        let found_records = Dish::get(query, &pool).await.unwrap().documents;
        for (i, value) in ["Pasta", "Pizza", "Quiche", "Steak"].iter().enumerate() {
            common::expect_assert_eq(*value, &found_records[i].record.name)?;
        }
        let query = Dish::query().sort("price", None).sort("name", None);
        let found_records = Dish::get(query, &pool).await.unwrap().documents;
        for (i, value) in ["Pasta", "Quiche", "Pizza", "Steak"].iter().enumerate() {
            common::expect_assert_eq(*value, &found_records[i].record.name)?;
        }
        Ok(())
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn exists() -> Result<(), String> {
        let pool = common::setup_db().await;
        create_dishes(&pool).await;
        let query = Dish::query().filter(
            Filter::new(Comparison::field("name").equals_str("Quiche"))
                .and(Comparison::field("price").equals(7)),
        );

        let res = Dish::exists(query, &pool).await;
        common::expect_assert_eq(res, true)?;
        Ok(())
    }

    mod graph_querying {
        use aragog::query::Query;

        use super::*;

        mod aql {
            use super::*;

            #[maybe_async::test(
                feature = "blocking",
                async(all(not(feature = "blocking")), tokio::test)
            )]
            async fn from_record() -> Result<(), String> {
                let pool = common::setup_db().await;
                let dish = create_dishes(&pool).await;
                let query = dish.outbound_query(2, 5, "edges");
                common::expect_assert_eq(
                    query.to_aql(),
                    format!(
                        "\
                        FOR a in 2..5 OUTBOUND \'{}\' edges \
                        return a\
                                ",
                        &dish.id
                    ),
                )?;
                Ok(())
            }

            #[maybe_async::test(
                feature = "blocking",
                async(all(not(feature = "blocking")), tokio::test)
            )]
            async fn explicit() -> Result<(), String> {
                let pool = common::setup_db().await;
                let dish = create_dishes(&pool).await;
                let query = Query::outbound(2, 5, "edges", &dish.id);
                common::expect_assert_eq(
                    query.to_aql(),
                    format!(
                        "\
                        FOR a in 2..5 OUTBOUND \'{}\' edges \
                        return a\
                                ",
                        &dish.id
                    ),
                )?;
                Ok(())
            }

            #[maybe_async::test(
                feature = "blocking",
                async(all(not(feature = "blocking")), tokio::test)
            )]
            async fn complex_query() -> Result<(), String> {
                let pool = common::setup_db().await;
                let dish = create_dishes(&pool).await;
                let query = dish
                    .outbound_query(2, 5, "edges")
                    .filter(compare!(field "price").greater_than(10).into())
                    .sort("_id", None);
                common::expect_assert_eq(
                    query.to_aql(),
                    format!(
                        "\
                        FOR a in 2..5 OUTBOUND \'{}\' edges \
                            FILTER a.price > 10 \
                            SORT a._id ASC \
                            return a\
                                ",
                        &dish.id
                    ),
                )?;
                Ok(())
            }
        }
    }
}

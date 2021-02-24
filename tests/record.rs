#[macro_use]
extern crate aragog;

use serde::{Deserialize, Serialize};

use aragog::{DatabaseAccess, DatabaseConnectionPool, ServiceError};
use aragog::{DatabaseRecord, Record, Validate};

pub mod common;

#[derive(Serialize, Deserialize, Clone, Record, Debug)]
pub struct Menu {
    pub dish_count: u16,
    pub last_dish_updated: Option<Dish>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Record, Validate)]
#[hook(before_write(func("validate")))]
#[hook(before_create(func("increment_menu"), is_async = true, db_access = true))]
#[hook(before_save(func("last_dish_update"), is_async = true, db_access = true))]
#[hook(after_all(func("after_all")))]
pub struct Dish {
    #[validate(min_length = 3)]
    pub name: String,
    pub description: String,
    #[validate(greater_than(0))]
    pub price: u16,
    pub menu_id: String,
}

impl Dish {
    fn after_all(&self) -> Result<(), ServiceError> {
        println!("Dish written in db");
        Ok(())
    }

    #[maybe_async::maybe_async]
    async fn increment_menu<D>(&self, db_access: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess,
    {
        let mut menu: DatabaseRecord<Menu> = Menu::find(&self.menu_id, db_access).await?;
        menu.dish_count += 1;
        menu.last_dish_updated = Some(self.clone());
        menu.save(db_access).await?;
        Ok(())
    }

    #[maybe_async::maybe_async]
    async fn last_dish_update<D>(&self, db_access: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess,
    {
        let mut menu: DatabaseRecord<Menu> = Menu::find(&self.menu_id, db_access).await?;
        menu.last_dish_updated = Some(self.clone());
        menu.save(db_access).await?;
        Ok(())
    }
}

fn init_dish(menu_id: &String) -> Dish {
    Dish {
        name: "Quiche".to_string(),
        description: "Part de quiche aux oeufs, lardons et fromage".to_string(),
        price: 7,
        menu_id: menu_id.clone(),
    }
}

#[maybe_async::maybe_async]
async fn init_menu(db_access: &DatabaseConnectionPool) -> DatabaseRecord<Menu> {
    DatabaseRecord::create(
        Menu {
            dish_count: 0,
            last_dish_updated: None,
        },
        db_access,
    )
    .await
    .unwrap()
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
        let menu = init_menu(&pool).await;
        let dish = init_dish(menu.key());
        let dish_record = DatabaseRecord::create(dish, &pool).await.unwrap();
        let found_record = Dish::find(dish_record.key(), &pool).await.unwrap();
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
        let menu = init_menu(&pool).await;
        let dish = init_dish(menu.key());
        DatabaseRecord::create(dish.clone(), &pool).await.unwrap();
        DatabaseRecord::create(dish, &pool).await.unwrap();
    }

    mod hooks {
        use super::*;

        #[maybe_async::test(
            feature = "blocking",
            async(all(not(feature = "blocking")), tokio::test)
        )]
        async fn before_create_and_save_hook() -> Result<(), String> {
            let pool = common::setup_db().await;
            let menu = init_menu(&pool).await;
            assert_eq!(menu.dish_count, 0);
            let dish = init_dish(menu.key());
            let mut res = DatabaseRecord::create(dish, &pool).await.unwrap();
            res.name = String::from("New Name");
            res.save(&pool).await.unwrap();
            let menu = menu.reload(&pool).await.unwrap();
            assert_eq!(menu.dish_count, 1);
            assert_eq!(menu.last_dish_updated.as_ref().unwrap().name, res.name);
            Ok(())
        }

        mod validations_fail {
            use super::*;

            #[maybe_async::test(
                feature = "blocking",
                async(all(not(feature = "blocking")), tokio::test)
            )]
            async fn before_create_hook() -> Result<(), String> {
                let pool = common::setup_db().await;
                let mut menu = init_menu(&pool).await;
                let dish = Dish {
                    name: "dish".to_string(),
                    description: "description".to_string(),
                    price: 0,
                    menu_id: menu.key().clone(),
                };
                match DatabaseRecord::create(dish, &pool).await {
                    Ok(_) => return Err("Hook should have called validations".to_string()),
                    Err(_) => (),
                };
                menu.reload_mut(&pool).await.unwrap();
                assert_eq!(menu.dish_count, 0);
                assert!(menu.last_dish_updated.is_none());
                Ok(())
            }

            #[maybe_async::test(
                feature = "blocking",
                async(all(not(feature = "blocking")), tokio::test)
            )]
            async fn succeeds_without_hook() -> Result<(), String> {
                let pool = common::setup_db().await;
                let mut menu = init_menu(&pool).await;
                let dish = Dish {
                    name: "dish".to_string(),
                    description: "description".to_string(),
                    price: 0,
                    menu_id: menu.key().clone(),
                };
                match DatabaseRecord::force_create(dish, &pool).await {
                    Ok(_) => (),
                    Err(_) => return Err("Hook should not have been called".to_string()),
                };
                menu.reload_mut(&pool).await.unwrap();
                // Hooks were not called
                assert_eq!(menu.dish_count, 0);
                assert!(menu.last_dish_updated.is_none());
                Ok(())
            }

            #[maybe_async::test(
                feature = "blocking",
                async(all(not(feature = "blocking")), tokio::test)
            )]
            async fn before_save_hook() -> Result<(), String> {
                let pool = common::setup_db().await;
                let mut menu = init_menu(&pool).await;
                let dish = init_dish(menu.key());
                let mut doc = DatabaseRecord::create(dish, &pool).await.unwrap();
                doc.name = String::from("wrong");
                doc.price = 0;
                match doc.save(&pool).await {
                    Ok(_) => return Err("Hook should have called validations".to_string()),
                    Err(_) => (),
                };
                menu.reload_mut(&pool).await.unwrap();
                assert_eq!(menu.dish_count, 1);
                assert_ne!(&menu.last_dish_updated.as_ref().unwrap().name, "wrong");
                Ok(())
            }
        }
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
        let menu = init_menu(&pool).await;
        let db_record = DatabaseRecord::create(
            Dish {
                name: "Pizza".to_string(),
                description: "Tomato and Mozarella".to_string(),
                price: 10,
                menu_id: menu.key().clone(),
            },
            &pool,
        )
        .await
        .unwrap();
        assert_eq!(
            format!("{}", db_record),
            format!("Dish {} Database Record", db_record.key())
        );
    }
}

mod read {
    use aragog::query::{Comparison, Filter};
    use aragog::ServiceError;

    use super::*;
    use aragog::error::{ArangoError, ArangoHttpError};

    #[maybe_async::maybe_async]
    async fn create_dishes(pool: &DatabaseConnectionPool) -> DatabaseRecord<Dish> {
        let menu = init_menu(pool).await;
        Dish::create(
            Dish {
                name: "Pizza".to_string(),
                description: "Tomato and Mozarella".to_string(),
                price: 10,
                menu_id: menu.key().clone(),
            },
            pool,
        )
        .await
        .unwrap();
        Dish::create(
            Dish {
                name: "Pasta".to_string(),
                description: "Ham and cheese".to_string(),
                price: 6,
                menu_id: menu.key().clone(),
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
                menu_id: menu.key().clone(),
            },
            pool,
        )
        .await
        .unwrap();
        DatabaseRecord::create(init_dish(menu.key()), pool)
            .await
            .unwrap()
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn find() -> Result<(), String> {
        let pool = common::setup_db().await;
        let dish_record = create_dishes(&pool).await;

        let found_record = Dish::find(dish_record.key(), &pool).await.unwrap();
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
            assert_eq!(error.to_string(), "Dish wrong_key not found".to_string());
            if let ServiceError::NotFound { item, id, source } = error {
                assert_eq!(&item, "Dish");
                assert_eq!(&id, "wrong_key");
                assert!(source.is_some());
                let source = source.unwrap();
                assert_eq!(source.http_error, ArangoHttpError::NotFound);
                assert_eq!(source.arango_error, ArangoError::ArangoDocumentNotFound);
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
            common::expect_assert_eq(*value, &found_records[i].name)?;
        }
        let query = Dish::query().sort("price", None).sort("name", None);
        let found_records = Dish::get(query, &pool).await.unwrap().documents;
        for (i, value) in ["Pasta", "Quiche", "Pizza", "Steak"].iter().enumerate() {
            common::expect_assert_eq(*value, &found_records[i].name)?;
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
                        dish.id()
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
                let query = Query::outbound(2, 5, "edges", dish.id());
                common::expect_assert_eq(
                    query.to_aql(),
                    format!(
                        "\
                        FOR a in 2..5 OUTBOUND \'{}\' edges \
                        return a\
                                ",
                        dish.id()
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
                        dish.id()
                    ),
                )?;
                Ok(())
            }
        }
    }
}

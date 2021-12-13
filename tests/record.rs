#[macro_use]
extern crate aragog;

use serde::{Deserialize, Serialize};

use aragog::{DatabaseAccess, DatabaseConnection, DatabaseRecord, Error, Record, Validate};

pub mod common;

#[derive(Serialize, Deserialize, Clone, Record, Debug)]
pub struct Menu {
    pub dish_count: u16,
    pub last_dish_updated: Option<Dish>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Record, Validate)]
#[before_write(func("validate"))]
#[before_create(func("increment_menu"), is_async = true, db_access = true)]
#[before_save(func("last_dish_update"), is_async = true, db_access = true)]
#[after_all(func("after_all"))]
pub struct Dish {
    #[validate(min_length = 3)]
    pub name: String,
    pub description: String,
    #[validate(greater_than(0))]
    pub price: u16,
    pub menu_id: String,
}

impl Dish {
    fn after_all(&self) -> Result<(), Error> {
        println!("Dish written in db");
        Ok(())
    }

    #[maybe_async::maybe_async]
    async fn increment_menu<D>(&self, db_access: &D) -> Result<(), Error>
    where
        D: DatabaseAccess + ?Sized,
    {
        let mut menu: DatabaseRecord<Menu> = Menu::find(&self.menu_id, db_access).await?;
        menu.dish_count += 1;
        menu.last_dish_updated = Some(self.clone());
        menu.save(db_access).await?;
        Ok(())
    }

    #[maybe_async::maybe_async]
    async fn last_dish_update<D>(&self, db_access: &D) -> Result<(), Error>
    where
        D: DatabaseAccess + ?Sized,
    {
        let mut menu: DatabaseRecord<Menu> = Menu::find(&self.menu_id, db_access).await?;
        menu.last_dish_updated = Some(self.clone());
        menu.save(db_access).await?;
        Ok(())
    }
}

fn init_dish(menu_id: &str) -> Dish {
    Dish {
        name: "Quiche".to_string(),
        description: "Part de quiche aux oeufs, lardons et fromage".to_string(),
        price: 7,
        menu_id: menu_id.to_string(),
    }
}

#[maybe_async::maybe_async]
async fn init_menu(db_access: &DatabaseConnection) -> DatabaseRecord<Menu> {
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
    assert_eq!(Dish::COLLECTION_NAME, "Dish");
}

mod write {
    use super::*;

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn can_be_recorded_and_retrieved() -> Result<(), String> {
        let connection = common::setup_db().await;
        let menu = init_menu(&connection).await;
        let dish = init_dish(menu.key());
        let dish_record = DatabaseRecord::create(dish, &connection).await.unwrap();
        let found_record = Dish::find(dish_record.key(), &connection).await.unwrap();
        common::expect_assert_eq(dish_record.record, found_record.record)?;
        Ok(())
    }

    #[should_panic(expected = "Conflict")]
    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn can_fail() {
        let connection = common::setup_db().await;
        let menu = init_menu(&connection).await;
        let dish = init_dish(menu.key());
        DatabaseRecord::create(dish.clone(), &connection)
            .await
            .unwrap();
        DatabaseRecord::create(dish, &connection).await.unwrap();
    }

    mod hooks {
        use super::*;

        #[maybe_async::test(
            feature = "blocking",
            async(all(not(feature = "blocking")), tokio::test)
        )]
        async fn before_create_and_save_hook() -> Result<(), String> {
            let connection = common::setup_db().await;
            let menu = init_menu(&connection).await;
            assert_eq!(menu.dish_count, 0);
            let dish = init_dish(menu.key());
            let mut res = DatabaseRecord::create(dish, &connection).await.unwrap();
            res.name = String::from("New Name");
            res.save(&connection).await.unwrap();
            let menu = menu.reload(&connection).await.unwrap();
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
                let connection = common::setup_db().await;
                let mut menu = init_menu(&connection).await;
                let dish = Dish {
                    name: "dish".to_string(),
                    description: "description".to_string(),
                    price: 0,
                    menu_id: menu.key().clone(),
                };
                if DatabaseRecord::create(dish, &connection).await.is_ok() {
                    return Err("Hook should have called validations".to_string());
                }
                menu.reload_mut(&connection).await.unwrap();
                assert_eq!(menu.dish_count, 0);
                assert!(menu.last_dish_updated.is_none());
                Ok(())
            }

            #[maybe_async::test(
                feature = "blocking",
                async(all(not(feature = "blocking")), tokio::test)
            )]
            async fn succeeds_without_hook() -> Result<(), String> {
                let connection = common::setup_db().await;
                let mut menu = init_menu(&connection).await;
                let dish = Dish {
                    name: "dish".to_string(),
                    description: "description".to_string(),
                    price: 0,
                    menu_id: menu.key().clone(),
                };
                match DatabaseRecord::force_create(dish, &connection).await {
                    Ok(_) => (),
                    Err(_) => return Err("Hook should not have been called".to_string()),
                };
                menu.reload_mut(&connection).await.unwrap();
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
                let connection = common::setup_db().await;
                let mut menu = init_menu(&connection).await;
                let dish = init_dish(menu.key());
                let mut doc = DatabaseRecord::create(dish, &connection).await.unwrap();
                doc.name = String::from("wrong");
                doc.price = 0;
                if doc.save(&connection).await.is_ok() {
                    return Err("Hook should have called validations".to_string());
                }
                menu.reload_mut(&connection).await.unwrap();
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
        let connection = common::setup_db().await;
        let menu = init_menu(&connection).await;
        let db_record = DatabaseRecord::create(
            Dish {
                name: "Pizza".to_string(),
                description: "Tomato and Mozarella".to_string(),
                price: 10,
                menu_id: menu.key().clone(),
            },
            &connection,
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
    use aragog::query::{Comparison, Filter, QueryCursor};
    use aragog::Error;

    use super::*;
    use aragog::error::{ArangoError, ArangoHttpError};

    #[maybe_async::maybe_async]
    async fn create_dishes(connection: &DatabaseConnection) -> DatabaseRecord<Dish> {
        let menu = init_menu(connection).await;
        Dish::create(
            Dish {
                name: "Pizza".to_string(),
                description: "Tomato and Mozarella".to_string(),
                price: 10,
                menu_id: menu.key().clone(),
            },
            connection,
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
            connection,
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
            connection,
        )
        .await
        .unwrap();
        DatabaseRecord::create(init_dish(menu.key()), connection)
            .await
            .unwrap()
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn find() -> Result<(), String> {
        let connection = common::setup_db().await;
        let dish_record = create_dishes(&connection).await;

        let found_record = Dish::find(dish_record.key(), &connection).await.unwrap();
        common::expect_assert_eq(dish_record.record, found_record.record)?;
        Ok(())
    }

    #[should_panic(expected = "NotFound")]
    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn find_can_fail() {
        let connection = common::setup_db().await;
        create_dishes(&connection).await;
        Dish::find("wrong_key", &connection).await.unwrap();
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn find_can_fail_with_correct_error() -> Result<(), String> {
        let connection = common::setup_db().await;
        create_dishes(&connection).await;
        let res = Dish::find("wrong_key", &connection).await;
        if let Err(error) = res {
            assert_eq!(error.to_string(), "Dish wrong_key not found".to_string());
            if let Error::NotFound { item, id, source } = error {
                assert_eq!(&item, "Dish");
                assert_eq!(&id, "wrong_key");
                assert!(source.is_some());
                let source = source.unwrap();
                assert_eq!(source.http_error, ArangoHttpError::NotFound);
                assert_eq!(source.arango_error, ArangoError::ArangoDocumentNotFound);
                Ok(())
            } else {
                Err("The find should return a NotFound".to_string())
            }
        } else {
            Err("The find should return an error".to_string())
        }
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn query_uniq() -> Result<(), String> {
        let connection = common::setup_db().await;
        let dish_record = create_dishes(&connection).await;
        let query = Dish::query().filter(
            Filter::new(Comparison::field("name").equals_str("Quiche"))
                .and(Comparison::field("price").equals(7)),
        );

        let found_record = Dish::get(query, &connection).await.unwrap().uniq().unwrap();
        common::expect_assert_eq(dish_record.record, found_record.record)?;
        Ok(())
    }

    #[should_panic(expected = "NotFound")]
    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn query_uniq_can_fail() {
        let connection = common::setup_db().await;
        let query =
            Dish::query().filter(Filter::new(Comparison::field("name").equals_str("Quiche")));

        Dish::get(query, &connection).await.unwrap().uniq().unwrap();
    }

    #[should_panic(expected = "NotFound")]
    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn query_uniq_can_fail_on_multiple_found() {
        let connection = common::setup_db().await;
        create_dishes(&connection).await;
        let query = Dish::query().filter(Filter::new(Comparison::field("price").equals(10)));

        Dish::get(query, &connection).await.unwrap().uniq().unwrap();
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn query() -> Result<(), String> {
        let connection = common::setup_db().await;
        let dish_record = create_dishes(&connection).await;
        let query = Dish::query().filter(
            Filter::new(Comparison::field("name").equals_str("Quiche"))
                .and(Comparison::field("price").equals(7)),
        );

        let found_records = Dish::get(query, &connection).await.unwrap();
        common::expect_assert_eq(found_records.len(), 1)?;
        common::expect_assert_eq(dish_record.record, found_records[0].record.clone())?;
        Ok(())
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn query_on_batches() -> Result<(), String> {
        let connection = common::setup_db().await;
        create_dishes(&connection).await;
        let cursor: QueryCursor<Dish> = Dish::get_in_batches(Dish::query(), &connection, 2)
            .await
            .unwrap();
        common::expect_assert_eq(cursor.result().len(), 2)?;
        common::expect_assert(cursor.has_more())?;
        let cursor: QueryCursor<Dish> = Dish::get_in_batches(Dish::query(), &connection, 10)
            .await
            .unwrap();
        common::expect_assert_eq(cursor.result().len(), 4)?;
        common::expect_assert(!cursor.has_more())?;
        Ok(())
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn query_can_return_empty_vec() {
        let connection = common::setup_db().await;
        let query =
            Dish::query().filter(Filter::new(Comparison::field("name").equals_str("Quiche")));
        let found_records = Dish::get(query, &connection).await.unwrap();
        common::expect_assert_eq(found_records.len(), 0).unwrap();
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn query_on_multiple_found() -> Result<(), String> {
        let connection = common::setup_db().await;
        create_dishes(&connection).await;
        // Can return multiple
        let query = Dish::query().filter(Filter::new(Comparison::field("price").equals(10)));
        let found_records = Dish::get(query, &connection).await.unwrap();
        common::expect_assert_eq(found_records.len(), 2)?;

        // Limit features
        let query = Dish::query()
            .filter(Filter::new(Comparison::field("price").equals(10)))
            .limit(1, None);
        let found_records = Dish::get(query, &connection).await.unwrap();
        common::expect_assert_eq(found_records.len(), 1)?;

        let query = Dish::query();
        let found_records = Dish::get(query, &connection).await.unwrap();
        common::expect_assert_eq(found_records.len(), 4)?;

        let query = Dish::query().limit(2, Some(3));
        let found_records = Dish::get(query, &connection).await.unwrap();
        common::expect_assert_eq(found_records.len(), 1)?;

        // Sorting
        let query = Dish::query().sort("name", None);
        let found_records = Dish::get(query, &connection).await.unwrap();
        for (i, value) in ["Pasta", "Pizza", "Quiche", "Steak"].iter().enumerate() {
            common::expect_assert_eq(*value, &found_records[i].name)?;
        }
        let query = Dish::query().sort("price", None).sort("name", None);
        let found_records = Dish::get(query, &connection).await.unwrap();
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
        let connection = common::setup_db().await;
        create_dishes(&connection).await;
        let query = Dish::query().filter(
            Filter::new(Comparison::field("name").equals_str("Quiche"))
                .and(Comparison::field("price").equals(7)),
        );

        let res = Dish::exists(query, &connection).await;
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
                let connection = common::setup_db().await;
                let dish = create_dishes(&connection).await;
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
                let connection = common::setup_db().await;
                let dish = create_dishes(&connection).await;
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
                let connection = common::setup_db().await;
                let dish = create_dishes(&connection).await;
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

mod enum_record {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize, Record)]
    enum Dish {
        Adult {
            price: u16,
            alcohol: bool,
            name: String,
        },
        Child {
            price: u16,
            name: String,
        },
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn can_be_create() {
        let conn = common::setup_db().await;
        let dish = Dish::Adult {
            price: 20,
            alcohol: true,
            name: "Baba au rhum".to_string(),
        };
        DatabaseRecord::create(dish, &conn).await.unwrap();
    }
}

mod collection_name {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, Record)]
    #[collection_name = "Dish"]
    pub struct Dish1 {}

    #[derive(Serialize, Deserialize, Clone, Record)]
    #[collection_name = "Dish"]
    pub struct Dish2 {}

    #[test]
    fn has_correct_collection_name() {
        assert_eq!(Dish1::COLLECTION_NAME, "Dish");
        assert_eq!(Dish2::COLLECTION_NAME, "Dish");
    }
}

mod all_hooks {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, Record, Default)]
    #[before_create(func = "before_create")]
    #[before_save(func = "before_save")]
    #[before_delete(func = "before_delete")]
    #[before_write(func = "before_write")]
    #[before_all(func = "before_all")]
    #[after_create(func = "after_create")]
    #[after_save(func = "after_save")]
    #[after_delete(func = "after_delete")]
    #[after_write(func = "after_write")]
    #[after_all(func = "after_all")]
    pub struct Dish {
        before_create_count: u16,
        before_save_count: u16,
        before_delete_count: u16,
        before_write_count: u16,
        before_all_count: u16,
        after_create_count: u16,
        after_save_count: u16,
        after_delete_count: u16,
        after_write_count: u16,
        after_all_count: u16,
    }

    impl Dish {
        fn before_create(&mut self) -> Result<(), Error> {
            self.before_create_count += 1;
            Ok(())
        }
        fn before_save(&mut self) -> Result<(), Error> {
            self.before_save_count += 1;
            Ok(())
        }
        fn before_delete(&mut self) -> Result<(), Error> {
            self.before_delete_count += 1;
            Ok(())
        }
        fn before_write(&mut self) -> Result<(), Error> {
            self.before_write_count += 1;
            Ok(())
        }
        fn before_all(&mut self) -> Result<(), Error> {
            self.before_all_count += 1;
            Ok(())
        }
        fn after_create(&mut self) -> Result<(), Error> {
            self.after_create_count += 1;
            Ok(())
        }
        fn after_save(&mut self) -> Result<(), Error> {
            self.after_save_count += 1;
            Ok(())
        }
        fn after_delete(&mut self) -> Result<(), Error> {
            self.after_delete_count += 1;
            Ok(())
        }
        fn after_write(&mut self) -> Result<(), Error> {
            self.after_write_count += 1;
            Ok(())
        }
        fn after_all(&mut self) -> Result<(), Error> {
            self.after_all_count += 1;
            Ok(())
        }
    }

    #[maybe_async::test(
        feature = "blocking",
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn hooks_are_called() {
        let db = common::setup_db().await;
        let doc = Dish::default();
        let mut rec = DatabaseRecord::create(doc, &db).await.unwrap();
        assert_eq!(rec.before_create_count, 1);
        assert_eq!(rec.before_save_count, 0);
        assert_eq!(rec.before_delete_count, 0);
        assert_eq!(rec.before_write_count, 1);
        assert_eq!(rec.before_all_count, 1);
        assert_eq!(rec.after_create_count, 1);
        assert_eq!(rec.after_save_count, 0);
        assert_eq!(rec.after_delete_count, 0);
        assert_eq!(rec.after_write_count, 1);
        assert_eq!(rec.after_all_count, 1);
        rec.record = Dish::default();
        rec.save(&db).await.unwrap();
        assert_eq!(rec.before_create_count, 0);
        assert_eq!(rec.before_save_count, 1);
        assert_eq!(rec.before_delete_count, 0);
        assert_eq!(rec.before_write_count, 1);
        assert_eq!(rec.before_all_count, 1);
        assert_eq!(rec.after_create_count, 0);
        assert_eq!(rec.after_save_count, 1);
        assert_eq!(rec.after_delete_count, 0);
        assert_eq!(rec.after_write_count, 1);
        assert_eq!(rec.after_all_count, 1);
        rec.record = Dish::default();
        rec.delete(&db).await.unwrap();
        assert_eq!(rec.before_create_count, 0);
        assert_eq!(rec.before_save_count, 0);
        assert_eq!(rec.before_delete_count, 1);
        assert_eq!(rec.before_write_count, 0);
        assert_eq!(rec.before_all_count, 1);
        assert_eq!(rec.after_create_count, 0);
        assert_eq!(rec.after_save_count, 0);
        assert_eq!(rec.after_delete_count, 1);
        assert_eq!(rec.after_write_count, 0);
        assert_eq!(rec.after_all_count, 1);
    }
}

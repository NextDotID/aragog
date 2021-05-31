#[macro_use]
extern crate aragog;

use serde::{Deserialize, Serialize};

use aragog::transaction::{Transaction, TransactionOutput};
use aragog::{DatabaseAccess, DatabaseRecord, Record};

pub mod common;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Record)]
pub struct User {
    pub name: String,
    pub description: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Record)]
pub struct Dish {
    pub name: String,
    pub price: u16,
}

mod safe_execute {
    use super::*;

    mod commit {
        use super::*;
        use aragog::error::{ArangoError, ArangoHttpError};
        use aragog::ServiceError;

        #[cfg(feature = "async")]
        async fn get_correct_result(
            transaction: &Transaction,
            user_doc: &User,
            dish_doc: &Dish,
        ) -> TransactionOutput<Vec<DatabaseRecord<User>>> {
            transaction
                .safe_execute(|connection| async move {
                    let mut res = vec![];
                    res.push(DatabaseRecord::create(user_doc.clone(), &connection).await?);
                    res.push(DatabaseRecord::create(user_doc.clone(), &connection).await?);
                    res.push(DatabaseRecord::create(user_doc.clone(), &connection).await?);
                    DatabaseRecord::create(dish_doc.clone(), &connection).await?;
                    Ok(res)
                })
                .await
                .unwrap()
        }

        #[cfg(not(feature = "async"))]
        fn get_correct_result(
            transaction: &Transaction,
            user_doc: &User,
            dish_doc: &Dish,
        ) -> TransactionOutput<Vec<DatabaseRecord<User>>> {
            transaction
                .safe_execute(|connection| {
                    let res = vec![
                        DatabaseRecord::create(user_doc.clone(), &connection)?,
                        DatabaseRecord::create(user_doc.clone(), &connection)?,
                        DatabaseRecord::create(user_doc.clone(), &connection)?,
                    ];
                    DatabaseRecord::create(dish_doc.clone(), &connection)?;
                    Ok(res)
                })
                .unwrap()
        }

        #[maybe_async::test(
            feature = "blocking",
            async(all(not(feature = "blocking")), tokio::test)
        )]
        async fn commit_works_on_global_transaction() {
            let connection = common::setup_db().await;
            let user_doc = User {
                name: "Felix".to_string(),
                description: "LM".to_string(),
                email: "felix.maneville@qonfucius.team".to_string(),
            };
            let dish_doc = Dish {
                name: "Pizza".to_string(),
                price: 10,
            };
            let count = connection
                .get_collection("User")
                .unwrap()
                .record_count()
                .await
                .unwrap();
            assert_eq!(count, 0);
            let count = connection
                .get_collection("Dish")
                .unwrap()
                .record_count()
                .await
                .unwrap();
            assert_eq!(count, 0);
            let transaction = Transaction::new(&connection).await.unwrap();
            let result = get_correct_result(&transaction, &user_doc, &dish_doc).await;
            assert!(result.is_committed());
            assert_eq!(result.unwrap().len(), 3);
            let count = connection
                .get_collection("User")
                .unwrap()
                .record_count()
                .await
                .unwrap();
            assert_eq!(count, 3);
            let count = connection
                .get_collection("Dish")
                .unwrap()
                .record_count()
                .await
                .unwrap();
            assert_eq!(count, 1);
        }

        #[maybe_async::test(
            feature = "blocking",
            async(all(not(feature = "blocking")), tokio::test)
        )]
        async fn commit_fails_on_restricted_transaction() {
            let connection = common::setup_db().await;
            let user_doc = User {
                name: "Felix".to_string(),
                description: "LM".to_string(),
                email: "felix.maneville@qonfucius.team".to_string(),
            };
            let dish_doc = Dish {
                name: "Pizza".to_string(),
                price: 10,
            };
            let count = connection
                .get_collection("User")
                .unwrap()
                .record_count()
                .await
                .unwrap();
            assert_eq!(count, 0);
            let count = connection
                .get_collection("Dish")
                .unwrap()
                .record_count()
                .await
                .unwrap();
            assert_eq!(count, 0);
            let transaction = User::transaction(&connection).await.unwrap();
            let result = get_correct_result(&transaction, &user_doc, &dish_doc).await;
            assert!(result.is_aborted());
            match result.err().unwrap() {
                ServiceError::ArangoError(db_error) => {
                    assert_eq!(
                        db_error.arango_error,
                        ArangoError::TransactionUnregisteredCollectionError
                    );
                    assert_eq!(db_error.http_error, ArangoHttpError::BadParameter);
                }
                _ => panic!("Wrong error retured"),
            }
            let count = connection
                .get_collection("User")
                .unwrap()
                .record_count()
                .await
                .unwrap();
            assert_eq!(count, 0);
            let count = connection
                .get_collection("Dish")
                .unwrap()
                .record_count()
                .await
                .unwrap();
            assert_eq!(count, 0);
        }
    }

    mod abort {
        use aragog::ServiceError;

        use super::*;

        #[cfg(feature = "async")]
        async fn get_failing_result(
            transaction: &Transaction,
            doc: &User,
        ) -> TransactionOutput<Vec<DatabaseRecord<User>>> {
            transaction
                .safe_execute(|connection| async move {
                    DatabaseRecord::create(doc.clone(), &connection).await?;
                    DatabaseRecord::create(doc.clone(), &connection).await?;
                    DatabaseRecord::create(doc.clone(), &connection).await?;
                    Err(ServiceError::default())
                })
                .await
                .unwrap()
        }

        #[cfg(not(feature = "async"))]
        fn get_failing_result(
            transaction: &Transaction,
            doc: &User,
        ) -> TransactionOutput<Vec<DatabaseRecord<User>>> {
            transaction
                .safe_execute(|connection| {
                    DatabaseRecord::create(doc.clone(), &connection)?;
                    DatabaseRecord::create(doc.clone(), &connection)?;
                    DatabaseRecord::create(doc.clone(), &connection)?;
                    Err(ServiceError::default())
                })
                .unwrap()
        }

        #[maybe_async::test(
            feature = "blocking",
            async(all(not(feature = "blocking")), tokio::test)
        )]
        async fn abort_works() {
            let connection = common::setup_db().await;
            let doc = User {
                name: "Felix".to_string(),
                description: "LM".to_string(),
                email: "felix.maneville@qonfucius.team".to_string(),
            };
            let count = connection
                .get_collection("User")
                .unwrap()
                .record_count()
                .await
                .unwrap();
            assert_eq!(count, 0);
            let transaction = Transaction::new(&connection).await.unwrap();
            let result = get_failing_result(&transaction, &doc).await;
            assert!(result.is_aborted());
            assert!(matches!(
                result.err().unwrap(),
                ServiceError::InternalError { .. }
            ));
            let count = connection
                .get_collection("User")
                .unwrap()
                .record_count()
                .await
                .unwrap();
            assert_eq!(count, 0);
        }
    }

    mod query {
        use super::*;

        #[maybe_async::test(
            feature = "blocking",
            async(all(not(feature = "blocking")), tokio::test)
        )]
        async fn query_transaction_elements() -> Result<(), String> {
            let db_connection = common::setup_db().await;
            let transaction = Transaction::new(&db_connection).await.unwrap();

            DatabaseRecord::create(
                User {
                    name: "Robert Surcouf".to_string(),
                    description: "Corsaire Français".to_string(),
                    email: "lerevenantmalouin@qonfucius.team".to_string(),
                },
                transaction.database_connection(),
            )
            .await
            .unwrap();

            let query =
                User::query().filter(compare!(field "name").equals_str("Robert Surcouf").into());
            let res = User::get(query.clone(), transaction.database_connection())
                .await
                .unwrap();
            assert_eq!(res.len(), 0);
            transaction.commit().await.unwrap();
            let res = User::get(query.clone(), &db_connection).await.unwrap();
            assert_eq!(res.len(), 1);
            Ok(())
        }
    }
}

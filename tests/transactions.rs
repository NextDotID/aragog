extern crate aragog;

use serde::{Deserialize, Serialize};

use aragog::transaction::{Transaction, TransactionOutput};
use aragog::{DatabaseRecord, Record};

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
                .safe_execute(|pool| async move {
                    let res = vec![
                        DatabaseRecord::create(user_doc.clone(), &pool).await?,
                        DatabaseRecord::create(user_doc.clone(), &pool).await?,
                        DatabaseRecord::create(user_doc.clone(), &pool).await?,
                    ];
                    DatabaseRecord::create(dish_doc.clone(), &pool).await?;
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
                .safe_execute(|pool| {
                    let res = vec![
                        DatabaseRecord::create(user_doc.clone(), &pool)?,
                        DatabaseRecord::create(user_doc.clone(), &pool)?,
                        DatabaseRecord::create(user_doc.clone(), &pool)?,
                    ];
                    DatabaseRecord::create(dish_doc.clone(), &pool)?;
                    Ok(res)
                })
                .unwrap()
        }

        #[maybe_async::test(
            feature = "blocking",
            async(all(not(feature = "blocking")), tokio::test)
        )]
        async fn commit_works_on_global_transaction() {
            let pool = common::setup_db().await;
            let user_doc = User {
                name: "Felix".to_string(),
                description: "LM".to_string(),
                email: "felix.maneville@qonfucius.team".to_string(),
            };
            let dish_doc = Dish {
                name: "Pizza".to_string(),
                price: 10,
            };
            let count = pool.collections["User"].record_count().await.unwrap();
            assert_eq!(count, 0);
            let count = pool.collections["Dish"].record_count().await.unwrap();
            assert_eq!(count, 0);
            let transaction = Transaction::new(&pool).await.unwrap();
            let result = get_correct_result(&transaction, &user_doc, &dish_doc).await;
            assert!(result.is_committed());
            assert_eq!(result.unwrap().len(), 3);
            let count = pool.collections["User"].record_count().await.unwrap();
            assert_eq!(count, 3);
            let count = pool.collections["Dish"].record_count().await.unwrap();
            assert_eq!(count, 1);
        }

        #[maybe_async::test(
            feature = "blocking",
            async(all(not(feature = "blocking")), tokio::test)
        )]
        async fn commit_fails_on_restricted_transaction() {
            let pool = common::setup_db().await;
            let user_doc = User {
                name: "Felix".to_string(),
                description: "LM".to_string(),
                email: "felix.maneville@qonfucius.team".to_string(),
            };
            let dish_doc = Dish {
                name: "Pizza".to_string(),
                price: 10,
            };
            let count = pool.collections["User"].record_count().await.unwrap();
            assert_eq!(count, 0);
            let count = pool.collections["Dish"].record_count().await.unwrap();
            assert_eq!(count, 0);
            let transaction = User::transaction(&pool).await.unwrap();
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
            let count = pool.collections["User"].record_count().await.unwrap();
            assert_eq!(count, 0);
            let count = pool.collections["Dish"].record_count().await.unwrap();
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
                .safe_execute(|pool| async move {
                    DatabaseRecord::create(doc.clone(), &pool).await?;
                    DatabaseRecord::create(doc.clone(), &pool).await?;
                    DatabaseRecord::create(doc.clone(), &pool).await?;
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
                .safe_execute(|pool| {
                    DatabaseRecord::create(doc.clone(), &pool)?;
                    DatabaseRecord::create(doc.clone(), &pool)?;
                    DatabaseRecord::create(doc.clone(), &pool)?;
                    Err(ServiceError::default())
                })
                .unwrap()
        }

        #[maybe_async::test(
            feature = "blocking",
            async(all(not(feature = "blocking")), tokio::test)
        )]
        async fn abort_works() {
            let pool = common::setup_db().await;
            let doc = User {
                name: "Felix".to_string(),
                description: "LM".to_string(),
                email: "felix.maneville@qonfucius.team".to_string(),
            };
            let count = pool.collections["User"].record_count().await.unwrap();
            assert_eq!(count, 0);
            let transaction = Transaction::new(&pool).await.unwrap();
            let result = get_failing_result(&transaction, &doc).await;
            assert!(result.is_aborted());
            assert!(match result.err().unwrap() {
                ServiceError::InternalError { .. } => true,
                _ => false,
            });
            let count = pool.collections["User"].record_count().await.unwrap();
            assert_eq!(count, 0);
        }
    }
}

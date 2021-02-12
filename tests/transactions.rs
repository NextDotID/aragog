extern crate aragog;

use serde::{Deserialize, Serialize};

use aragog::{DatabaseRecord, Record};

pub mod common;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Record)]
pub struct User {
    pub name: String,
    pub description: String,
    pub email: String,
}

mod safe_execute {
    use aragog::transaction::{Transaction, TransactionOutput};

    use super::*;

    mod success {
        use super::*;

        #[cfg(feature = "async")]
        async fn get_correct_result(
            transaction: &Transaction,
            doc: &User,
        ) -> TransactionOutput<Vec<DatabaseRecord<User>>> {
            transaction
                .safe_execute(|pool| async move {
                    let res = vec![
                        DatabaseRecord::create(doc.clone(), &pool).await?,
                        DatabaseRecord::create(doc.clone(), &pool).await?,
                        DatabaseRecord::create(doc.clone(), &pool).await?,
                    ];
                    Ok(res)
                })
                .await
                .unwrap()
        }

        #[cfg(not(feature = "async"))]
        fn get_correct_result(
            transaction: &Transaction,
            doc: &User,
        ) -> TransactionOutput<Vec<DatabaseRecord<User>>> {
            transaction
                .safe_execute(|pool| {
                    let res = vec![
                        DatabaseRecord::create(doc.clone(), &pool)?,
                        DatabaseRecord::create(doc.clone(), &pool)?,
                        DatabaseRecord::create(doc.clone(), &pool)?,
                    ];
                    Ok(res)
                })
                .unwrap()
        }

        #[maybe_async::test(
            feature = "blocking",
            async(all(not(feature = "blocking")), tokio::test)
        )]
        async fn commit_works() {
            let pool = common::setup_db().await;
            let doc = User {
                name: "Felix".to_string(),
                description: "LM".to_string(),
                email: "felix.maneville@qonfucius.team".to_string(),
            };
            let count = pool.collections["User"].record_count().await.unwrap();
            assert_eq!(count, 0);
            let transaction = Transaction::new(&pool).await.unwrap();
            let result = get_correct_result(&transaction, &doc).await;
            assert!(result.is_committed());
            assert_eq!(result.unwrap().len(), 3);
            let count = pool.collections["User"].record_count().await.unwrap();
            assert_eq!(count, 3);
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
                    Err(ServiceError::InternalError)
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
                    Err(ServiceError::InternalError)
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
                ServiceError::InternalError => true,
                _ => false,
            });
            let count = pool.collections["User"].record_count().await.unwrap();
            assert_eq!(count, 0);
        }
    }
}

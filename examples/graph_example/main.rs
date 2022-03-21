extern crate env_logger;

use aragog::{query::Query, AuthMode};
use aragog::{DatabaseConnection, DatabaseRecord};

use crate::models::{Character, ChildOf};

mod models;

const DEFAULT_DB_HOST: &str = "http://localhost:8529";
const DEFAULT_DB_NAME: &str = "aragog_test";
const DEFAULT_DB_USER: &str = "test";
const DEFAULT_DB_PWD: &str = "test";

fn create_child() -> ChildOf {
    ChildOf {}
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info,aragog=debug");
    env_logger::init();

    // Connect to database and generates collections and indexes
    let db_connection = DatabaseConnection::builder()
        .with_credentials(
            &std::env::var("DB_HOST").unwrap_or_else(|_| DEFAULT_DB_HOST.to_string()),
            &std::env::var("DB_NAME").unwrap_or_else(|_| DEFAULT_DB_NAME.to_string()),
            &std::env::var("DB_USER").unwrap_or_else(|_| DEFAULT_DB_USER.to_string()),
            &std::env::var("DB_PWD").unwrap_or_else(|_| DEFAULT_DB_PWD.to_string()),
        )
        .with_auth_mode(AuthMode::Jwt)
        .with_schema_path("examples/graph_example/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();
    // Testing purposes
    db_connection.truncate().await;

    // Character creation

    // Stark
    let ned = DatabaseRecord::create(
        Character {
            name: "Ned".to_string(),
            surname: "Stark".to_string(),
        },
        &db_connection,
    )
    .await
    .unwrap();
    let catelyn = DatabaseRecord::create(
        Character {
            name: "Catelyn".to_string(),
            surname: "Stark".to_string(),
        },
        &db_connection,
    )
    .await
    .unwrap();

    let robb = DatabaseRecord::create(
        Character {
            name: "Robb".to_string(),
            surname: "Stark".to_string(),
        },
        &db_connection,
    )
    .await
    .unwrap();
    let bran = DatabaseRecord::create(
        Character {
            name: "Bran".to_string(),
            surname: "Stark".to_string(),
        },
        &db_connection,
    )
    .await
    .unwrap();
    let arya = DatabaseRecord::create(
        Character {
            name: "Arya".to_string(),
            surname: "Stark".to_string(),
        },
        &db_connection,
    )
    .await
    .unwrap();
    let sansa = DatabaseRecord::create(
        Character {
            name: "Sansa".to_string(),
            surname: "Stark".to_string(),
        },
        &db_connection,
    )
    .await
    .unwrap();
    let john = DatabaseRecord::create(
        Character {
            name: "John".to_string(),
            surname: "Snow".to_string(),
        },
        &db_connection,
    )
    .await
    .unwrap();

    // Lannister
    let tywin = DatabaseRecord::create(
        Character {
            name: "Tywin".to_string(),
            surname: "Lannister".to_string(),
        },
        &db_connection,
    )
    .await
    .unwrap();
    let jaime = DatabaseRecord::create(
        Character {
            name: "Jaime".to_string(),
            surname: "Lannister".to_string(),
        },
        &db_connection,
    )
    .await
    .unwrap();
    let cersei = DatabaseRecord::create(
        Character {
            name: "Cersei".to_string(),
            surname: "Lannister".to_string(),
        },
        &db_connection,
    )
    .await
    .unwrap();
    let tyrion = DatabaseRecord::create(
        Character {
            name: "Tyrion".to_string(),
            surname: "Lannister".to_string(),
        },
        &db_connection,
    )
    .await
    .unwrap();
    let joffrey = DatabaseRecord::create(
        Character {
            name: "Joffrey".to_string(),
            surname: "Baratheom".to_string(),
        },
        &db_connection,
    )
    .await
    .unwrap();

    // Link characters to their parents

    //    Robb -> Ned
    //    Robb -> Catelyn
    DatabaseRecord::link(&robb, &ned, &db_connection, create_child())
        .await
        .unwrap();
    DatabaseRecord::link(&robb, &catelyn, &db_connection, create_child())
        .await
        .unwrap();
    //    Sansa -> Ned
    //    Sansa -> Catelyn
    DatabaseRecord::link(&sansa, &ned, &db_connection, create_child())
        .await
        .unwrap();
    DatabaseRecord::link(&sansa, &catelyn, &db_connection, create_child())
        .await
        .unwrap();
    //     Arya -> Ned
    //     Arya -> Catelyn
    DatabaseRecord::link(&arya, &ned, &db_connection, create_child())
        .await
        .unwrap();
    DatabaseRecord::link(&arya, &catelyn, &db_connection, create_child())
        .await
        .unwrap();
    //     Bran -> Ned
    //     Bran -> Catelyn
    DatabaseRecord::link(&bran, &ned, &db_connection, create_child())
        .await
        .unwrap();
    DatabaseRecord::link(&bran, &catelyn, &db_connection, create_child())
        .await
        .unwrap();
    //      Jon -> Ned
    DatabaseRecord::link(&john, &ned, &db_connection, create_child())
        .await
        .unwrap();

    //    Jaime -> Tywin
    DatabaseRecord::link(&jaime, &tywin, &db_connection, create_child())
        .await
        .unwrap();
    //   Cersei -> Tywin
    DatabaseRecord::link(&cersei, &tywin, &db_connection, create_child())
        .await
        .unwrap();
    //   Tyrion -> Tywin
    DatabaseRecord::link(&tyrion, &tywin, &db_connection, create_child())
        .await
        .unwrap();
    //  Joffrey -> Jaime
    //  Joffrey -> Cersei
    DatabaseRecord::link(&joffrey, &cersei, &db_connection, create_child())
        .await
        .unwrap();
    DatabaseRecord::link(&joffrey, &jaime, &db_connection, create_child())
        .await
        .unwrap();

    // Requests

    // Find catelyn children
    let children =
        DatabaseRecord::<Character>::get(&catelyn.inbound_query(1, 1, "ChildOf"), &db_connection)
            .await
            .unwrap();
    assert_eq!(
        children
            .iter()
            .map(|r| r.id().as_str())
            .collect::<Vec<&str>>(),
        vec![robb.id(), sansa.id(), arya.id(), bran.id()]
    );
    // Find ned children
    let children =
        DatabaseRecord::<Character>::get(&ned.inbound_query(1, 1, "ChildOf"), &db_connection)
            .await
            .unwrap();
    assert_eq!(
        children
            .iter()
            .map(|r| r.id().as_str())
            .collect::<Vec<&str>>(),
        vec![robb.id(), sansa.id(), arya.id(), bran.id(), john.id()]
    );

    // Find joffrey ancestors
    let ancestors = DatabaseRecord::<Character>::get(
        &joffrey.outbound_query(1, 2, "ChildOf").distinct(),
        &db_connection,
    )
    .await
    .unwrap();
    assert_eq!(
        ancestors
            .iter()
            .map(|r| r.id().as_str())
            .collect::<Vec<&str>>(),
        vec![cersei.id(), tywin.id(), jaime.id()]
    );

    // Find all brothers and nephews, returns self
    let relatives = DatabaseRecord::<Character>::get(
        &tyrion.outbound_query(1, 2, "ChildOf").join_inbound(
            1,
            2,
            false,
            Query::new("ChildOf").distinct(),
        ),
        &db_connection,
    )
    .await
    .unwrap();
    assert_eq!(
        relatives
            .iter()
            .map(|r| r.id().as_str())
            .collect::<Vec<&str>>(),
        vec![jaime.id(), joffrey.id(), cersei.id(), tyrion.id()]
    );

    println!("Done !");
}

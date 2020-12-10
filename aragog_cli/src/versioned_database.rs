use std::ops::Deref;

use arangors::client::reqwest::ReqwestClient;
use arangors::document::options::{InsertOptions, ReplaceOptions};
use arangors::Connection;
use arangors::{Collection, Database};
use serde::{Deserialize, Serialize};

use aragog::schema::DatabaseSchema;

use crate::config::Config;
use crate::error::MigrationError;
use crate::migration::MigrationVersion;
use crate::LOG_STR;

const SCHEMA_DOC_KEY: &str = "DatabaseSchema";

#[derive(Serialize, Deserialize)]
struct SchemaWithKey {
    pub _key: String,
    pub version: Option<MigrationVersion>,
    pub collections: Vec<String>,
}

#[derive(Debug)]
pub struct VersionedDatabase {
    pub db: Database<ReqwestClient>,
    pub collection: Collection<ReqwestClient>,
    pub schema: DatabaseSchema,
}

impl VersionedDatabase {
    pub fn init(config: &Config) -> Result<Self, MigrationError> {
        let connection =
            Connection::establish_basic_auth(&config.db_host, &config.db_user, &config.db_pwd)
                .unwrap();
        let db: Database<ReqwestClient> = match connection.db(&config.db_name) {
            Ok(val) => val,
            Err(_) => {
                println!(
                    "{} Missing database {}, creating it...",
                    LOG_STR, &config.db_name
                );
                connection.create_database(&config.db_name)?
            }
        };
        let collection = match db.collection(&config.collection_name) {
            Ok(coll) => coll,
            Err(_error) => {
                println!(
                    "{} Missing collection {}, creating it...",
                    LOG_STR, &config.collection_name
                );
                db.create_collection(&config.collection_name)?
            }
        };
        let schema = match collection.document(SCHEMA_DOC_KEY) {
            Ok(doc) => doc.document,
            Err(_err) => {
                println!(
                    "{} Missing database schema document, creating it...",
                    LOG_STR
                );
                let schema = DatabaseSchema::default();
                let doc = SchemaWithKey {
                    _key: SCHEMA_DOC_KEY.to_string(),
                    version: schema.version,
                    collections: vec![],
                };
                collection
                    .create_document(doc, InsertOptions::builder().wait_for_sync(true).build())?;
                DatabaseSchema::default()
            }
        };
        Ok(Self {
            db,
            collection,
            schema,
        })
    }

    pub fn save(&self) -> Result<(), MigrationError> {
        // TODO: remove the to_string on arangors 0.4.6 and use clone()
        let doc = serde_json::to_string(&self.schema).unwrap();
        let schema: DatabaseSchema = serde_json::from_str(&doc).unwrap();
        self.collection.replace_document(
            SCHEMA_DOC_KEY,
            schema,
            ReplaceOptions::builder().wait_for_sync(true).build(),
            None,
        )?;
        Ok(())
    }

    pub fn schema_version(&self) -> MigrationVersion {
        self.schema.version.unwrap_or(0)
    }
}

impl Deref for VersionedDatabase {
    type Target = Database<ReqwestClient>;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

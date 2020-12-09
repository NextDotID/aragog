use crate::error::MigrationError;
use crate::migration_data::MigrationData;
use crate::LOG_STR;
use aragog::schema::DatabaseSchema;
use arangors::client::reqwest::ReqwestClient;
use arangors::Database;
use chrono::Utc;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub type MigrationVersion = u64;

const HELP_MESSAGE: &str = "# The migration files contain two sections: \n\
                            # - up: The commands to execute on migration \n\
                            # - down: The commands to execute on rollback (optional) \n\
                            # check https://docs.rs/aragog_cli for complete documentation and examples \n";
const MIGRATION_PATH: &str = "migrations";

#[derive(Debug)]
pub struct Migration {
    pub name: String,
    pub version: MigrationVersion,
    pub data: MigrationData,
}

impl Migration {
    pub fn migration_path(schema_path: &str) -> Result<String, MigrationError> {
        let db_path = format!("{}/{}", schema_path, MIGRATION_PATH);
        if !Path::new(&db_path).is_dir() {
            println!("{} Missing {}/ path. creating it..", MIGRATION_PATH, LOG_STR);
            fs::create_dir(&db_path)?;
        }
        Ok(db_path)
    }

    pub fn new(name: &str, schema_path: &str) -> Result<Self, MigrationError> {
        let data = MigrationData::default();
        let data_str = serde_yaml::to_string(&data).unwrap();
        let version = Utc::now().timestamp_millis() as u64;
        let migration_path = Self::migration_path(schema_path)?;
        let path = format!("{}/{}_{}.yaml", migration_path, version, name.to_ascii_lowercase());
        let mut file = File::create(&path)?;
        let buff = format!("{}{}", HELP_MESSAGE, data_str);
        file.write_all(buff.as_bytes())?;
        println!("{} Created Migration {}", LOG_STR, path);
        Ok(Self {
            name: name.to_string(),
            version,
            data,
        })
    }

    pub fn load(file_name: &str, schema_path: &str) -> Result<Self, MigrationError> {
        let file_path = format!("{}/{}/{}", schema_path, MIGRATION_PATH, file_name);
        println!("{} Loading migration file {}", LOG_STR, file_path);
        let mut split = file_name.split("_");
        let version = match split.next() {
            Some(str) => str,
            None => {
                return Err(MigrationError::InvalidFileName {
                    file_name: file_name.to_string(),
                });
            }
        };
        let version: MigrationVersion = match version.parse() {
            Ok(value) => value,
            Err(_error) => {
                return Err(MigrationError::InvalidFileName {
                    file_name: file_name.to_string(),
                });
            }
        };
        let name = split.collect();
        let data = MigrationData::load(&file_path)?;
        Ok(Self {
            name,
            version,
            data,
        })
    }

    pub fn apply_up(
        self,
        schema: &mut DatabaseSchema,
        db: &Database<ReqwestClient>,
    ) -> Result<MigrationVersion, MigrationError> {
        println!("{} Apply Migration {} ...", LOG_STR, &self.name);
        for operation in self.data.up.into_iter() {
            operation.apply(schema, db)?;
        }
        println!("{} Done.", LOG_STR);
        Ok(self.version)
    }

    pub fn apply_down(
        self,
        schema: &mut DatabaseSchema,
        db: &Database<ReqwestClient>,
    ) -> Result<MigrationVersion, MigrationError> {
        println!("{} Rollback Migration {} ...", LOG_STR, &self.name);
        for operation in self.data.down.unwrap_or(Vec::new()).into_iter() {
            operation.apply(schema, db)?;
        }
        println!("{} Done.", LOG_STR);
        Ok(self.version)
    }
}

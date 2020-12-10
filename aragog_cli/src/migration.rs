use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use chrono::Utc;

use crate::error::MigrationError;
use crate::log;
use crate::log_level::LogLevel;
use crate::migration_data::MigrationData;
use crate::versioned_database::VersionedDatabase;

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
        if !Path::new(&schema_path).is_dir() {
            return Err(MigrationError::InitError {
                item: schema_path.to_string(),
                message: String::from("Is not a valid directory"),
            });
        }
        let db_path = format!("{}/{}", schema_path, MIGRATION_PATH);
        if !Path::new(&db_path).is_dir() {
            log(
                format!(
                    "Missing {}/ path in {}. creating it...",
                    MIGRATION_PATH, schema_path
                ),
                LogLevel::Debug,
            );
            fs::create_dir(&db_path)?;
        }
        Ok(db_path)
    }

    pub fn new(name: &str, schema_path: &str) -> Result<Self, MigrationError> {
        let data = MigrationData::default();
        let data_str = serde_yaml::to_string(&data).unwrap();
        let version = Utc::now().timestamp_millis() as u64;
        let migration_path = Self::migration_path(schema_path)?;
        let path = format!(
            "{}/{}_{}.yaml",
            migration_path,
            version,
            name.to_ascii_lowercase()
        );
        let mut file = File::create(&path)?;
        let buff = format!("{}{}", HELP_MESSAGE, data_str);
        file.write_all(buff.as_bytes())?;
        log(format!("Created Migration {}", path), LogLevel::Info);
        Ok(Self {
            name: name.to_string(),
            version,
            data,
        })
    }

    pub fn load(file_name: &str, schema_path: &str) -> Result<Self, MigrationError> {
        let file_path = format!("{}/{}/{}", schema_path, MIGRATION_PATH, file_name);
        log(
            format!("Loading migration file {}", file_path),
            LogLevel::Info,
        );
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
        let vec: Vec<&str> = split.collect();
        let name = vec.join("_");
        let data = MigrationData::load(&file_path)?;
        Ok(Self {
            name,
            version,
            data,
        })
    }

    pub fn apply_up(self, db: &mut VersionedDatabase) -> Result<MigrationVersion, MigrationError> {
        log(
            format!("Apply Migration {} ...", &self.name),
            LogLevel::Info,
        );
        for operation in self.data.up.into_iter() {
            operation.apply(db)?;
        }
        db.schema.version = Some(self.version);
        log("Done.", LogLevel::Info);
        Ok(self.version)
    }

    pub fn apply_down(
        self,
        db: &mut VersionedDatabase,
    ) -> Result<MigrationVersion, MigrationError> {
        log(
            format!("Rollback Migration {} ...", &self.name),
            LogLevel::Info,
        );
        for operation in self.data.down.unwrap_or(Vec::new()).into_iter() {
            operation.apply(db)?;
        }
        db.schema.version = Some(self.version - 1);
        log("Done.", LogLevel::Info);
        Ok(self.version)
    }
}

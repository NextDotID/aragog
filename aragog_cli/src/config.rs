use clap::ArgMatches;

use crate::error::MigrationError;

const ARAGOG_DEFAULT_COLLECTION: &str = "AragogConfiguration";

#[derive(Debug)]
pub struct Config {
    pub collection_name: String,
    pub schema_path: String,
    pub db_host: String,
    pub db_name: String,
    pub db_user: String,
    pub db_pwd: String,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Result<Self, MigrationError> {
        Ok(Self {
            collection_name: matches
                .value_of("aragog_collection")
                .unwrap_or(ARAGOG_DEFAULT_COLLECTION)
                .to_string(),
            schema_path: {
                match Self::load_str(matches, "schema_path", "SCHEMA_PATH", "path") {
                    Ok(val) => val,
                    Err(_err) => String::from(aragog::schema::SCHEMA_DEFAULT_PATH),
                }
            },
            db_host: Self::load_str(matches, "db_host", "DB_HOST", "db-host")?,
            db_name: Self::load_str(matches, "db_name", "DB_NAME", "db-name")?,
            db_user: Self::load_str(matches, "db_user", "DB_USER", "db-user")?,
            db_pwd: Self::load_str(matches, "db_password", "DB_PASSWORD", "db-password")?,
        })
    }

    pub fn load_str(
        matches: &ArgMatches,
        value: &str,
        env_default: &str,
        option: &str,
    ) -> Result<String, MigrationError> {
        match matches.value_of(value) {
            Some(value) => Ok(value.to_string()),
            None => match std::env::var(env_default) {
                Ok(value) => Ok(value),
                Err(_error) => Err(MigrationError::InitError {
                    item: value.to_string(),
                    message: format!(
                        "{} is not specified, please set the env var or use the --{} option",
                        env_default, option
                    ),
                }),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values_work() {
        let matches = ArgMatches::default();
        std::env::set_var("DB_HOST", "test_DB_HOST");
        std::env::set_var("DB_NAME", "test_DB_NAME");
        std::env::set_var("DB_USER", "test_DB_USER");
        std::env::set_var("DB_PASSWORD", "test_DB_PASSWORD");
        std::env::set_var("SCHEMA_PATH", "test_path");
        let config = Config::new(&matches);
        assert_eq!(config.db_host, "test_DB_HOST".to_string());
        assert_eq!(config.db_name, "test_DB_NAME".to_string());
        assert_eq!(config.db_user, "test_DB_USER".to_string());
        assert_eq!(config.db_pwd, "test_DB_PASSWORD".to_string());
        assert_eq!(config.schema_path, "test_path".to_string());
        std::env::remove_var("SCHEMA_PATH");
        let config = Config::new(&matches);
        assert_eq!(
            config.schema_path,
            aragog::schema::SCHEMA_DEFAULT_PATH.to_string()
        );
    }
}

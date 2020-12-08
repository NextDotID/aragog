use crate::ERROR_STR;
use clap::ArgMatches;

#[derive(Debug)]
pub struct Config {
    pub schema_path: String,
    pub db_host: String,
    pub db_name: String,
    pub db_user: String,
    pub db_pwd: String,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Self {
        Self {
            schema_path: Self::load_str(matches, "schema_path", "SCHEMA_PATH", "path"),
            db_host: Self::load_str(matches, "db_host", "DB_HOST", "db-host"),
            db_name: Self::load_str(matches, "db_name", "DB_NAME", "db-name"),
            db_user: Self::load_str(matches, "db_user", "DB_USER", "db-user"),
            db_pwd: Self::load_str(matches, "db_password", "DB_PASSWORD", "db-password"),
        }
    }

    pub fn load_str(matches: &ArgMatches, value: &str, env_default: &str, option: &str) -> String {
        match matches.value_of(value) {
            Some(value) => value.to_string(),
            None => match std::env::var(env_default) {
                Ok(value) => value,
                Err(_error) => panic!(
                    "{} {} is not specified, please set the env var or use the --{} option",
                    env_default, ERROR_STR, option,
                ),
            },
        }
    }
}

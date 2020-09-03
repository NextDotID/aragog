use serde::{Deserialize, Serialize};
use aragog::{ServiceError, Authenticate};

pub mod common;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub encrypted_password: String,
}

impl Authenticate for User {
    fn authenticate(&self, secret: &str) -> Result<(), ServiceError> {
        // Obviously you must use some bcrypt or Argon2 hashing tool
        if secret != self.encrypted_password {
            return Err(ServiceError::Unauthorized);
        }
        Ok(())
    }
}

#[test]
fn can_succeed() {
    let user = User {
        username: "MichelPolnareff4Ever".to_string(),
        email: "michou@gmail.net".to_string(),
        encrypted_password: "password".to_string()
    };
    user.authenticate("password").unwrap();
}

#[should_panic(expected = "Unauthorized")]
#[test]
fn can_fail() {
    let user = User {
        username: "MichelPolnareff4Ever".to_string(),
        email: "michou@gmail.net".to_string(),
        encrypted_password: "password".to_string()
    };
    user.authenticate("wrong").unwrap();
}

#[cfg(feature = "password_hashing")]
mod password_hashing {
    use super::*;

    #[test]
    fn hash_password() -> Result<(), String> {
        let password = "foobar";
        let secret_key = "keySecret";

        match User::hash_password(password, secret_key) {
            Ok(_value) => Ok(()),
            Err(_error) => Err("Failed to hash password".to_string())
        }
    }

    #[test]
    fn hash_password_can_fail() -> Result<(), String> {
        let password = "";
        let secret_key = "";

        match User::hash_password(password, secret_key) {
            Ok(_value) => Err("Should have failed".to_string()),
            Err(_error) => Ok(())
        }
    }


    #[test]
    fn verify_password() -> Result<(), String> {
        let password = "foobar";
        let secret_key = "keySecret";
        let hashed_password = User::hash_password(password, secret_key).unwrap();

        match User::verify_password(password, &hashed_password, secret_key) {
            Ok(_value) => Ok(()),
            Err(_error) => Err("Failed to verify password".to_string())
        }
    }

    #[test]
    fn verify_password_can_fail() -> Result<(), String> {
        let password = "foobar";
        let secret_key = "keySecret";
        let hashed_password = User::hash_password(password, secret_key).unwrap();

        match User::verify_password("wrong password", &hashed_password, secret_key) {
            Ok(_value) => return Err("Should have failed".to_string()),
            Err(_error) => ()
        };
        match User::verify_password(password, "wrong hash", secret_key) {
            Ok(_value) => return Err("Should have failed".to_string()),
            Err(_error) => ()
        };
        match User::verify_password(password, &hashed_password, "wrong key") {
            Ok(_value) => return Err("Should have failed".to_string()),
            Err(_error) => ()
        };
        Ok(())
    }
}
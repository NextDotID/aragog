#![allow(clippy::used_underscore_binding)]
use crate::{DatabaseRecord, Error, Record};
use arangors_lite::document::response::DocumentResponse;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Serialize, Deserialize)]
pub struct DatabaseRecordDto<T> {
    #[serde(rename(serialize = "_key", deserialize = "_key"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(rename(serialize = "_key", deserialize = "_key"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename(serialize = "_key", deserialize = "_key"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    rev: Option<String>,
    #[serde(flatten)]
    pub record: T,
}

impl<T: Record> DatabaseRecordDto<T> {
    pub fn new(record: T) -> Self {
        Self {
            key: None,
            id: None,
            rev: None,
            record,
        }
    }
}

impl<T: Record> TryInto<DatabaseRecord<T>> for DocumentResponse<DatabaseRecord<T>> {
    type Error = Error;

    fn try_into(self) -> Result<DatabaseRecord<T>, Self::Error> {
        match self {
            DocumentResponse::Silent => Err(Error::InternalError {
                message: Some(String::from("Received unexpected silent document response")),
            }),
            DocumentResponse::Response { new, header, .. } => match new {
                Some(value) => Ok(value),
                None => Err(Error::InternalError {
                    message: Some(format!(
                        "Expected `ArangoDB` to return the new {} document",
                        header._id
                    )),
                }),
            },
        }
    }
}

impl<T: Record> TryInto<DatabaseRecord<T>> for DocumentResponse<DatabaseRecordDto<T>> {
    type Error = Error;

    fn try_into(self) -> Result<DatabaseRecord<T>, Self::Error> {
        match self {
            DocumentResponse::Silent => Err(Error::InternalError {
                message: Some(String::from("Received unexpected silent document response")),
            }),
            DocumentResponse::Response { header, new, .. } => {
                let doc: T = match new {
                    Some(value) => value.record,
                    None => {
                        return Err(Error::InternalError {
                            message: Some(format!(
                                "Expected `ArangoDB` to return the new {} document",
                                header._id
                            )),
                        });
                    }
                };
                Ok(DatabaseRecord {
                    key: header._key.clone(),
                    id: header._id.clone(),
                    rev: header._rev,
                    record: doc,
                })
            }
        }
    }
}

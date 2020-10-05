use aragog::{EdgeRecord, Record, Validate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, EdgeRecord, Validate)]
pub struct ChildOf {
    pub _from: String,
    pub _to: String,
}
use aragog::{EdgeRecord, Record, Validate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, EdgeRecord, Record, Validate)]
#[hook(before_write(func = "validate"))]
#[validate(func = "validate_edge_fields")]
pub struct ChildOf {
    pub _from: String,
    pub _to: String,
}

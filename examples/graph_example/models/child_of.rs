use aragog::{EdgeRecord, Record, Validate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, EdgeRecord)]
pub struct ChildOf {
    pub _from: String,
    pub _to: String,
}

impl Validate for ChildOf {
    fn validations(&self, errors: &mut Vec<String>) {
        self.validate_edge_fields(errors);
    }
}

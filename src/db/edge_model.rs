use serde::{Serialize, Deserialize};
use crate::{Record, Validate};

#[derive(Clone, Serialize, Deserialize, Record)]
pub struct EdgeModel {
    pub _from: String,
    pub _to: String,
}

impl Validate for EdgeModel {
    fn validations(&self, errors: &mut Vec<String>) {
        validate_id_format("from", &self._from, errors);
        validate_id_format("to", &self._to, errors);
    }
}

fn validate_id_format(field_name: &str, field: &str, errors: &mut Vec<String>) {
    let parts :Vec<&str> = field.split('/').collect();
    if parts.len() != 2 {
        errors.push(format!("{} {}has invalid format", field_name, field))
    }
}
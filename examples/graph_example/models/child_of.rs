use aragog::Record;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Record)]
pub struct ChildOf {}

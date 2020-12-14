use crate::error::MigrationError;
use crate::migration_operation::MigrationOperation;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MigrationData {
    pub up: Vec<MigrationOperation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down: Option<Vec<MigrationOperation>>,
}

impl MigrationData {
    pub fn load(file_path: &str) -> Result<Self, MigrationError> {
        let mut file = File::open(file_path)?;
        let mut migration = String::new();
        file.read_to_string(&mut migration)?;
        let res: Self = serde_yaml::from_str(&migration)?;
        Ok(res)
    }
}

impl Default for MigrationData {
    fn default() -> Self {
        Self {
            up: vec![MigrationOperation::CreateCollection {
                name: String::from("MyCollection"),
            }],
            down: Some(vec![MigrationOperation::DeleteCollection {
                name: String::from("MyCollection"),
            }]),
        }
    }
}

#[cfg(test)]
mod tests {
    use arangors::graph::EdgeDefinition;
    use arangors::index::IndexSettings;

    use super::*;

    #[test]
    fn simple_migration_serializes() {
        let migration = MigrationData {
            up: vec![
                MigrationOperation::CreateCollection {
                    name: "Collection1".to_string(),
                },
                MigrationOperation::CreateCollection {
                    name: "Collection2".to_string(),
                },
                MigrationOperation::CreateIndex {
                    name: "OnNameAndEmail".to_string(),
                    collection: "Collection1".to_string(),
                    fields: vec!["name".to_string(), "email".to_string()],
                    settings: IndexSettings::Persistent {
                        unique: true,
                        sparse: false,
                        deduplicate: false,
                    },
                },
                MigrationOperation::Aql("This is a query".to_string()),
                MigrationOperation::CreateEdgeCollection {
                    name: "Edge".to_string(),
                },
                MigrationOperation::CreateGraph {
                    name: "Named Graph".to_string(),
                    edge_definitions: vec![EdgeDefinition {
                        collection: "Edge".to_string(),
                        from: vec!["Collection1".to_string()],
                        to: vec!["Collection2".to_string()],
                    }],
                    orphan_collections: None,
                    is_smart: None,
                    is_disjoint: None,
                    options: None,
                },
            ],
            down: Some(vec![
                MigrationOperation::DeleteGraph {
                    name: "Named Graph".to_string(),
                },
                MigrationOperation::DeleteEdgeCollection {
                    name: "Edge".to_string(),
                },
                MigrationOperation::DeleteIndex {
                    name: "OnNameAndEmail".to_string(),
                    collection: "Collection1".to_string(),
                },
                MigrationOperation::DeleteCollection {
                    name: "Collection2".to_string(),
                },
                MigrationOperation::DeleteCollection {
                    name: "Collection1".to_string(),
                },
            ]),
        };
        let res = serde_yaml::to_string(&migration).unwrap();
        println!("{}", res);
    }
}

use crate::error::AragogCliError;
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
    pub fn load(file_path: &str) -> Result<Self, AragogCliError> {
        let mut file = File::open(file_path)?;
        let mut migration = String::new();
        file.read_to_string(&mut migration)?;
        let res: Self = serde_yaml::from_str(&migration)?;
        Ok(res)
    }

    pub fn is_empty(&self) -> bool {
        self.up.is_empty()
            && match self.down {
                None => true,
                Some(ref arr) => arr.is_empty(),
            }
    }
}

impl Default for MigrationData {
    fn default() -> Self {
        Self {
            up: vec![MigrationOperation::CreateCollection {
                name: String::from("MyCollection"),
                wait_for_sync: None,
            }],
            down: Some(vec![MigrationOperation::DeleteCollection {
                name: String::from("MyCollection"),
            }]),
        }
    }
}

#[cfg(test)]
mod tests {
    use arangors::graph::{EdgeDefinition, GraphOptions};
    use arangors::index::IndexSettings;

    use super::*;

    #[test]
    fn migration_serializes() {
        let migration = MigrationData {
            up: vec![
                MigrationOperation::CreateCollection {
                    name: "Collection1".to_string(),
                    wait_for_sync: None,
                },
                MigrationOperation::CreateCollection {
                    name: "Collection2".to_string(),
                    wait_for_sync: Some(true),
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
                    wait_for_sync: None,
                },
                MigrationOperation::CreateGraph {
                    name: "Named Graph".to_string(),
                    edge_definitions: vec![EdgeDefinition {
                        collection: "Edge".to_string(),
                        from: vec!["Collection1".to_string()],
                        to: vec!["Collection2".to_string()],
                    }],
                    orphan_collections: None,
                    is_smart: Some(false),
                    is_disjoint: Some(true),
                    options: Some(GraphOptions {
                        smart_graph_attribute: None,
                        number_of_shards: Some(10),
                        replication_factor: None,
                        write_concern: Some(2),
                    }),
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
        serde_yaml::to_string(&migration).unwrap();
    }

    #[test]
    fn migration_deserializes() {
        let migration_yaml = "
            up:
              - create_collection:
                  name: Collection1
              - create_collection:
                  name: Collection2
                  wait_for_sync: true
              - create_index:
                  name: OnNameAndEmail
                  collection: Collection1
                  fields:
                    - name
                    - email
                  settings:
                    type: persistent
                    unique: true
                    sparse: false
                    deduplicate: false
              - aql: This is a query
              - create_edge_collection:
                  name: Edge
              - create_graph:
                  name: Named Graph
                  edge_definitions:
                    - collection: Edge
                      from:
                        - Collection1
                      to:
                        - Collection2
                  is_smart: false
                  is_disjoint: true
                  options:
                    numberOfShards: 10
                    writeConcern: 2
            down:
              - delete_graph:
                  name: Named Graph
              - delete_edge_collection:
                  name: Edge
              - delete_index:
                  name: OnNameAndEmail
                  collection: Collection1
              - delete_collection:
                  name: Collection2
              - delete_collection:
                  name: Collection1";
        serde_yaml::from_str::<MigrationData>(&migration_yaml).unwrap();
    }
}

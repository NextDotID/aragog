use arangors::client::reqwest::ReqwestClient;
use arangors::graph::{EdgeDefinition, Graph, GraphOptions};
use arangors::index::IndexSettings;
use arangors::Database;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use aragog::schema::{
    CollectionSchema, DatabaseSchema, GraphSchema, IndexSchema, SchemaDatabaseOperation,
};

use crate::error::MigrationError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationOperation {
    CreateCollection {
        name: String,
    },
    DeleteCollection {
        name: String,
    },
    CreateEdgeCollection {
        name: String,
    },
    DeleteEdgeCollection {
        name: String,
    },
    CreateIndex {
        name: String,
        collection: String,
        fields: Vec<String>,
        settings: IndexSettings,
    },
    DeleteIndex {
        name: String,
    },
    CreateGraph {
        name: String,
        edge_definitions: Vec<EdgeDefinition>,
        #[serde(skip_serializing_if = "Option::is_none")]
        orphan_collections: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_smart: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_disjoint: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        options: Option<GraphOptions>,
    },
    DeleteGraph {
        name: String,
    },
    Aql(String),
}

impl MigrationOperation {
    pub fn apply(
        self,
        schema: &mut DatabaseSchema,
        database: &Database<ReqwestClient>,
    ) -> Result<(), MigrationError> {
        match self {
            MigrationOperation::CreateCollection { name } => {
                let mut item = match schema.collection(&name) {
                    Some(_) => return Err(MigrationError::DuplicateCollection { name }),
                    None => CollectionSchema::new(&name, false),
                };
                item.apply_to_database(database, false)?;
                schema.collections.push(item);
            }
            MigrationOperation::CreateEdgeCollection { name } => {
                let mut item = match schema.collection(&name) {
                    Some(_) => return Err(MigrationError::DuplicateEdgeCollection { name }),
                    None => CollectionSchema::new(&name, true),
                };
                item.apply_to_database(database, false)?;
                schema.collections.push(item);
            }
            MigrationOperation::DeleteCollection { name } => match schema.collection_index(&name) {
                None => return Err(MigrationError::MissingCollection { name }),
                Some(index) => {
                    let item = schema.collections.remove(index);
                    item.drop(database)?;
                }
            },
            MigrationOperation::DeleteEdgeCollection { name } => {
                match schema.collection_index(&name) {
                    None => return Err(MigrationError::MissingEdgeCollection { name }),
                    Some(index) => {
                        let item = schema.collections.remove(index);
                        item.drop(database)?;
                    }
                }
            }
            MigrationOperation::CreateIndex {
                collection,
                name,
                settings,
                fields,
            } => {
                let mut item = match schema.index(&name) {
                    Some(_) => return Err(MigrationError::DuplicateIndex { name, collection }),
                    None => IndexSchema {
                        id: None,
                        name,
                        collection,
                        fields,
                        settings,
                    },
                };
                item.apply_to_database(database, false)?;
                schema.indexes.push(item);
            }
            MigrationOperation::DeleteIndex { name } => match schema.index_index(&name) {
                None => return Err(MigrationError::MissingIndex { name }),
                Some(index) => {
                    let item = schema.indexes.remove(index);
                    item.drop(database)?;
                }
            },
            MigrationOperation::CreateGraph {
                name,
                edge_definitions,
                orphan_collections,
                is_smart,
                is_disjoint,
                options,
            } => {
                let mut item = match schema.graph(&name) {
                    Some(_) => return Err(MigrationError::DuplicateGraph { name }),
                    None => GraphSchema(Graph {
                        name,
                        edge_definitions,
                        orphan_collections: orphan_collections.unwrap_or(Vec::new()),
                        is_smart,
                        is_disjoint,
                        options,
                    }),
                };
                item.apply_to_database(database, false)?;
                schema.graphs.push(item);
            }
            MigrationOperation::DeleteGraph { name } => match schema.graph_index(&name) {
                None => return Err(MigrationError::MissingGraph { name }),
                Some(graph) => {
                    let item = schema.graphs.remove(graph);
                    item.drop(database)?;
                }
            },
            MigrationOperation::Aql(aql) => {
                log::debug!("Executing AQL {} ...", &aql);
                database.aql_str::<Value>(aql.as_str())?;
            }
        };
        Ok(())
    }
}

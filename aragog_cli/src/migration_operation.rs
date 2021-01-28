use arangors::graph::{EdgeDefinition, Graph, GraphOptions};
use arangors::index::IndexSettings;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use aragog::schema::{CollectionSchema, GraphSchema, IndexSchema, SchemaDatabaseOperation};

use crate::error::MigrationError;
use crate::log;
use crate::log_level::LogLevel;
use crate::VersionedDatabase;

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
        collection: String,
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
    pub fn apply(self, db: &mut VersionedDatabase) -> Result<(), MigrationError> {
        match self {
            MigrationOperation::CreateCollection { name } => {
                log("Executing create_collection operation", LogLevel::Verbose);
                let item = match db.schema.collection(&name) {
                    Some(_) => return Err(MigrationError::DuplicateCollection { name }),
                    None => CollectionSchema::new(&name, false),
                };
                item.apply_to_database(db, false)?;
                db.schema.collections.push(item);
            }
            MigrationOperation::CreateEdgeCollection { name } => {
                log(
                    "Executing create_edge_collection operation",
                    LogLevel::Verbose,
                );
                let item = match db.schema.collection(&name) {
                    Some(_) => return Err(MigrationError::DuplicateEdgeCollection { name }),
                    None => CollectionSchema::new(&name, true),
                };
                item.apply_to_database(db, false)?;
                db.schema.collections.push(item);
            }
            MigrationOperation::DeleteCollection { name } => {
                log("Executing delete_collection operation", LogLevel::Verbose);
                match db.schema.collection_index(&name) {
                    None => return Err(MigrationError::MissingCollection { name }),
                    Some(index) => {
                        let item = db.schema.collections.remove(index);
                        item.drop(db)?;
                    }
                }
            }
            MigrationOperation::DeleteEdgeCollection { name } => {
                log(
                    "Executing delete_edge_collection operation",
                    LogLevel::Verbose,
                );
                match db.schema.collection_index(&name) {
                    None => return Err(MigrationError::MissingEdgeCollection { name }),
                    Some(index) => {
                        let item = db.schema.collections.remove(index);
                        item.drop(db)?;
                    }
                }
            }
            MigrationOperation::CreateIndex {
                collection,
                name,
                settings,
                fields,
            } => {
                log("Executing create_index operation", LogLevel::Verbose);
                let item = match db.schema.index(&collection, &name) {
                    Some(_) => return Err(MigrationError::DuplicateIndex { name, collection }),
                    None => IndexSchema {
                        name,
                        collection,
                        fields,
                        settings,
                    },
                };
                item.apply_to_database(db, false)?;
                db.schema.indexes.push(item);
            }
            MigrationOperation::DeleteIndex { name, collection } => {
                log("Executing delete_index operation", LogLevel::Verbose);
                match db.schema.index_index(&collection, &name) {
                    None => return Err(MigrationError::MissingIndex { collection, name }),
                    Some(index) => {
                        let item = db.schema.indexes.remove(index);
                        item.drop(db)?;
                    }
                }
            }
            MigrationOperation::CreateGraph {
                name,
                edge_definitions,
                orphan_collections,
                is_smart,
                is_disjoint,
                options,
            } => {
                log("Executing create_graph operation", LogLevel::Verbose);
                let item = match db.schema.graph(&name) {
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
                item.apply_to_database(db, false)?;
                db.schema.graphs.push(item);
            }
            MigrationOperation::DeleteGraph { name } => {
                log("Executing delete_graph operation", LogLevel::Verbose);
                match db.schema.graph_index(&name) {
                    None => return Err(MigrationError::MissingGraph { name }),
                    Some(graph) => {
                        let item = db.schema.graphs.remove(graph);
                        item.drop(db)?;
                    }
                }
            }
            MigrationOperation::Aql(aql) => {
                log("Executing aql operation", LogLevel::Verbose);
                let res: Vec<Value> = db.aql_str(aql.as_str())?;
                log(format!("{:?}", res), LogLevel::Verbose);
            }
        };
        Ok(())
    }
}

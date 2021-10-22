use crate::config::Config;
use crate::error::AragogCliError;
use crate::migration::Migration;
use crate::migration_operation::MigrationOperation;
use crate::versioned_database::VersionedDatabase;
use arangors_lite::collection::response::Info;
use arangors_lite::collection::CollectionType;
use arangors_lite::index::{Index, IndexSettings};

pub fn discover_migration(
    db: &mut VersionedDatabase,
    config: &Config,
) -> Result<Migration, AragogCliError> {
    let collections: Vec<Info> = db.accessible_collections()?;
    let collections_to_create: Vec<(String, bool)> = collections
        .into_iter()
        .filter(|i| {
            db.schema.collection_index(&i.name).is_none()
                && !i.is_system
                && i.name != config.schema_collection_name
        })
        .map(|i| (i.name, matches!(i.collection_type, CollectionType::Edge)))
        .collect();
    let mut migration = Migration::new("discover_migration", &config.schema_path, false)?;
    let mut operations_up = Vec::new();
    let mut operations_down = Vec::new();

    for (name, is_edge) in collections_to_create.iter() {
        if *is_edge {
            operations_up.push(MigrationOperation::CreateEdgeCollection {
                name: name.clone(),
                wait_for_sync: None,
            });
            operations_down.push(MigrationOperation::DeleteEdgeCollection { name: name.clone() });
        } else {
            operations_up.push(MigrationOperation::CreateCollection {
                name: name.clone(),
                wait_for_sync: None,
            });
            operations_down.push(MigrationOperation::DeleteCollection { name: name.clone() });
        }
    }
    for (name, _) in collections_to_create.iter() {
        for index in db.indexes(name)?.indexes.into_iter() {
            let index: Index = index;
            if let IndexSettings::Primary { .. } | IndexSettings::Edge { .. } = index.settings {
                continue;
            }
            operations_up.push(MigrationOperation::CreateIndex {
                name: index.name.clone(),
                collection: name.clone(),
                fields: index.fields,
                settings: index.settings,
            });
            operations_down.insert(
                0,
                MigrationOperation::DeleteIndex {
                    name: index.name,
                    collection: name.clone(),
                },
            )
        }
    }
    for graph in db.graphs()?.graphs.into_iter() {
        if db.schema.graph_index(&graph.name).is_none() {
            operations_up.push(MigrationOperation::CreateGraph {
                name: graph.name.clone(),
                edge_definitions: graph.edge_definitions,
                orphan_collections: Some(graph.orphan_collections),
                is_smart: graph.is_smart,
                is_disjoint: graph.is_disjoint,
                options: graph.options,
            });
            operations_down.push(MigrationOperation::DeleteGraph { name: graph.name })
        }
    }
    migration.data.up = operations_up;
    migration.data.down = Some(operations_down);
    migration.save()?;
    Ok(migration)
}

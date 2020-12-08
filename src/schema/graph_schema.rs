use crate::schema::SchemaDatabaseOperation;
use arangors::client::reqwest::ReqwestClient;
use arangors::graph::Graph;
use arangors::{ClientError, Database};
use serde::{Deserialize, Serialize};

/// Aragog schema representation of an ArangoDB Named Graph.
/// This struct is meant to load/generate the schema file.
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphSchema(pub Graph);

impl Into<Graph> for GraphSchema {
    fn into(self) -> Graph {
        self.0
    }
}

#[maybe_async::maybe_async]
impl SchemaDatabaseOperation for GraphSchema {
    type PoolType = Graph;

    async fn apply_to_database(
        &mut self,
        database: &Database<ReqwestClient>,
        silent: bool,
    ) -> Result<(), ClientError> {
        log::debug!("Creating Graph {}", &self.0.name);
        let duplicate = serde_json::to_string(self).unwrap();
        let duplicate: Self = serde_json::from_str(&duplicate).unwrap();
        let graph = duplicate.into();
        Self::handle_error(database.create_graph(graph, true).await, silent)?;
        Ok(())
    }

    async fn drop(&self, database: &Database<ReqwestClient>) -> Result<(), ClientError> {
        log::debug!("Deleting Graph {}", &self.0.name);
        database.drop_graph(&self.0.name, false).await?;
        Ok(())
    }

    async fn get(&self, database: &Database<ReqwestClient>) -> Result<Self::PoolType, ClientError> {
        database.graph(&self.0.name).await
    }
}

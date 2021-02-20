use crate::model::RelationInstance;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use indradb::EdgeKey;

#[derive(Debug)]
pub struct RelationInstanceCreationError;

#[derive(Debug)]
pub struct RelationInstanceImportError;

#[async_trait]
pub trait RelationInstanceManager: Send + Sync {
    /// Returns true, if an relation instance exists with the given key.
    fn has(&self, edge_key: EdgeKey) -> bool;

    /// Returns the relation instance with the given key or None.
    fn get(&self, edge_key: EdgeKey) -> Option<RelationInstance>;

    fn create(&self, edge_key: EdgeKey, properties: HashMap<String, Value>)
        -> Result<EdgeKey, RelationInstanceCreationError>;

    fn create_from_instance(
        &self,
        relation_instance: RelationInstance
    ) -> Result<EdgeKey, RelationInstanceCreationError>;

    // TODO: return result
    fn commit(&self, relation_instance: RelationInstance);

    fn delete(&self, edge_key: EdgeKey);

    fn import(&self, path: String) -> Result<RelationInstance, RelationInstanceImportError>;

    // TODO: return result
    // TODO: egde_key ?
    fn export(&self, edge_key: EdgeKey, path: String);
}

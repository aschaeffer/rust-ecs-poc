use async_trait::async_trait;
use indradb::{Edge, EdgeProperties, EdgeKey};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RelationEdgeKeyInvalid;

#[derive(Debug)]
pub struct RelationEdgeCreationError;

#[async_trait]
pub trait RelationEdgeManager: Send + Sync {

    // /// Returns the edge key.
    // // TODO: move out of RelationEdgeManager and allow static access (EdgeKeyBuilder ?)
    // fn get_key(&self, outbound_id: Uuid, type_name: String, inbound_id: Uuid) -> Result<EdgeKey, RelationEdgeKeyInvalid>;

    // outbound_id: Uuid, type_name: String, inbound_id: Uuid

    /// Returns true, if an relation instance edge exists with the given UUID.
    fn has(&self, edge_key: EdgeKey) -> bool;

    /// Returns the edge by UUID.
    fn get(&self, edge_key: EdgeKey) -> Option<Edge>;

    /// Returns the edge properties by UUID. The result contains
    /// the edge and the type.
    fn get_properties(&self, edge_key: EdgeKey) -> Option<EdgeProperties>;

    /// Creates a new edge with the given edge key and the given properties.
    fn create(&self, edge_key: EdgeKey, properties: HashMap<String, Value>)
        -> Result<EdgeKey, RelationEdgeCreationError>;

    // TODO: return result RelationEdgeUpdateError
    // TODO: rename commit -> "update" or "save"
    fn commit(&self, edge_key: EdgeKey, properties: HashMap<String, Value>);

    /// Deletes the edge with the given edge key.
    // TODO: return result RelationEdgeDeletionError
    fn delete(&self, edge_key: EdgeKey);
}

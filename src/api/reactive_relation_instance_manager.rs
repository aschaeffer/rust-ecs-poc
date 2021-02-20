use crate::model::{RelationInstance, ReactiveRelationInstance};
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use indradb::EdgeKey;

#[derive(Debug)]
pub struct ReactiveRelationInstanceCreationError;

#[derive(Debug)]
pub struct ReactiveRelationInstanceImportError;

#[async_trait]
pub trait ReactiveRelationInstanceManager: Send + Sync {
    /// Returns true, if an relation of the given type exists which starts at the given outbound entity and
    /// ends at the given inbound entity.
    fn has(&self, edge_key: EdgeKey) -> bool;

    /// Returns the ReactiveRelationInstance with the given type_name, which starts at the given
    /// outbound entity and ends at the given inbound entity.
    fn get(&self, edge_key: EdgeKey) -> Option<Arc<ReactiveRelationInstance>>;

    fn create(&self, edge_key: EdgeKey, properties: HashMap<String, Value>)
        -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError>;

    fn create_reactive_instance(&self, relation_instance: RelationInstance)
        -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError>;

    fn register_reactive_instance(&self, reactive_relation_instance: Arc<ReactiveRelationInstance>);

    // TODO: fn commit(&self, relation_instance: RelationInstance);
    // TODO: return result
    fn commit(&self, edge_key: EdgeKey);

    fn delete(&self, edge_key: EdgeKey);

    fn unregister_reactive_instance(&self, edge_key: EdgeKey);

    fn import(&self, path: String)
        -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceImportError>;

    // TODO: return result
    fn export(&self, edge_key: EdgeKey, path: String);
}

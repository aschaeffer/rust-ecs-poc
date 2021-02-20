use crate::model::{EntityInstance, ReactiveEntityInstance};
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;
use std::sync::Arc;

#[derive(Debug)]
pub struct ReactiveEntityInstanceCreationError;

#[derive(Debug)]
pub struct ReactiveEntityInstanceImportError;

#[async_trait]
pub trait ReactiveEntityInstanceManager: Send + Sync {
    /// Returns true, if an entity instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the reactive entity instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<Arc<ReactiveEntityInstance>>;

    fn create(&self, type_name: String, properties: HashMap<String, Value>)
        -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError>;

    fn create_with_id(&self, type_name: String, id: Uuid, properties: HashMap<String, Value>)
        -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError>;

    fn create_reactive_instance(&self, entity_instance: EntityInstance)
        -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError>;

    fn register_reactive_instance(&self, reactive_entity_instance: Arc<ReactiveEntityInstance>);

    // TODO: return result
    fn commit(&self, id: Uuid);

    fn delete(&self, id: Uuid);

    fn unregister_reactive_instance(&self, id: Uuid);

    fn import(&self, path: String)
        -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceImportError>;

    // TODO: return result
    fn export(&self, id: Uuid, path: String);
}

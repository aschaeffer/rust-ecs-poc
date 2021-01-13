use crate::model::EntityInstance;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct EntityInstanceCreationError;

#[derive(Debug)]
pub struct EntityInstanceImportError;

#[async_trait]
pub trait EntityInstanceManager: Send + Sync {
    /// Returns true, if an entity instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the entity instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<EntityInstance>;

    fn create(
        &self,
        type_name: String,
        properties: HashMap<String, Value>,
    ) -> Result<Uuid, EntityInstanceCreationError>;

    fn create_with_id(
        &self,
        type_name: String,
        id: Uuid,
        properties: HashMap<String, Value>,
    ) -> Result<Uuid, EntityInstanceCreationError>;

    // fn create_by_type(&self, type_name: String);

    fn delete(&self, id: Uuid);

    fn import(&self, path: String) -> Result<Uuid, EntityInstanceImportError>;

    fn export(&self, id: Uuid, path: String);
}

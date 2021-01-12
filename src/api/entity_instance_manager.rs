use crate::model::EntityInstance;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;
use indradb::{Vertex, VertexProperties};

#[async_trait]
pub trait EntityInstanceManager: Send + Sync {

    /// Returns true, if an entity instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the entity instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<EntityInstance>;

    fn create(&self, type_name: String, properties: HashMap<String, Value>);

    fn delete(&self, id: Uuid);

    fn import(&self, path: String) -> Option<Uuid>;

    fn export(&self, id: Uuid, path: String);

}

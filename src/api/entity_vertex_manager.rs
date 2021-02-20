use async_trait::async_trait;
use indradb::{Vertex, VertexProperties};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct EntityVertexCreationError;

#[async_trait]
pub trait EntityVertexManager: Send + Sync {
    /// Returns true, if an entity instance vertex exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the vertex by UUID.
    fn get(&self, id: Uuid) -> Option<Vertex>;

    /// Returns the vertex properties by UUID. The result contains
    /// the vertex and the type.
    fn get_properties(&self, id: Uuid) -> Option<VertexProperties>;

    /// Creates a new vertex with the given type and the given properties.
    fn create(
        &self,
        type_name: String,
        properties: HashMap<String, Value>,
    ) -> Result<Uuid, EntityVertexCreationError>;

    /// Creates a new vertex with the given id, the given type and the given properties.
    /// This is useful for importing an entity instance with the fixed id.
    fn create_with_id(
        &self,
        type_name: String,
        id: Uuid,
        properties: HashMap<String, Value>,
    ) -> Result<Uuid, EntityVertexCreationError>;

    // TODO: return result
    fn commit(&self, id: Uuid, properties: HashMap<String, Value>);

    /// Deletes the vertex with the given id.
    // TODO: return result
    fn delete(&self, id: Uuid);
}

use crate::model::{EntityType,PropertyType};
use async_trait::async_trait;
use crate::api::Lifecycle;

#[derive(Debug)]
pub struct EntityTypeImportError;

#[async_trait]
pub trait EntityTypeManager: Send + Sync + Lifecycle {
    fn register(&self, entity_type: EntityType);
    fn load_static_entity_types(&self);
    fn get_entity_types(&self) -> Vec<EntityType>;

    fn has(&self, name: String) -> bool;
    fn get(&self, name: String) -> Option<EntityType>;

    fn create(&self, name: String, components: Vec<String>, behaviours: Vec<String>, properties: Vec<PropertyType>);
    fn delete(&self, name: String);

    fn import(&self, path: String) -> Result<EntityType, EntityTypeImportError>;
    fn export(&self, name: String, path: String);
}

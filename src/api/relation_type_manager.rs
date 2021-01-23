use crate::model::{RelationType,PropertyType};
use async_trait::async_trait;

#[derive(Debug)]
pub struct RelationTypeImportError;

#[async_trait]
pub trait RelationTypeManager: Send + Sync {
    fn register(&self, entity_type: RelationType);
    fn load_static_relation_types(&self);
    fn get_relation_types(&self) -> Vec<RelationType>;
    fn list_relation_types(&self);

    fn has(&self, name: String) -> bool;
    fn get(&self, name: String) -> Option<RelationType>;

    fn create(&self, name: String, outbound_type: String, inbound_type: String, components: Vec<String>, properties: Vec<PropertyType>);
    fn delete(&self, name: String);

    fn import(&self, path: String) -> Result<RelationType, RelationTypeImportError>;
    fn export(&self, name: String, path: String);
}

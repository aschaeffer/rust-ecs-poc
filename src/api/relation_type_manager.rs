use crate::model::{RelationType,PropertyType};
use async_trait::async_trait;
use crate::api::Lifecycle;

#[derive(Debug)]
pub struct RelationTypeImportError;

#[async_trait]
pub trait RelationTypeManager: Send + Sync + Lifecycle {
    // TODO: Result
    fn register(&self, entity_type: RelationType);
    fn load_static_relation_types(&self);
    fn get_relation_types(&self) -> Vec<RelationType>;

    fn has(&self, type_name: String) -> bool;
    fn get(&self, type_name: String) -> Option<RelationType>;
    fn get_starts_with(&self, type_name_starts_with: String) -> Option<RelationType>;

    // TODO: Result
    fn create(&self, outbound_type: String, type_name: String, inbound_type: String, components: Vec<String>, behaviours: Vec<String>, properties: Vec<PropertyType>);
    fn delete(&self, type_name: String);

    fn import(&self, path: String) -> Result<RelationType, RelationTypeImportError>;
    fn export(&self, type_name: String, path: String);
}

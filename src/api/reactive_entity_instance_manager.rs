// use crate::model::{EntityInstance, EntityType, PropertyType};
use async_trait::async_trait;
// use uuid::Uuid;
use crate::model::EntityInstance;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[async_trait]
pub trait ReactiveEntityInstanceManager: Send + Sync {
    fn has(&self, id: Uuid) -> bool;
    fn get(&self, name: String) -> Option<EntityInstance>;

    fn create(&self, type_name: String, properties: HashMap<String, Value>);
    fn delete(&self, id: Uuid);

    fn import(&self, path: String);
    fn export(&self, name: String, path: String);
}

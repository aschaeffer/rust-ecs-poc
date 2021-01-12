use crate::api::{
    EntityInstanceManager,
    EntityInstanceVertexManager,
    ComponentManager,
    EntityTypeManager
};
use crate::model::EntityInstance;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use uuid::Uuid;
use waiter_di::*;

// This service operates on the graph database.

#[component]
pub struct EntityInstanceManagerImpl {

    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    entity_instance_vertex_manager: Wrc<dyn EntityInstanceVertexManager>,

}

#[async_trait]
#[provides]
impl EntityInstanceManager for EntityInstanceManagerImpl {

    fn has(&self, id: Uuid) -> bool {
        unimplemented!()
    }

    fn get(&self, id: Uuid) -> Option<EntityInstance> {
        unimplemented!()
    }

    fn create(&self, type_name: String, properties: HashMap<String, Value, RandomState>) {
        unimplemented!()
    }

    fn delete(&self, id: Uuid) {
        unimplemented!()
    }

    fn import(&self, path: String) -> Option<Uuid> {
        unimplemented!()
    }

    fn export(&self, id: Uuid, path: String) {
        unimplemented!()
    }

}

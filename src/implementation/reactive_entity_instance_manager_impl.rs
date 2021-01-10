use crate::api::{EntityTypeManager, ComponentManager, ReactiveEntityInstanceManager, ReactivePropertyInstanceManager};
use crate::api::GraphDatabase;
use async_trait::async_trait;
use waiter_di::*;
use std::sync::RwLock;
use uuid::Uuid;
use std::collections::HashMap;
use serde_json::Value;
use std::collections::hash_map::RandomState;

#[wrapper]
pub struct ReactiveEntityInstances(RwLock<std::collections::HashMap<Uuid, crate::model::ReactiveEntityInstance<'_>>>);

#[provides]
fn create_external_type_dependency() -> ReactiveEntityInstances {
    ReactiveEntityInstances(RwLock::new(std::collections::HashMap::new()))
}

#[component]
pub struct EntityInstanceManagerImpl {

    graph_database: Wrc<dyn GraphDatabase>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    property_instance_manager: Wrc<dny ReactivePropertyInstanceManager>,

    entity_instances: ReactiveEntityInstances,

}

#[async_trait]
#[provides]
impl ReactiveEntityInstanceManager for EntityInstanceManagerImpl {

    fn create(&self, type_name: String, properties: HashMap<String, Value>) {
        unimplemented!()
    }

    fn create_vertex(&self, type_name: String, properties: HashMap<String, Value, RandomState>) {
        unimplemented!()
    }

}

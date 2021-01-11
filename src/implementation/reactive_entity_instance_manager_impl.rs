// use crate::api::{
//     EntityTypeManager,
//     ComponentManager,
//     GraphDatabase,
//     ReactiveEntityInstanceManager,
//     // ReactivePropertyInstanceManager,
//     EntityInstanceManager
// };
// use crate::model::{
//     EntityInstance
// };
// use async_trait::async_trait;
// use waiter_di::*;
// use std::sync::RwLock;
// use uuid::Uuid;
// use std::collections::HashMap;
// use serde_json::Value;
// use std::collections::hash_map::RandomState;
//
// #[wrapper]
// pub struct ReactiveEntityInstances<'b>(RwLock<std::collections::HashMap<Uuid, crate::model::ReactiveEntityInstance<'b>>>);
//
// #[provides]
// fn create_external_type_dependency<'b>() -> ReactiveEntityInstances<'b> {
//     ReactiveEntityInstances(RwLock::new(std::collections::HashMap::new()))
// }
//
// #[component]
// pub struct ReactiveEntityInstanceManagerImpl<'b> {
//
//     graph_database: Wrc<dyn GraphDatabase>,
//
//     component_manager: Wrc<dyn ComponentManager>,
//
//     entity_type_manager: Wrc<dyn EntityTypeManager>,
//
//     entity_instance_manager: Wrc<dyn EntityInstanceManager>,
//
//     // property_instance_manager: Wrc<dny ReactivePropertyInstanceManager>,
//
//     // They live here
//     entity_instances: ReactiveEntityInstances<'b>,
//
// }
//
// #[async_trait]
// #[provides]
// impl<'b> ReactiveEntityInstanceManager for ReactiveEntityInstanceManagerImpl<'b> {
//
//     fn has(&self, id: Uuid) -> bool {
//         unimplemented!()
//     }
//
//     fn get(&self, name: String) -> Option<EntityInstance> {
//         unimplemented!()
//     }
//
//     fn create(&self, type_name: String, properties: HashMap<String, Value, RandomState>) {
//         unimplemented!()
//     }
//
//     fn delete(&self, id: Uuid) {
//         unimplemented!()
//     }
//
//     fn import(&self, path: String) {
//         unimplemented!()
//     }
//
//     fn export(&self, name: String, path: String) {
//         unimplemented!()
//     }
// }

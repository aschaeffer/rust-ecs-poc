use crate::api::{ReactiveEntityInstanceManager, EntityInstanceManager, ReactiveEntityInstanceImportError, ReactiveEntityInstanceCreationError, EntityBehaviourManager};
use crate::model::{EntityInstance, ReactiveEntityInstance};
use async_trait::async_trait;
use waiter_di::*;
use std::sync::Arc;
use uuid::Uuid;
use std::collections::HashMap;
use serde_json::Value;

#[wrapper]
pub struct ReactiveEntityInstances(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<crate::model::ReactiveEntityInstance>>>);

#[provides]
fn create_external_type_dependency() -> ReactiveEntityInstances {
    ReactiveEntityInstances(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[component]
pub struct ReactiveEntityInstanceManagerImpl {
    entity_instance_manager: Wrc<dyn EntityInstanceManager>,

    entity_behaviour_manager: Wrc<dyn EntityBehaviourManager>,

    reactive_entity_instances: ReactiveEntityInstances,
}

#[async_trait]
#[provides]
impl ReactiveEntityInstanceManager for ReactiveEntityInstanceManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.entity_instance_manager.has(id) && self.reactive_entity_instances.0.read().unwrap().contains_key(&id)
    }

    fn get(&self, id: Uuid) -> Option<Arc<ReactiveEntityInstance>> {
        let reader = self.reactive_entity_instances.0.read().unwrap();
        let instance = reader.get(&id);
        if instance.is_some() {
            return Some(instance.unwrap().clone());
        }
        None
    }

    fn create(&self, type_name: String, properties: HashMap<String, Value>) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError> {
        let result = self.entity_instance_manager.create(type_name, properties);
        if result.is_ok() {
            let entity_instance = self.entity_instance_manager.get(result.unwrap());
            if entity_instance.is_some() {
                return self.create_reactive_instance(entity_instance.unwrap());
            }
        }
        Err(ReactiveEntityInstanceCreationError.into())
    }

    fn create_with_id(&self, type_name: String, id: Uuid, properties: HashMap<String, Value>) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError> {
        if !self.has(id) {
            let entity_instance = self.entity_instance_manager.get(id);
            match entity_instance {
                Some(entity_instance) => {
                    // TODO: update properties first?
                    return self.create_reactive_instance(entity_instance);
                },
                None => {
                    let result = self.entity_instance_manager.create_with_id(type_name, id, properties);
                    if result.is_ok() {
                        let entity_instance = self.entity_instance_manager.get(id);
                        if entity_instance.is_some() {
                            return self.create_reactive_instance(entity_instance.unwrap());
                        }
                    }
                }
            }
        }
        Err(ReactiveEntityInstanceCreationError.into())
    }

    fn create_reactive_instance(&self, entity_instance: EntityInstance) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError> {
        let reactive_entity_instance = Arc::new(ReactiveEntityInstance::from(entity_instance));
        self.register_reactive_instance(reactive_entity_instance.clone());
        Ok (reactive_entity_instance)
    }

    fn register_reactive_instance(&self, reactive_entity_instance: Arc<ReactiveEntityInstance>) {
        self.reactive_entity_instances.0.write().unwrap().insert(reactive_entity_instance.id, reactive_entity_instance.clone());
        self.entity_behaviour_manager.add_behaviours(reactive_entity_instance.clone());
    }

    fn commit(&self, id: Uuid) {
        let reactive_entity_instance = self.get(id);
        if reactive_entity_instance.is_some() {
            self.entity_instance_manager.commit(reactive_entity_instance.unwrap().into());
        }
    }

    fn delete(&self, id: Uuid) {
        if self.has(id) {
            self.unregister_reactive_instance(id);
            // self.entity_behaviour_manager.remove_behaviours_by_id(id);
            // let id = &id;
            // self.reactive_entity_instances.0.write().unwrap().remove(id);
        }
        self.entity_instance_manager.delete(id);
    }

    fn unregister_reactive_instance(&self, id: Uuid) {
        self.entity_behaviour_manager.remove_behaviours_by_id(id);
        let id = &id;
        self.reactive_entity_instances.0.write().unwrap().remove(id);
    }

    fn import(&self, path: String) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceImportError> {
        let result = self.entity_instance_manager.import(path.clone());
        if result.is_ok() {
            let entity_instance = self.entity_instance_manager.get(result.unwrap());
            if entity_instance.is_some() {
                let result = self.create_reactive_instance(entity_instance.unwrap());
                if result.is_ok() {
                    let reactive_entity_instance= result.unwrap();
                    return Ok(reactive_entity_instance);
                }
            }
        }
        Err(ReactiveEntityInstanceImportError.into())
    }

    fn export(&self, id: Uuid, path: String) {
        if self.has(id) {
            self.commit(id);
            self.entity_instance_manager.export(id, path);
        }
    }
}

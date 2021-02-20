use crate::api::{ReactiveRelationInstanceManager, RelationInstanceManager, ReactiveRelationInstanceImportError, ReactiveRelationInstanceCreationError, ReactiveEntityInstanceManager, RelationBehaviourManager};
use crate::model::{RelationInstance, ReactiveRelationInstance};
use async_trait::async_trait;
use waiter_di::*;
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;
use indradb::EdgeKey;

#[wrapper]
pub struct ReactiveRelationInstances(std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<crate::model::ReactiveRelationInstance>>>);

#[provides]
fn create_external_type_dependency() -> ReactiveRelationInstances {
    ReactiveRelationInstances(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[component]
pub struct ReactiveRelationInstanceManagerImpl {
    relation_instance_manager: Wrc<dyn RelationInstanceManager>,

    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    relation_behaviour_manager: Wrc<dyn RelationBehaviourManager>,

    reactive_relation_instances: ReactiveRelationInstances,
}

#[async_trait]
#[provides]
impl ReactiveRelationInstanceManager for ReactiveRelationInstanceManagerImpl {
    fn has(&self, edge_key: EdgeKey) -> bool {
        self.relation_instance_manager.has(edge_key.clone())
            && self.reactive_relation_instances.0.read().unwrap().contains_key(&edge_key.clone())
    }

    fn get(&self, edge_key: EdgeKey) -> Option<Arc<ReactiveRelationInstance>> {
        let reader = self.reactive_relation_instances.0.read().unwrap();
        let instance = reader.get(&edge_key.clone());
        if instance.is_some() {
            return Some(instance.unwrap().clone());
        }
        None
    }

    fn create(&self, edge_key: EdgeKey, properties: HashMap<String, Value>)
        -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError>
    {
        let result = self.relation_instance_manager.create(edge_key.clone(), properties);
        if result.is_ok() {
            let edge_key = result.unwrap();
            let relation_instance = self.relation_instance_manager.get(edge_key.clone());
            if relation_instance.is_some() {
                return self.create_reactive_instance(relation_instance.unwrap());
            }
        }
        Err(ReactiveRelationInstanceCreationError.into())
    }

    fn create_reactive_instance(&self, relation_instance: RelationInstance)
        -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError>
    {
        let edge_key = relation_instance.get_key();
        if edge_key.is_some() {
            let outbound = self.reactive_entity_instance_manager.get(relation_instance.outbound_id);
            let inbound = self.reactive_entity_instance_manager.get(relation_instance.inbound_id);
            if outbound.is_some() && inbound.is_some() {
                let outbound = outbound.unwrap();
                let inbound = inbound.unwrap();
                let reactive_relation_instance = Arc::new(ReactiveRelationInstance::from_instance(outbound.clone(), inbound.clone(), relation_instance));
                self.register_reactive_instance(reactive_relation_instance.clone());
                return Ok(reactive_relation_instance.clone());
            }
        }
        Err(ReactiveRelationInstanceCreationError.into())
    }

    fn register_reactive_instance(&self, reactive_relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = reactive_relation_instance.get_key();
        if edge_key.is_some() {
            self.reactive_relation_instances.0.write().unwrap().insert(edge_key.unwrap(), reactive_relation_instance.clone());
            self.relation_behaviour_manager.add_behaviours(reactive_relation_instance.clone());
        }
    }

    fn commit(&self, edge_key: EdgeKey) {
        let reactive_relation_instance = self.get(edge_key.clone());
        if reactive_relation_instance.is_some() {
            self.relation_instance_manager.commit(reactive_relation_instance.unwrap().into());
        }
    }

    fn delete(&self, edge_key: EdgeKey) {
        if self.has(edge_key.clone()) {
            self.unregister_reactive_instance(edge_key.clone());
        }
        self.relation_instance_manager.delete(edge_key.clone());
    }

    fn unregister_reactive_instance(&self, edge_key: EdgeKey) {
        self.relation_behaviour_manager.remove_behaviours_by_key(edge_key.clone());
        self.reactive_relation_instances.0.write().unwrap().remove(&edge_key.clone());
    }

    fn import(&self, path: String) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceImportError> {
        let result = self.relation_instance_manager.import(path.clone());
        if result.is_ok() {
            let relation_instance = result.unwrap();
            let result = self.create_reactive_instance(relation_instance);
            if result.is_ok() {
                return Ok(result.unwrap());
            }
        }
        Err(ReactiveRelationInstanceImportError.into())
    }

    fn export(&self, edge_key: EdgeKey, path: String) {
        if self.has(edge_key.clone()) {
            self.commit(edge_key.clone());
            self.relation_instance_manager.export(edge_key.clone(), path);
        }
    }
}

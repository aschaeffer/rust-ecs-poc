use crate::api::{ReactiveFlowManager, FlowManager, ReactiveFlowImportError, ReactiveFlowCreationError, ReactiveRelationInstanceManager, ReactiveEntityInstanceManager};
use crate::model::{Flow, ReactiveFlow};
use async_trait::async_trait;
use waiter_di::*;
use std::sync::Arc;
use uuid::Uuid;
use std::convert::TryFrom;

#[wrapper]
pub struct ReactiveFlows(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<crate::model::ReactiveFlow>>>);

#[provides]
fn create_external_type_dependency() -> ReactiveFlows {
    ReactiveFlows(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[component]
pub struct ReactiveFlowManagerImpl {
    flow_manager: Wrc<dyn FlowManager>,

    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    reactive_relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,

    reactive_flows: ReactiveFlows,
}

#[async_trait]
#[provides]
impl ReactiveFlowManager for ReactiveFlowManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.reactive_flows.0.read().unwrap().contains_key(&id)
    }

    fn get(&self, id: Uuid) -> Option<Arc<ReactiveFlow>> {
        let reader = self.reactive_flows.0.read().unwrap();
        let instance = reader.get(&id);
        if instance.is_some() {
            return Some(instance.unwrap().clone());
        }
        None
    }

    // fn create(&self, type_name: String, properties: HashMap<String, Value, RandomState>) -> Result<Arc<ReactiveFlow>, ReactiveFlowCreationError> {
    fn create(&self, flow: Flow) -> Result<Arc<ReactiveFlow>, ReactiveFlowCreationError> {
        let reactive_flow = ReactiveFlow::try_from(flow);
        if reactive_flow.is_ok() {
            let reactive_flow = Arc::new(reactive_flow.unwrap());
            self.register(reactive_flow.clone());
            // for (_, entity_instance) in reactive_flow.entity_instances.read().unwrap().iter() {
            //     self.reactive_entity_instance_manager.register_reactive_instance(entity_instance.clone());
            // }
            // for (_, relation_instance) in reactive_flow.relation_instances.read().unwrap().iter() {
            //     self.reactive_relation_instance_manager.register_reactive_instance(relation_instance.clone());
            // }
            // self.reactive_flows.0.write().unwrap().insert(reactive_flow.id, reactive_flow.clone());
            return Ok(reactive_flow.clone());
        }
        Err(ReactiveFlowCreationError.into())
    }

    fn register(&self, reactive_flow: Arc<ReactiveFlow>) {
        if !self.has(reactive_flow.id) {
            for (_, entity_instance) in reactive_flow.entity_instances.read().unwrap().iter() {
                self.reactive_entity_instance_manager.register_reactive_instance(entity_instance.clone());
            }
            for (_, relation_instance) in reactive_flow.relation_instances.read().unwrap().iter() {
                self.reactive_relation_instance_manager.register_reactive_instance(relation_instance.clone());
            }
            self.reactive_flows.0.write().unwrap().insert(reactive_flow.id, reactive_flow.clone());
        }
    }

    // TODO: how to detect if the flow has removed an entity? => remove behaviour
    // TODO: how to detect if the flow has removed an relation? => remove behaviour
    fn commit(&self, id: Uuid) {
        let reactive_flow = self.get(id);
        if reactive_flow.is_some() {
            let reactive_flow = reactive_flow.unwrap();

            // Unregister removed relations
            for edge_key in reactive_flow.relations_removed.read().unwrap().iter() {
                self.reactive_relation_instance_manager.unregister_reactive_instance(edge_key.clone());
            }
            reactive_flow.relations_removed.write().unwrap().clear();

            // Unregister removed entities
            for id in reactive_flow.entities_removed.read().unwrap().iter() {
                self.reactive_entity_instance_manager.unregister_reactive_instance(*id);
            }
            reactive_flow.entities_removed.write().unwrap().clear();

            // Register added entities
            for id in reactive_flow.entities_added.read().unwrap().iter() {
                let entity_instance = reactive_flow.get_entity(*id);
                if entity_instance.is_some() {
                    self.reactive_entity_instance_manager.register_reactive_instance(entity_instance.unwrap().clone());
                }
            }
            reactive_flow.entities_added.write().unwrap().clear();

            // Register added relations
            for edge_key in reactive_flow.relations_added.read().unwrap().iter() {
                let relation_instance = reactive_flow.get_relation(edge_key.clone());
                if relation_instance.is_some() {
                    self.reactive_relation_instance_manager.register_reactive_instance(relation_instance.unwrap().clone());
                }
            }
            reactive_flow.relations_added.write().unwrap().clear();

            // for (_, entity_instance) in reactive_flow.entity_instances.read().unwrap().iter() {
            //     if !self.reactive_entity_instance_manager.has(entity_instance.id) {
            //         self.reactive_entity_instance_manager.register_reactive_instance(entity_instance.clone());
            //     }
            // }
            // for (_, relation_instance) in reactive_flow.relation_instances.read().unwrap().iter() {
            //     let edge_key = relation_instance.get_key();
            //     if edge_key.is_some() {
            //         let edge_key = edge_key.unwrap();
            //         if !self.reactive_relation_instance_manager.has(edge_key.clone()) {
            //             self.reactive_relation_instance_manager.register_reactive_instance(relation_instance.clone());
            //         }
            //     }
            // }


            let flow = Flow::try_from(reactive_flow.clone());
            if flow.is_ok() {
                self.flow_manager.commit(flow.unwrap());
            }
        }
    }

    fn delete(&self, id: Uuid) {
        if self.has(id) {
            let reactive_flow = self.get(id).unwrap();
            for (_, entity_instance) in reactive_flow.entity_instances.read().unwrap().iter() {
                self.reactive_entity_instance_manager.unregister_reactive_instance(entity_instance.id);
            }
            for (_, relation_instance) in reactive_flow.relation_instances.read().unwrap().iter() {
                let edge_key = relation_instance.get_key();
                if edge_key.is_some() {
                    self.reactive_relation_instance_manager.unregister_reactive_instance(edge_key.unwrap());
                }
            }
            self.reactive_flows.0.write().unwrap().remove(&id);
        }
    }

    fn import(&self, path: String) -> Result<Arc<ReactiveFlow>, ReactiveFlowImportError> {
        let flow = self.flow_manager.import(path.clone());
        if flow.is_ok() {
            let flow = flow.unwrap();
            let reactive_flow = self.create(flow.clone());
            if reactive_flow.is_ok() {
                return Ok(reactive_flow.unwrap().clone());
            }
        }
        Err(ReactiveFlowImportError.into())
    }

    fn export(&self, id: Uuid, path: String) {
        if self.has(id) {
            self.commit(id);
            let flow = Flow::try_from(self.get(id).unwrap().clone());
            if flow.is_ok() {
                self.flow_manager.export(flow.unwrap(), path);
            }
        }
    }
}

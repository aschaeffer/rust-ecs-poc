use std::sync::Arc;

use async_trait::async_trait;
use log::{debug, error};
use uuid::Uuid;
use waiter_di::*;

use crate::api::EntityTypeManager;
use crate::behaviour::{EntityBehaviourFactory, EntityBehaviourRegistry};
use crate::behaviour::entity::numeric_gates::factory::*;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::numeric_gate::NumericGate;

#[wrapper]
pub struct NumericGatesStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<crate::reactive::entity::numeric_gate::NumericGate<'static>>>>);

#[waiter_di::provides]
fn create_numeric_gates_storage() -> NumericGatesStorage {
    NumericGatesStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

pub trait NumericGatesRegistry: EntityBehaviourRegistry<NumericGate<'static>> {}

#[component]
pub struct NumericGatesRegistryImpl {
    entity_type_manager: Wrc<dyn EntityTypeManager>,

    pow_gate_factory: Arc<PowGateFactory>,

    numeric_gates: NumericGatesStorage,
}

#[async_trait]
#[provides]
impl NumericGatesRegistry for NumericGatesRegistryImpl {}
impl EntityBehaviourRegistry<NumericGate<'static>> for NumericGatesRegistryImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let factory = self.get_entity_behaviour_factory(entity_instance.clone());
        if factory.is_some() {
            let factory = factory.unwrap();
            let behaviour_instance = factory.build(entity_instance.clone());
            if behaviour_instance.is_ok() {
                let behaviour_instance = behaviour_instance.unwrap();
                debug!("NumericGatesRegistry handle_id {}", behaviour_instance.handle_id);
                self.numeric_gates.0.write().unwrap().insert(id, behaviour_instance);
            } else {
                error!("Failed: {}", id);
            }
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_behaviours_by_id(entity_instance.id);
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.numeric_gates.0.write().unwrap().remove(&id);
    }

    fn get_entity_behaviour_factory(&self, entity_instance: Arc<ReactiveEntityInstance>)
                                    -> Option<Arc<dyn EntityBehaviourFactory<NumericGate<'static>>>>
    {
        let entity_type = self.entity_type_manager.get(entity_instance.type_name.clone());
        if entity_type.is_some() {
            match entity_type.unwrap().name.as_str() {
                "pow" => return Some(self.pow_gate_factory.clone()),
                _ => {}
            }
        }
        None
    }
}

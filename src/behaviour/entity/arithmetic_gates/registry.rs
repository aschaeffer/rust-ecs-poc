use std::sync::Arc;

use async_trait::async_trait;
use log::{debug, error};
use uuid::Uuid;
use waiter_di::*;

use crate::api::EntityTypeManager;
use crate::behaviour::{EntityBehaviourFactory, EntityBehaviourRegistry};
use crate::behaviour::entity::arithmetic_gates::factory::*;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::arithmetic_gate::ArithmeticGate;

#[wrapper]
pub struct ArithmeticGatesStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<crate::reactive::entity::arithmetic_gate::ArithmeticGate<'static>>>>);

#[waiter_di::provides]
fn create_arithmetic_gates_storage() -> ArithmeticGatesStorage {
    ArithmeticGatesStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

pub trait ArithmeticGatesRegistry: EntityBehaviourRegistry<ArithmeticGate<'static>> {}

#[component]
pub struct ArithmeticGatesRegistryImpl {
    entity_type_manager: Wrc<dyn EntityTypeManager>,

    add_gate_factory: Arc<AddGateFactory>,
    div_gate_factory: Arc<DivGateFactory>,
    max_gate_factory: Arc<MaxGateFactory>,
    min_gate_factory: Arc<MinGateFactory>,
    mod_gate_factory: Arc<ModGateFactory>,
    mul_gate_factory: Arc<MulGateFactory>,
    sub_gate_factory: Arc<SubGateFactory>,

    arithmetic_gates: ArithmeticGatesStorage,
}

#[async_trait]
#[provides]
impl ArithmeticGatesRegistry for ArithmeticGatesRegistryImpl {}
impl EntityBehaviourRegistry<ArithmeticGate<'static>> for ArithmeticGatesRegistryImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let factory = self.get_entity_behaviour_factory(entity_instance.clone());
        if factory.is_some() {
            let factory = factory.unwrap();
            let behaviour_instance = factory.build(entity_instance.clone());
            if behaviour_instance.is_ok() {
                let behaviour_instance = behaviour_instance.unwrap();
                debug!("ArithmeticGatesRegistry handle_id {}", behaviour_instance.handle_id);
                self.arithmetic_gates.0.write().unwrap().insert(id, behaviour_instance);
            } else {
                error!("Failed: {}", id);
            }
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_behaviours_by_id(entity_instance.id);
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.arithmetic_gates.0.write().unwrap().remove(&id);
    }

    fn get_entity_behaviour_factory(&self, entity_instance: Arc<ReactiveEntityInstance>)
                                    -> Option<Arc<dyn EntityBehaviourFactory<ArithmeticGate<'static>>>>
    {
        let entity_type = self.entity_type_manager.get(entity_instance.type_name.clone());
        if entity_type.is_some() {
            match entity_type.unwrap().name.as_str() {
                "add" => return Some(self.add_gate_factory.clone()),
                "div" => return Some(self.div_gate_factory.clone()),
                "max" => return Some(self.max_gate_factory.clone()),
                "min" => return Some(self.min_gate_factory.clone()),
                "mod" => return Some(self.mod_gate_factory.clone()),
                "mul" => return Some(self.mul_gate_factory.clone()),
                "sub" => return Some(self.sub_gate_factory.clone()),
                _ => {}
            }
        }
        None
    }
}

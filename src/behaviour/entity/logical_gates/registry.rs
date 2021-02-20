use std::sync::Arc;

use async_trait::async_trait;
use log::{debug, error};
use uuid::Uuid;
use waiter_di::*;

use crate::api::EntityTypeManager;
use crate::behaviour::{EntityBehaviourFactory, EntityBehaviourRegistry};
use crate::behaviour::entity::logical_gates::factory::*;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::logical_gate::LogicalGate;

#[wrapper]
pub struct LogicalGatesStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<LogicalGate<'static>>>>);

#[waiter_di::provides]
fn create_logical_gates_storage() -> LogicalGatesStorage {
    LogicalGatesStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

pub trait LogicalGatesRegistry: EntityBehaviourRegistry<LogicalGate<'static>> {}

#[component]
pub struct LogicalGatesRegistryImpl {
    entity_type_manager: Wrc<dyn EntityTypeManager>,

    and_gate_factory: Arc<AndGateFactory>,
    nand_gate_factory: Arc<NandGateFactory>,
    nor_gate_factory: Arc<NorGateFactory>,
    not_gate_factory: Arc<NotGateFactory>,
    or_gate_factory: Arc<OrGateFactory>,
    xor_gate_factory: Arc<XorGateFactory>,
    xnor_gate_factory: Arc<XnorGateFactory>,

    logical_gates: LogicalGatesStorage,
}

#[async_trait]
#[provides]
impl LogicalGatesRegistry for LogicalGatesRegistryImpl {}
impl EntityBehaviourRegistry<LogicalGate<'static>> for LogicalGatesRegistryImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let factory = self.get_entity_behaviour_factory(entity_instance.clone());
        if factory.is_some() {
            let factory = factory.unwrap();
            let behaviour_instance = factory.build(entity_instance.clone());
            if behaviour_instance.is_ok() {
                let behaviour_instance = behaviour_instance.unwrap();
                debug!("LogicalGate handle_id {}", behaviour_instance.handle_id);
                self.logical_gates.0.write().unwrap().insert(id, behaviour_instance);
            } else {
                error!("Failed: {}", id);
            }
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_behaviours_by_id(entity_instance.id);
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.logical_gates.0.write().unwrap().remove(&id);
    }

    fn get_entity_behaviour_factory(&self, entity_instance: Arc<ReactiveEntityInstance>)
                                    -> Option<Arc<dyn EntityBehaviourFactory<LogicalGate<'static>>>>
    {
        let entity_type = entity_instance.type_name.clone();
        if self.entity_type_manager.has(entity_type.clone()) {

            match entity_type.as_str() {
                "and" => return Some(self.and_gate_factory.clone()),
                "nand" => return Some(self.nand_gate_factory.clone()),
                "nor" => return Some(self.nor_gate_factory.clone()),
                "not" => return Some(self.not_gate_factory.clone()),
                "or" => return Some(self.or_gate_factory.clone()),
                "xor" => return Some(self.xor_gate_factory.clone()),
                "xnor" => return Some(self.xnor_gate_factory.clone()),
                _ => {}
            }
        }
        None
    }
}

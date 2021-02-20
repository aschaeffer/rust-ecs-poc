use std::sync::Arc;

use async_trait::async_trait;
use log::{debug, error};
use uuid::Uuid;
use waiter_di::*;

use crate::api::EntityTypeManager;
use crate::behaviour::{EntityBehaviourFactory, EntityBehaviourRegistry};
use crate::behaviour::entity::numeric_operations::factory::*;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::numeric_operation::NumericOperation;

#[wrapper]
pub struct NumericOperationsStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<crate::reactive::entity::numeric_operation::NumericOperation<'static>>>>);

#[waiter_di::provides]
fn create_numeric_operations_storage() -> NumericOperationsStorage {
    NumericOperationsStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

pub trait NumericOperationsRegistry: EntityBehaviourRegistry<NumericOperation<'static>> {}

#[component]
pub struct NumericOperationsRegistryImpl {
    entity_type_manager: Wrc<dyn EntityTypeManager>,

    acos_gate_factory: Arc<AcosGateFactory>,
    asin_gate_factory: Arc<AsinGateFactory>,
    atan_gate_factory: Arc<AtanGateFactory>,
    cos_gate_factory: Arc<CosGateFactory>,
    cosh_gate_factory: Arc<CoshGateFactory>,
    sin_gate_factory: Arc<SinGateFactory>,
    sinh_gate_factory: Arc<SinhGateFactory>,
    tan_gate_factory: Arc<TanGateFactory>,
    tanh_gate_factory: Arc<TanhGateFactory>,

    numeric_operations: NumericOperationsStorage,
}

#[async_trait]
#[provides]

impl NumericOperationsRegistry for NumericOperationsRegistryImpl {}
impl EntityBehaviourRegistry<NumericOperation<'static>> for NumericOperationsRegistryImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let factory = self.get_entity_behaviour_factory(entity_instance.clone());
        if factory.is_some() {
            let factory = factory.unwrap();
            let behaviour_instance = factory.build(entity_instance.clone());
            if behaviour_instance.is_ok() {
                let behaviour_instance = behaviour_instance.unwrap();
                debug!("NumericOperationsRegistry handle_id {}", behaviour_instance.handle_id);
                self.numeric_operations.0.write().unwrap().insert(id, behaviour_instance);
            } else {
                error!("Failed to add numeric operation behaviour: {}", id);
            }
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_behaviours_by_id(entity_instance.id);
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.numeric_operations.0.write().unwrap().remove(&id);
    }

    fn get_entity_behaviour_factory(&self, entity_instance: Arc<ReactiveEntityInstance>)
                                    -> Option<Arc<dyn EntityBehaviourFactory<NumericOperation<'static>>>>
    {
        let entity_type = self.entity_type_manager.get(entity_instance.type_name.clone());
        if entity_type.is_some() {
            match entity_type.unwrap().name.as_str() {
                "acos" => return Some(self.acos_gate_factory.clone()),
                "asin" => return Some(self.asin_gate_factory.clone()),
                "atan" => return Some(self.atan_gate_factory.clone()),
                "cos" => return Some(self.cos_gate_factory.clone()),
                "cosh" => return Some(self.cosh_gate_factory.clone()),
                "sin" => return Some(self.sin_gate_factory.clone()),
                "sinh" => return Some(self.sinh_gate_factory.clone()),
                "tan" => return Some(self.tan_gate_factory.clone()),
                "tanh" => return Some(self.tanh_gate_factory.clone()),
                _ => {}
            }
        }
        None
    }
}

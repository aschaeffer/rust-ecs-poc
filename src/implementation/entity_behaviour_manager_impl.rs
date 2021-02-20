use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;
use waiter_di::*;

use crate::api::EntityBehaviourManager;
use crate::behaviour::{LogicalGatesRegistry, ArithmeticGatesRegistry, NumericOperationsRegistry, NumericGatesRegistry, SimpleClosuresRegistry};
use crate::model::ReactiveEntityInstance;

#[component]
pub struct EntityBehaviourManagerImpl {
    logical_gates_registry: Wrc<dyn LogicalGatesRegistry>,
    arithmetic_gates_registry: Wrc<dyn ArithmeticGatesRegistry>,
    numeric_operations_registry: Wrc<dyn NumericOperationsRegistry>,
    numeric_gates_registry: Wrc<dyn NumericGatesRegistry>,
    simple_closures_registry: Wrc<dyn SimpleClosuresRegistry>,

    // TODO: more behaviours
}

#[async_trait]
#[provides]
impl EntityBehaviourManager for EntityBehaviourManagerImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        println!("EntityBehaviourManager::add_behaviours {}", entity_instance.id);
        self.logical_gates_registry.add_behaviours(entity_instance.clone());
        self.arithmetic_gates_registry.add_behaviours(entity_instance.clone());
        self.numeric_operations_registry.add_behaviours(entity_instance.clone());
        self.numeric_gates_registry.add_behaviours(entity_instance.clone());
        self.simple_closures_registry.add_behaviours(entity_instance.clone());
        // TODO: more behaviours
        // TODO: unit test with multiple behaviours on a single entity
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.logical_gates_registry.remove_behaviours(entity_instance.clone());
        self.arithmetic_gates_registry.remove_behaviours(entity_instance.clone());
        self.numeric_operations_registry.remove_behaviours(entity_instance.clone());
        self.numeric_gates_registry.remove_behaviours(entity_instance.clone());
        self.simple_closures_registry.remove_behaviours(entity_instance.clone());
        // TODO: more behaviours
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.logical_gates_registry.remove_behaviours_by_id(id);
        self.arithmetic_gates_registry.remove_behaviours_by_id(id);
        self.numeric_operations_registry.remove_behaviours_by_id(id);
        self.numeric_gates_registry.remove_behaviours_by_id(id);
        self.simple_closures_registry.remove_behaviours_by_id(id);
        // TODO: more behaviours
    }
}

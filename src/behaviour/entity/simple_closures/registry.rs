use std::sync::Arc;

use async_trait::async_trait;
use log::{debug, error};
use uuid::Uuid;
use waiter_di::*;

use crate::api::EntityTypeManager;
use crate::behaviour::{EntityBehaviourFactory, EntityBehaviourRegistry};
use crate::behaviour::entity::simple_closures::factory::*;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::simple_closure::SimpleClosure;

#[wrapper]
pub struct SimpleClosuresStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<crate::reactive::entity::simple_closure::SimpleClosure>>>);

#[waiter_di::provides]
fn create_simple_closures_storage() -> SimpleClosuresStorage {
    SimpleClosuresStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

pub trait SimpleClosuresRegistry: EntityBehaviourRegistry<SimpleClosure> {}

#[component]
pub struct SimpleClosuresRegistryImpl {
    entity_type_manager: Wrc<dyn EntityTypeManager>,

    log_debug_factory: Arc<LogDebugFactory>,
    log_info_factory: Arc<LogInfoFactory>,

    simple_closures: SimpleClosuresStorage,
}

#[async_trait]
#[provides]

impl SimpleClosuresRegistry for SimpleClosuresRegistryImpl {}
impl EntityBehaviourRegistry<SimpleClosure> for SimpleClosuresRegistryImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let factory = self.get_entity_behaviour_factory(entity_instance.clone());
        if factory.is_some() {
            let factory = factory.unwrap();
            let behaviour_instance = factory.build(entity_instance.clone());
            if behaviour_instance.is_ok() {
                let behaviour_instance = behaviour_instance.unwrap();
                debug!("SimpleClosuresRegistry handle_id {}", behaviour_instance.handle_id);
                self.simple_closures.0.write().unwrap().insert(id, behaviour_instance);
            } else {
                error!("Failed to add simple closure behaviour: {}", id);
            }
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.remove_behaviours_by_id(entity_instance.id);
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.simple_closures.0.write().unwrap().remove(&id);
    }

    fn get_entity_behaviour_factory(&self, entity_instance: Arc<ReactiveEntityInstance>) -> Option<Arc<dyn EntityBehaviourFactory<SimpleClosure>>>
    {
        let entity_type = self.entity_type_manager.get(entity_instance.type_name.clone());
        if entity_type.is_some() {
            match entity_type.unwrap().name.as_str() {
                "log_debug" => return Some(self.log_debug_factory.clone()),
                "log_info" => return Some(self.log_info_factory.clone()),
                _ => {}
            }
        }
        None
    }
}

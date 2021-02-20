use std::sync::Arc;
use crate::model::ReactiveEntityInstance;
use uuid::Uuid;
use crate::behaviour::EntityBehaviourFactory;

pub trait EntityBehaviourRegistry<T>: Send + Sync {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_behaviours_by_id(&self, id: Uuid);

    fn get_entity_behaviour_factory(&self, entity_instance: Arc<ReactiveEntityInstance>) -> Option<Arc<dyn EntityBehaviourFactory<T>>>;
}

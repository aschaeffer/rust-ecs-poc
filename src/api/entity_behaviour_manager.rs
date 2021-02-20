use crate::model::ReactiveEntityInstance;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait EntityBehaviourManager: Send + Sync {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_behaviours_by_id(&self, id: Uuid);
}

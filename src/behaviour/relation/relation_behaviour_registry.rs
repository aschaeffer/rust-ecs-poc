use std::sync::Arc;
use crate::model::ReactiveRelationInstance;
use crate::behaviour::RelationBehaviourFactory;
use indradb::EdgeKey;

pub trait RelationBehaviourRegistry<T>: Send + Sync {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey);

    fn get_relation_behaviour_factory(&self, relation_instance: Arc<ReactiveRelationInstance>) -> Option<Arc<dyn RelationBehaviourFactory<T>>>;
}

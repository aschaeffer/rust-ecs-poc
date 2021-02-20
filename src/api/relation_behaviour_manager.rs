use crate::model::ReactiveRelationInstance;
use async_trait::async_trait;
use std::sync::Arc;
use indradb::EdgeKey;

#[async_trait]
pub trait RelationBehaviourManager: Send + Sync {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey);
}

use std::sync::Arc;

use async_trait::async_trait;
use waiter_di::*;

use crate::api::RelationBehaviourManager;
use crate::behaviour::{ConnectorsRegistry};
use crate::model::ReactiveRelationInstance;
use indradb::EdgeKey;

#[component]
pub struct RelationBehaviourManagerImpl {
    connectors_registry: Wrc<dyn ConnectorsRegistry>,

    // TODO: more behaviours
}

#[async_trait]
#[provides]
impl RelationBehaviourManager for RelationBehaviourManagerImpl {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        // TODO: safety checks
        println!("RelationBehaviourManager::add_behaviours {}", relation_instance.get_key().unwrap().t.0.as_str());
        self.connectors_registry.add_behaviours(relation_instance.clone());
        // TODO: more behaviours
        // TODO: unit test with multiple behaviours on a single relation
    }

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        self.connectors_registry.remove_behaviours(relation_instance.clone());
        // TODO: more behaviours
    }

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {
        self.connectors_registry.remove_behaviours_by_key(edge_key);
        // TODO: more behaviours
    }
}

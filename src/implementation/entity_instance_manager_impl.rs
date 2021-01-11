use crate::api::{
    EntityInstanceManager,
    GraphDatabase,
    ComponentManager,
    EntityTypeManager
};
use crate::model::EntityInstance;
use async_trait::async_trait;
use indradb::{
    Transaction,
    SpecificVertexQuery
};
use serde_json::Value;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use uuid::Uuid;
use waiter_di::*;

// This service operates on the graph database.

#[component]
pub struct EntityInstanceManagerImpl {

    graph_database: Wrc<dyn GraphDatabase>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

}

#[async_trait]
#[provides]
impl EntityInstanceManager for EntityInstanceManagerImpl {

    fn has(&self, id: Uuid) -> bool {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let result = transaction.get_vertices(SpecificVertexQuery::single(id));
            result.into_ok().len() > 0
        }
        false
    }

    fn get(&self, name: String) -> Option<EntityInstance> {
        unimplemented!()
    }

    fn create(&self, type_name: String, properties: HashMap<String, Value, RandomState>) {
        unimplemented!()
    }

    fn delete(&self, id: Uuid) {
        unimplemented!()
    }

    fn import(&self, path: String) {
        unimplemented!()
    }

    fn export(&self, name: String, path: String) {
        unimplemented!()
    }
}

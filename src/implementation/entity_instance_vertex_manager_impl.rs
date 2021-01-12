use crate::api::{
    EntityInstanceVertexManager,
    GraphDatabase,
    ComponentManager,
    EntityTypeManager
};
use crate::model::EntityInstance;
use async_trait::async_trait;
use indradb::{Transaction, SpecificVertexQuery, Vertex, VertexProperties};
use serde_json::Value;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use uuid::Uuid;
use waiter_di::*;

// This service operates on the graph database.

#[component]
pub struct EntityInstanceVertexManagerImpl {

    graph_database: Wrc<dyn GraphDatabase>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

}

#[async_trait]
#[provides]
impl EntityInstanceVertexManager for EntityInstanceVertexManagerImpl {

    fn has(&self, id: Uuid) -> bool {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let vertices = transaction.get_vertices(SpecificVertexQuery::single(id));
            return vertices.unwrap().len() > 0;
        }
        false
    }

    fn get(&self, id: Uuid) -> Option<Vertex> {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let vertices = transaction.get_vertices(SpecificVertexQuery::single(id));
            if vertices.is_ok() {
                let vertices = vertices.unwrap();
                if !vertices.is_empty() {
                    return Some(vertices.first().unwrap().clone());
                }
            }
        }
        None
    }

    fn get_properties(&self, id: Uuid) -> Option<VertexProperties> {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let result = transaction.get_all_vertex_properties(SpecificVertexQuery::single(id));
            if result.is_ok() {
                let vertex_properties = result.unwrap()[0].clone();
                return Some(vertex_properties);
            }
        }
        None
    }

    fn create(&self, type_name: String, properties: HashMap<String, Value, RandomState>) -> Option<Uuid> {
        unimplemented!()
    }

    fn create_with_id(&self, type_name: String, id: Uuid, properties: HashMap<String, Value>) -> Option<Uuid> {
        unimplemented!()
    }

    fn delete(&self, id: Uuid) {
        unimplemented!()
    }
}

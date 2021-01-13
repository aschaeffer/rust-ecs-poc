use crate::api::{
    ComponentManager, EntityInstanceVertexCreationError, EntityInstanceVertexManager,
    EntityTypeManager, GraphDatabase,
};
use async_trait::async_trait;
use indradb::{SpecificVertexQuery, Transaction, Vertex, VertexProperties, VertexQueryExt};
use serde_json::Value;
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

    fn create(
        &self,
        type_name: String,
        properties: HashMap<String, Value>,
    ) -> Result<Uuid, EntityInstanceVertexCreationError> {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            if self.entity_type_manager.has(type_name.clone()) {
                let entity_type = self.entity_type_manager.get(type_name).unwrap();
                let result = transaction.create_vertex_from_type(entity_type.t);
                if result.is_ok() {
                    let id = result.unwrap();
                    let q = SpecificVertexQuery::single(id);
                    for (property_name, value) in properties {
                        let property_result = transaction
                            .set_vertex_properties(q.clone().property(property_name), &value);
                        if !property_result.is_ok() {
                            return Err(EntityInstanceVertexCreationError.into());
                        }
                    }
                    return Ok(id);
                }
            }
        }
        Err(EntityInstanceVertexCreationError.into())
    }

    fn create_with_id(
        &self,
        type_name: String,
        id: Uuid,
        properties: HashMap<String, Value>,
    ) -> Result<Uuid, EntityInstanceVertexCreationError> {
        if !self.has(id) {
            let r_transaction = self.graph_database.get_transaction();
            if r_transaction.is_ok() {
                let transaction = r_transaction.unwrap();
                if self.entity_type_manager.has(type_name.clone()) {
                    let entity_type = self.entity_type_manager.get(type_name).unwrap();
                    let result = transaction.create_vertex(&Vertex::with_id(id, entity_type.t));
                    if result.is_ok() {
                        let q = SpecificVertexQuery::single(id);
                        for (property_name, value) in properties {
                            let property_result = transaction
                                .set_vertex_properties(q.clone().property(property_name), &value);
                            if !property_result.is_ok() {
                                return Err(EntityInstanceVertexCreationError.into());
                            }
                        }
                        return Ok(id);
                    }
                }
            }
        }
        Err(EntityInstanceVertexCreationError.into())
    }

    fn delete(&self, id: Uuid) {
        if self.has(id) {
            let r_transaction = self.graph_database.get_transaction();
            if r_transaction.is_ok() {
                let transaction = r_transaction.unwrap();
                transaction.delete_vertices(SpecificVertexQuery::single(id));
            }
        }
    }
}

use crate::api::{RelationEdgeCreationError, RelationEdgeManager, RelationTypeManager, GraphDatabase};
use async_trait::async_trait;
use indradb::{SpecificEdgeQuery, Transaction, Edge, EdgeProperties, EdgeQueryExt, EdgeKey};
use serde_json::Value;
use std::collections::HashMap;
use waiter_di::*;
use std::collections::hash_map::RandomState;

// This service operates on the graph database.

#[component]
pub struct RelationEdgeManagerImpl {
    graph_database: Wrc<dyn GraphDatabase>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,
}

#[async_trait]
#[provides]
impl RelationEdgeManager for RelationEdgeManagerImpl {
    // // TODO: move out of RelationEdgeManager and allow static access (EdgeKeyBuilder ?)
    // // No usage of &self !
    // fn get_key(&self, outbound_id: Uuid, type_name: String, inbound_id: Uuid) -> Result<EdgeKey, RelationEdgeKeyInvalid> {
    //     let t = Type::from_str(type_name.as_str());
    //     if t.is_ok() {
    //         return Ok(EdgeKey::new(outbound_id, t.unwrap(), inbound_id));
    //     }
    //     Err(RelationEdgeKeyInvalid.into())
    // }

    fn has(&self, edge_key: EdgeKey) -> bool {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let edges = transaction.get_edges(SpecificEdgeQuery::single(edge_key));
            return edges.unwrap().len() > 0;
        }
        false
    }

    fn get(&self, edge_key: EdgeKey) -> Option<Edge> {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let edges = transaction.get_edges(SpecificEdgeQuery::single(edge_key));
            if edges.is_ok() {
                let edges = edges.unwrap();
                if !edges.is_empty() {
                    return Some(edges.first().unwrap().clone());
                }
            }
        }
        None
    }

    fn get_properties(&self, edge_key: EdgeKey) -> Option<EdgeProperties> {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let result = transaction.get_all_edge_properties(SpecificEdgeQuery::single(edge_key));
            if result.is_ok() {
                let edge_properties = result.unwrap();
                if edge_properties.len() > 0 { // == 1 ?
                    let edge_properties = edge_properties[0].clone();
                    return Some(edge_properties);
                }
            }
        }
        None
    }

    fn create(&self, edge_key: EdgeKey, properties: HashMap<String, Value>) -> Result<EdgeKey, RelationEdgeCreationError> {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let type_name = edge_key.t.0.clone();
            if !self.relation_type_manager.has(type_name.clone()) {
                // Relation Type does not exist!
                return Err(RelationEdgeCreationError.into());
            }
            // if self.relation_type_manager.has(type_name.clone()) {
            let relation_type = self.relation_type_manager.get(type_name.clone()).unwrap();
            let result = transaction.create_edge(&edge_key.clone());
            if result.is_ok() {
                let edge_query = SpecificEdgeQuery::single(edge_key.clone());
                for property_type in relation_type.properties {
                    let property_name = property_type.name;
                    let value = properties.get(&*property_name.clone());
                    if value.is_none() {
                        // Missing required property
                        return Err(RelationEdgeCreationError.into());
                    }
                    let value = value.unwrap();
                    let property_query = edge_query.clone().property(property_name);
                    let property_result = transaction.set_edge_properties(property_query, value);
                    if !property_result.is_ok() {
                        return Err(RelationEdgeCreationError.into());
                    }
                }
                return Ok(edge_key.clone());
            }
            // }
        }
        Err(RelationEdgeCreationError.into())
    }

    fn commit(&self, edge_key: EdgeKey, properties: HashMap<String, Value, RandomState>) {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let q = SpecificEdgeQuery::single(edge_key);
            for (property_name, value) in properties {
                let property_query = q.clone().property(property_name);
                let _property_result = transaction.set_edge_properties(property_query, &value);
                // if !property_result.is_ok() {
                //     return Err(EntityVertexCreationError.into());
                // }
            }
        }
    }

    fn delete(&self, edge_key: EdgeKey) {
        if self.has(edge_key.clone()) {
            let r_transaction = self.graph_database.get_transaction();
            if r_transaction.is_ok() {
                let transaction = r_transaction.unwrap();
                let _result = transaction.delete_edges(SpecificEdgeQuery::single(edge_key));
            }
        }
    }
}

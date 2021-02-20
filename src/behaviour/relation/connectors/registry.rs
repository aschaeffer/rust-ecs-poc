use std::sync::Arc;

use async_trait::async_trait;
use waiter_di::*;

use crate::api::RelationTypeManager;
use crate::behaviour::{RelationBehaviourFactory, RelationBehaviourRegistry};
use crate::behaviour::relation::connectors::factory::*;
use crate::model::ReactiveRelationInstance;
use crate::reactive::relation::connector::Connector;
use indradb::EdgeKey;
use log::{debug,error};

#[wrapper]
pub struct ConnectorsStorage(std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<crate::reactive::relation::connector::Connector>>>);

#[waiter_di::provides]
fn create_connectors_storage() -> ConnectorsStorage {
    ConnectorsStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

pub trait ConnectorsRegistry: RelationBehaviourRegistry<Connector> {}

#[component]
pub struct ConnectorsRegistryImpl {
    relation_type_manager: Wrc<dyn RelationTypeManager>,

    default_connector_factory: Arc<DefaultConnectorFactory>,
    parse_int_factory: Arc<ParseIntConnectorFactory>,
    to_string_factory: Arc<ToStringConnectorFactory>,

    connectors: ConnectorsStorage,
}

#[async_trait]
#[provides]
impl ConnectorsRegistry for ConnectorsRegistryImpl {}
impl RelationBehaviourRegistry<Connector> for ConnectorsRegistryImpl {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_some() {
            let edge_key = edge_key.unwrap();
            debug!("Try to add connector behaviour {}", edge_key.clone().t.0.as_str());
            let factory = self.get_relation_behaviour_factory(relation_instance.clone());
            if factory.is_some() {
                let factory = factory.unwrap();
                let behaviour_instance = factory.build(relation_instance.clone());
                if behaviour_instance.is_ok() {
                    let behaviour_instance = behaviour_instance.unwrap();
                    debug!("Added connector behaviour with handle id: {}", behaviour_instance.handle_id);
                    self.connectors.0.write().unwrap().insert(edge_key.clone(), behaviour_instance);
                } else {
                    error!("Failed to add connector behaviour: {}", edge_key.clone().t.0.as_str());
                }
            } else {
                // TODO: Remove
                debug!("ConnectorsRegistry doesn't handle relation type: {}", edge_key.clone().t.0.as_str());
            }
        }
    }

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        self.remove_behaviours_by_key(relation_instance.get_key().unwrap());
    }

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {
        self.connectors.0.write().unwrap().remove(&edge_key.clone());
    }

    fn get_relation_behaviour_factory(&self, relation_instance: Arc<ReactiveRelationInstance>) -> Option<Arc<dyn RelationBehaviourFactory<Connector>>> {
        // The type name have to start with the name of the connector type
        let relation_type = self.relation_type_manager.get_starts_with(relation_instance.type_name.clone());
        if relation_type.is_some() {
            let relation_type = relation_type.unwrap();
            match relation_type.type_name.as_str() {
                "default_connector" => return Some(self.default_connector_factory.clone()),
                "parse_int_connector" => return Some(self.parse_int_factory.clone()),
                "to_string_connector" => return Some(self.to_string_factory.clone()),
                _ => {}
            }
        }
        None
    }
}

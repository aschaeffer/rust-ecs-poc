use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::model::{ReactiveRelationInstance, ReactiveEntityInstance};
use serde_json::{json,Value};
use std::sync::{Arc};
use std::collections::HashMap;

pub static TYPE_NAME_CONNECTOR: &'static str = "connector";
pub static PROPERTY_NAME_OUTBOUND: &'static str = "outbound_property_name";
pub static PROPERTY_NAME_INBOUND: &'static str = "inbound_property_name";

pub struct Connector<'a> {
    pub relation: Arc<ReactiveRelationInstance<'a>>,

    pub handle_id: u128,
}

impl Connector<'_> {
    pub fn from_relation<'a>(relation: Arc<ReactiveRelationInstance<'a>>) -> Connector<'a> {
        let mut connector = Connector {
            relation: relation.clone(),
            handle_id: 0,
        };
        connector.connect();
        connector
    }

    /// Constructs a new connector using an outbound entity (+ name of the property) and
    /// an inbound entity (+ name of the property)
    pub fn new(
        outbound: Arc<ReactiveEntityInstance<'static>>,
        outbound_property_name: String,
        inbound: Arc<ReactiveEntityInstance<'static>>,
        inbound_property_name: String,
    ) -> Connector<'static> {
        let properties = get_connector_relation_properties(outbound_property_name, inbound_property_name);
        let relation = Arc::new(ReactiveRelationInstance::create_with_properties(
            outbound.clone(),
            TYPE_NAME_CONNECTOR.to_string(),
            inbound.clone(),
            properties
        ));
        Connector::from_relation(relation)
    }

    /// TODO: Add guard: disconnect only if connected
    pub fn connect<'a>(&mut self) {
        let outbound_property_name = self.relation.as_string(PROPERTY_NAME_OUTBOUND.to_string());
        let inbound_property_name = self.relation.as_string(PROPERTY_NAME_INBOUND.to_string());
        if outbound_property_name.is_some() && inbound_property_name.is_some() {
            let outbound_property_name = outbound_property_name.unwrap();
            let inbound_property_name = inbound_property_name.unwrap();
            let outbound_property = self.relation.outbound.properties.get(&outbound_property_name.clone());
            let inbound_property = self.relation.inbound.properties.get(&inbound_property_name.clone());
            if outbound_property.is_some() && inbound_property.is_some() {
                let inbound = self.relation.inbound.clone();
                self.handle_id = inbound.id.as_u128();

                outbound_property.unwrap().stream.read().unwrap().observe_with_handle(move |value: &Value| {
                    println!("Propagate value {}", &value.clone());
                    // self.internal_value.read().unwrap().send(&value.clone());
                    inbound.set(inbound_property_name.clone(), value.clone());
                }, self.handle_id);
            }
        }
    }

    /// TODO: Add guard: disconnect only if connected
    pub fn disconnect(&self) {
        let outbound_property_name = self.relation.as_string(PROPERTY_NAME_OUTBOUND.to_string());
        if outbound_property_name.is_some() {
            let outbound_property = self.relation.outbound.properties.get(&outbound_property_name.unwrap().clone());
            if outbound_property.is_some() {
                println!("stopping connector with handle_id {}", self.handle_id);
                outbound_property.unwrap().stream.read().unwrap().remove(self.handle_id);
            }
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for Connector<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}

/// The relation instance of type connector contains exactly two properties
/// which contains the names of the entity properties.
fn get_connector_relation_properties(outbound_property_name: String, inbound_property_name: String) -> HashMap<String, Value> {
    let mut properties = HashMap::new();
    properties.insert(PROPERTY_NAME_OUTBOUND.to_string(), json!(outbound_property_name));
    properties.insert(PROPERTY_NAME_INBOUND.to_string(), json!(inbound_property_name));
    properties
}
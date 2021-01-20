use crate::model::{ReactivePropertyInstance, ReactiveEntityInstance};
use indradb::EdgeProperties;
use std::collections::HashMap;
use uuid::Uuid;
use serde_json::Value;
use std::sync::Arc;
use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};

pub struct ReactiveRelationInstance<'a> {
    pub outbound: Arc<ReactiveEntityInstance<'a>>,

    pub type_name: String,

    pub inbound: Arc<ReactiveEntityInstance<'a>>,

    pub description: String,

    pub properties: HashMap<String, ReactivePropertyInstance<'a>>,

    // TODO: pub components: Vec<String>
    // TODO: pub fn is_a(component: String) -> bool {}
}

impl ReactiveRelationInstance<'_> {
    pub fn from(outbound: Arc<ReactiveEntityInstance<'static>>,
                inbound: Arc<ReactiveEntityInstance<'static>>,
                properties: EdgeProperties
    ) -> ReactiveRelationInstance<'static> {
        let type_name = properties.edge.key.t.0.clone();
        let properties = properties
            .props
            .iter()
            .map(|named_property| {
                (
                    named_property.name.clone(),
                    ReactivePropertyInstance::new(
                        Uuid::new_v4(), // or generate a combined uuid from "outbound_id + type + inbound_id"
                        named_property.name.clone(),
                        named_property.value.clone(),
                    ),
                )
            })
            .collect();
        ReactiveRelationInstance {
            outbound,
            type_name,
            inbound,
            description: String::new(),
            properties,
        }
    }

    // TODO: unit test
    pub fn create_with_properties(
        outbound: Arc<ReactiveEntityInstance<'static>>,
        type_name: String,
        inbound: Arc<ReactiveEntityInstance<'static>>,
        properties: HashMap<String, Value>
    ) -> ReactiveRelationInstance<'static> {
        let properties = properties.iter()
            .map(|(name, value)| {
                (
                    name.clone(),
                    ReactivePropertyInstance::new(
                        Uuid::new_v4(), // or generate a combined uuid from "outbound_id + type + inbound_id"
                        name.clone(),
                        value.clone(),
                    ),
                )
            })
            .collect();
        ReactiveRelationInstance {
            outbound,
            type_name,
            inbound,
            description: String::new(),
            properties,
        }
    }
}

impl PropertyInstanceGetter for ReactiveRelationInstance<'_> {
    // fn set(&self, property_name: String, value: Value) {
    //     if let Some(instance) = self.properties.get(&property_name) {
    //         instance.set(value);
    //     }
    // }

    fn as_bool(&self, property_name: String) -> Option<bool> {
        self.properties.get(&property_name).and_then(|p| p.as_bool())
    }

    fn as_u64(&self, property_name: String) -> Option<u64> {
        self.properties.get(&property_name).and_then(|p| p.as_u64())
    }

    fn as_i64(&self, property_name: String) -> Option<i64> {
        self.properties.get(&property_name).and_then(|p| p.as_i64())
    }

    fn as_f64(&self, property_name: String) -> Option<f64> {
        self.properties.get(&property_name).and_then(|p| p.as_f64())
    }

    fn as_string(&self, property_name: String) -> Option<String> {
        self.properties.get(&property_name).and_then(|p| p.as_string())
    }
}

impl PropertyInstanceSetter for ReactiveRelationInstance<'_> {
    fn set(&self, property_name: String, value: Value) {
        if let Some(instance) = self.properties.get(&property_name) {
            instance.set(value);
        }
    }
}

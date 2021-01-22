use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::model::ReactivePropertyInstance;
use indradb::VertexProperties;
use std::collections::HashMap;
use uuid::Uuid;
use serde_json::Value;

pub struct ReactiveEntityInstance<'a> {
    pub type_name: String,

    pub id: Uuid,

    pub description: String,

    pub properties: HashMap<String, ReactivePropertyInstance<'a>>,

    // TODO: pub components: Vec<String>
    // TODO: pub fn is_a(component: String) -> bool {}
}

impl ReactiveEntityInstance<'_> {
    pub fn from(properties: VertexProperties) -> ReactiveEntityInstance<'static> {
        let type_name = properties.vertex.t.0.clone();
        let id = properties.vertex.id.clone();
        let properties = properties
            .props
            .iter()
            .map(|named_property| {
                (
                    named_property.name.clone(),
                    ReactivePropertyInstance::new(
                        id.clone(),
                        named_property.name.clone(),
                        named_property.value.clone(),
                    ),
                )
            })
            .collect();
        ReactiveEntityInstance {
            type_name,
            id,
            description: String::new(),
            properties,
        }
    }
}

impl PropertyInstanceGetter for ReactiveEntityInstance<'_> {
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

impl PropertyInstanceSetter for ReactiveEntityInstance<'_> {
    fn set(&self, property_name: String, value: Value) {
        if let Some(instance) = self.properties.get(&property_name) {
            instance.set(value);
        }
    }

    // TODO: fn set(&self, Map<String, Value>
    // TODO: Set values transactional: first set all values internally, then send all affected streams
}

use indradb::VertexProperties;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;
use crate::api::{MutablePropertyInstanceSetter, PropertyInstanceGetter};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntityInstance {
    #[serde(alias = "type")]
    pub type_name: String,

    pub id: Uuid,

    #[serde(default = "String::new")]
    pub description: String,

    #[serde(default = "HashMap::new")]
    pub properties: HashMap<String, Value>,
}

impl EntityInstance {
    pub fn new(type_name: String, id: Uuid, properties: HashMap<String, Value>) -> EntityInstance {
        EntityInstance {
            type_name,
            id,
            description: String::from(""),
            properties,
        }
    }

    pub fn from_vertex_properties(properties: VertexProperties) -> EntityInstance {
        let type_name = properties.vertex.t.0.clone();
        let id = properties.vertex.id.clone();
        let properties: HashMap<String, Value> = properties
            .props
            .iter()
            .map(|p| (p.name.clone(), p.value.clone()))
            .collect();
        EntityInstance {
            type_name,
            id,
            description: String::new(),
            properties,
        }
    }
}

impl PropertyInstanceGetter for EntityInstance {
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
        self.properties.get(&property_name).and_then(|p| p.as_str().and_then(|s| Some(s.to_string())))
    }
}

impl MutablePropertyInstanceSetter for EntityInstance {
    fn set(&mut self, property_name: String, value: Value) {
        let property_value = self.properties.get_mut(&property_name).unwrap();
        *property_value = value.clone()
    }
}

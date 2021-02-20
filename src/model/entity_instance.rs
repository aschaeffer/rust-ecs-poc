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
    pub fn new<S: Into<String>>(type_name: S, id: Uuid, properties: HashMap<String, Value>) -> EntityInstance {
        EntityInstance {
            type_name: type_name.into(),
            id,
            description: String::from(""),
            properties,
        }
    }
}

impl From<VertexProperties> for EntityInstance {
    fn from(properties: VertexProperties) -> Self {
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
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value> {
        self.properties.get(&property_name.into()).and_then(|v| Some(v.clone()))
    }

    fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_bool())
    }

    fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_u64())
    }

    fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_i64())
    }

    fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_f64())
    }

    fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_str().and_then(|s| Some(s.to_string())))
    }
}

impl MutablePropertyInstanceSetter for EntityInstance {
    fn set<S: Into<String>>(&mut self, property_name: S, value: Value) {
        let property_value = self.properties.get_mut(&property_name.into()).unwrap();
        *property_value = value.clone()
    }
}

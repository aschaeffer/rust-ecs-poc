use indradb::{EdgeProperties, EdgeKey, Type};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;
use crate::api::{PropertyInstanceGetter, MutablePropertyInstanceSetter};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationInstance {

    /// The id of the outbound vertex.
    pub outbound_id: Uuid,

    /// The name of the relation type
    #[serde(alias = "type")]
    pub type_name: String,

    /// The id of the inbound vertex.
    pub inbound_id: Uuid,

    #[serde(default = "String::new")]
    pub description: String,

    #[serde(default = "HashMap::new")]
    pub properties: HashMap<String, Value>,
}

impl RelationInstance {
    pub fn new(
        outbound_id: Uuid,
        type_name: String,
        inbound_id: Uuid,
        properties: HashMap<String, Value>
    ) -> RelationInstance {
        RelationInstance {
            outbound_id,
            type_name,
            inbound_id,
            description: String::from(""),
            properties,
        }
    }

    pub fn get_key(&self) -> Option<EdgeKey> {
        let t = Type::from_str(self.type_name.as_str());
        if t.is_ok() {
            return Some(EdgeKey::new(self.outbound_id, t.unwrap(), self.inbound_id));
        }
        None
    }
}

impl From<EdgeProperties> for RelationInstance {
    fn from(properties: EdgeProperties) -> Self {
        let outbound_id = properties.edge.key.outbound_id.clone();
        let type_name = properties.edge.key.t.0.clone();
        let inbound_id = properties.edge.key.inbound_id.clone();
        let properties: HashMap<String, Value> = properties.props.iter()
            .map(|p| (p.name.clone(), p.value.clone()))
            .collect();
        RelationInstance {
            outbound_id,
            type_name,
            inbound_id,
            description: String::new(),
            properties,
        }
    }
}

impl PropertyInstanceGetter for RelationInstance {
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

impl MutablePropertyInstanceSetter for RelationInstance {
    fn set<S: Into<String>>(&mut self, property_name: S, value: Value) {
        let property_value = self.properties.get_mut(&property_name.into()).unwrap();
        *property_value = value.clone()
    }
}

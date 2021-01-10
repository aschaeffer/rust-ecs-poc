use indradb::VertexProperties;
use uuid::Uuid;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    pub fn from (properties: VertexProperties) -> EntityInstance {
        let type_name = properties.vertex.t.0.clone();
        let id = properties.vertex.id.clone();
        let properties: HashMap<String, Value> = properties.props
            .iter()
            .map(| p | (p.name.clone(), p.value.clone()))
            .collect();
        EntityInstance {
            type_name,
            id,
            description: String::new(),
            properties
        }
    }

}

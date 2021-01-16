use indradb::EdgeProperties;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

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

    pub fn from_edge_properties(properties: EdgeProperties) -> RelationInstance {
        let outbound_id = properties.edge.key.outbound_id.clone();
        let type_name = properties.edge.key.t.0.clone();
        let inbound_id = properties.edge.key.inbound_id.clone();
        let properties: HashMap<String, Value> = properties
            .props
            .iter()
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

use crate::model::property_type::PropertyType;
use indradb::Type;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RelationType {
    pub name: String,

    #[serde(default = "empty_string")]
    pub description: String,

    pub outbound_type: String,
    pub inbound_type: String,
    pub components: Vec<String>,
    pub properties: Vec<PropertyType>,

    #[serde(skip)]
    pub t: Type,
}

impl RelationType {
    pub fn new(
        name: String,
        outbound_type: String,
        inbound_type: String,
        components: Vec<String>,
        properties: Vec<PropertyType>,
    ) -> RelationType {
        RelationType {
            name,
            description: String::from(""),
            outbound_type,
            inbound_type,
            components,
            properties,
            t: Default::default(),
        }
    }

    /// Returns true, if the relation type is a.
    pub fn is_a(&self, component_name: String) -> bool {
        self.components.contains(&component_name)
    }

    /// Returns true, if the relation type contains an own property with the given name.
    /// Doesn't respect properties from potential components.
    pub fn has_own_property(&self, property_name: String) -> bool {
        !self
            .properties
            .iter()
            .filter(|&p| p.name == property_name)
            .collect::<Vec<_>>()
            .is_empty()
    }
}

#[cfg_attr(tarpaulin, ignore)]
fn empty_string() -> String {
    "".to_string()
}

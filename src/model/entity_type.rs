use crate::model::property_type::PropertyType;
use indradb::Type;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntityType {
    pub name: String,

    #[serde(default = "empty_string")]
    pub description: String,

    pub components: Vec<String>,
    pub properties: Vec<PropertyType>,

    #[serde(skip)]
    pub t: Type,
}

impl EntityType {
    pub fn new(
        name: String,
        components: Vec<String>,
        properties: Vec<PropertyType>
    ) -> EntityType {
        EntityType {
            name,
            description: String::from(""),
            components,
            properties,
            t: Default::default(),
        }
    }

    /// Returns true, if the entity type is a.
    pub fn is_a(&self, component_name: String) -> bool {
        self.components.contains(&component_name)
    }

    /// Returns true, if the entity type contains an own property with the given name.
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

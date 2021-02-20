use crate::model::property_type::PropertyType;
use indradb::Type;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationType {
    pub outbound_type: String,

    #[serde(alias = "name")]
    pub type_name: String,

    pub inbound_type: String,

    #[serde(default = "empty_string")]
    pub description: String,

    #[serde(default = "Vec::new")]
    pub components: Vec<String>,

    #[serde(default = "Vec::new")]
    pub behaviours: Vec<String>,

    #[serde(default = "Vec::new")]
    pub properties: Vec<PropertyType>,

    #[serde(skip)]
    pub t: Type,
}

impl RelationType {
    pub fn new(
        outbound_type: String,
        type_name: String,
        inbound_type: String,
        components: Vec<String>,
        behaviours: Vec<String>,
        properties: Vec<PropertyType>,
    ) -> RelationType {
        let t = Type::from_str(type_name.clone().as_str()).unwrap();
        RelationType {
            outbound_type,
            type_name,
            inbound_type,
            description: empty_string(),
            components,
            behaviours,
            properties,
            t,
        }
    }

    /// Returns true, if the relation type is a.
    pub fn is_a(&self, component_name: String) -> bool {
        self.components.contains(&component_name)
    }

    /// Returns true, if the relation type behaves as.
    pub fn behaves_as(&self, behaviour_name: String) -> bool {
        self.behaviours.contains(&behaviour_name)
    }

    /// Returns true, if the relation type contains an own property with the given name.
    /// Doesn't respect properties from potential components.
    pub fn has_own_property(&self, property_name: String) -> bool {
        !self.properties.iter()
            .filter(|&p| p.name == property_name).collect::<Vec<_>>().is_empty()
    }
}

#[cfg_attr(tarpaulin, ignore)]
fn empty_string() -> String {
    "".to_string()
}

// #[cfg_attr(tarpaulin, ignore)]
// fn empty_vec() -> Vec<String> {
//     Vec::new()
// }

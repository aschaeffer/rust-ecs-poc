use crate::model::property_type::PropertyType;
use indradb::Type;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RelationType {
    pub name: String,
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
            outbound_type,
            inbound_type,
            components,
            properties,
            t: Default::default(),
        }
    }
}

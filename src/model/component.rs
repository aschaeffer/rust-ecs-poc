use crate::model::property_type::PropertyType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Component {
    pub name: String,
    pub properties: Vec<PropertyType>,
}

impl Component {
    pub fn new (name: String, properties: Vec<PropertyType>) -> Component {
        Component {
            name,
            properties,
        }
    }
}

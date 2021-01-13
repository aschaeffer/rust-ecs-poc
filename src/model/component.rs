use crate::model::property_type::PropertyType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Component {
    /// The name of the component.
    pub name: String,

    /// The properties which are applied on entity instances.
    pub properties: Vec<PropertyType>,
}

impl Component {
    /// Constructs a new component with the given name and properties
    pub fn new(name: String, properties: Vec<PropertyType>) -> Component {
        Component { name, properties }
    }

    /// Returns true, if the component contains a property with the given name.
    pub fn has_property(&self, property_name: String) -> bool {
        !self
            .properties
            .iter()
            .filter(|&p| p.name == property_name)
            .collect::<Vec<_>>()
            .is_empty()
    }
}

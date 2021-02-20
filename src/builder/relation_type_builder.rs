use crate::model::{PropertyType, RelationType, DataType};
use std::sync::Arc;
use crate::api::RelationTypeManager;

#[allow(dead_code)]
pub struct RelationTypeBuilder {
    outbound_type: String,
    type_name: String,
    inbound_type: String,
    components: Vec<String>,
    behaviours: Vec<String>,
    properties: Vec<PropertyType>
}

#[allow(dead_code)]
impl RelationTypeBuilder {
    pub fn new<S: Into<String>>(outbound_type: S, type_name: S, inbound_type: S) -> RelationTypeBuilder {
        RelationTypeBuilder {
            outbound_type: outbound_type.into(),
            type_name: type_name.into(),
            inbound_type: inbound_type.into(),
            components: Vec::new(),
            behaviours: Vec::new(),
            properties: Vec::new(),
        }
    }

    pub fn component<'a, S: Into<String>>(&'a mut self, component_name: S) -> &'a mut RelationTypeBuilder {
        self.components.push(component_name.into());
        self
    }

    pub fn behaviour<'a, S: Into<String>>(&'a mut self, behaviour_name: S) -> &'a mut RelationTypeBuilder {
        self.behaviours.push(behaviour_name.into());
        self
    }

    pub fn property<'a, S: Into<String>>(&'a mut self, property_name: S, data_type: DataType) -> &'a mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), data_type));
        self
    }

    pub fn string_property<'a, S: Into<String>>(&'a mut self, property_name: S) -> &'a mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::String));
        self
    }

    pub fn build<'a>(&'a mut self) -> RelationType {
        RelationType::new(
            self.outbound_type.clone(),
            self.type_name.clone(),
            self.inbound_type.clone(),
            self.components.to_vec(),
            self.behaviours.to_vec(),
            self.properties.to_vec()
        )
    }

    pub fn register<'a>(&'a mut self, relation_type_manager: Arc<dyn RelationTypeManager>) -> RelationType {
        let relation_type = self.build();
        relation_type_manager.register(relation_type.clone());
        relation_type
    }

}

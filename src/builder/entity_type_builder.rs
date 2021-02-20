use crate::model::{PropertyType, EntityType, DataType};
use std::sync::Arc;
use crate::api::EntityTypeManager;

#[allow(dead_code)]
pub struct EntityTypeBuilder {
    type_name: String,
    components: Vec<String>,
    behaviours: Vec<String>,
    properties: Vec<PropertyType>
}

#[allow(dead_code)]
impl EntityTypeBuilder {
    pub fn new<S: Into<String>>(type_name: S) -> EntityTypeBuilder {
        EntityTypeBuilder {
            type_name: type_name.into(),
            components: Vec::new(),
            behaviours: Vec::new(),
            properties: Vec::new(),
        }
    }

    pub fn component<'a, S: Into<String>>(&'a mut self, component_name: S) -> &'a mut EntityTypeBuilder {
        self.components.push(component_name.into());
        self
    }

    pub fn behaviour<'a, S: Into<String>>(&'a mut self, behaviour_name: S) -> &'a mut EntityTypeBuilder {
        self.behaviours.push(behaviour_name.into());
        self
    }

    pub fn property<'a, S: Into<String>>(&'a mut self, property_name: S, data_type: DataType) -> &'a mut EntityTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), data_type));
        self
    }

    pub fn string_property<'a, S: Into<String>>(&'a mut self, property_name: S) -> &'a mut EntityTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::String));
        self
    }

    pub fn bool_property<'a, S: Into<String>>(&'a mut self, property_name: S) -> &'a mut EntityTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Bool));
        self
    }

    pub fn number_property<'a, S: Into<String>>(&'a mut self, property_name: S) -> &'a mut EntityTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Number));
        self
    }

    pub fn build<'a>(&'a mut self) -> EntityType {
        EntityType::new(
            self.type_name.clone(),
            self.components.to_vec(),
            self.behaviours.to_vec(),
            self.properties.to_vec()
        )
    }

    pub fn register<'a>(&'a mut self, entity_type_manager: Arc<dyn EntityTypeManager>) -> EntityType {
        let entity_type = self.build();
        entity_type_manager.register(entity_type.clone());
        entity_type
    }

}

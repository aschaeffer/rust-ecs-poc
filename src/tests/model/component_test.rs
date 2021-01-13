use crate::model::{Component, PropertyType};

#[test]
fn create_component_test() {
    let component_name = String::from("component_name");
    let mut property_types = Vec::new();
    let property_name = String::from("property_name");
    let property_type = PropertyType::new(property_name, String::from("string"));
    property_types.push(property_type.clone());
    let c = Component::new(component_name.clone(), property_types.clone());
    assert_eq!(component_name, c.name);
    assert_eq!(property_type.name, c.properties.first().unwrap().name);
    assert_eq!(property_type.data_type, c.properties.first().unwrap().data_type);
}

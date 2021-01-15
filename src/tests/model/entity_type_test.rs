use crate::model::{EntityType, PropertyType};

#[test]
fn create_entity_type_test() {
    let entity_type_name = String::from("entity_type_name");

    let component_name = String::from("component_name");
    let mut component_names = Vec::new();
    component_names.push(component_name.clone());

    let mut property_types = Vec::new();
    let property_name = String::from("property_name");
    let property_type = PropertyType::new(property_name.clone(), String::from("string"));
    property_types.push(property_type.clone());

    let entity_type = EntityType::new(entity_type_name.clone(), component_names, property_types);

    assert_eq!(entity_type_name, entity_type.name);

    assert_eq!(component_name, *entity_type.components.first().unwrap());

    assert!(entity_type.is_a(component_name.clone()));

    assert_eq!(property_name, *entity_type.properties.first().unwrap().name);

    assert!(entity_type.has_own_property(property_name.clone()));
}

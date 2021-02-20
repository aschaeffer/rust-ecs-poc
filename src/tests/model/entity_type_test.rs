use crate::model::{EntityType, PropertyType, DataType};
use crate::tests::r_string;

#[test]
fn create_entity_type_test() {
    let entity_type_name = "entity_type_name";

    let component_name = r_string();
    let mut component_names = Vec::new();
    component_names.push(component_name.clone());

    let mut property_types = Vec::new();
    let property_name = "property_name";
    let property_type = PropertyType::new(property_name, DataType::String);
    property_types.push(property_type.clone());

    let entity_type = EntityType::new(entity_type_name, component_names, Vec::new(), property_types);

    assert_eq!(entity_type_name, entity_type.name);

    assert_eq!(component_name, *entity_type.components.first().unwrap());

    assert!(entity_type.is_a(component_name.clone()));

    assert_eq!(property_name, entity_type.properties.first().unwrap().name);

    assert!(entity_type.has_own_property(property_name));
}

use crate::model::{Component, PropertyType};
use random_string::{RandomString, Charset, Charsets};

#[test]
fn component_test() {
    let component_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();
    let property_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let mut property_types = Vec::new();
    let property_type = PropertyType::new(property_name.clone(), String::from("string"));
    property_types.push(property_type.clone());

    let mut component = Component {
        name: component_name.clone(),
        properties: property_types
    };

    let component_name_2 =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    assert_eq!(component_name, component.name);
    component.name = component_name_2.clone();
    assert_ne!(component_name, component.name);
    assert_eq!(component_name_2, component.name);

    let component_2 = component.clone();
    assert_eq!(component_2.name, component.name);

}

#[test]
fn create_component_test() {
    let component_name = String::from("component_name");
    let mut property_types = Vec::new();
    let property_name = String::from("property_name");
    let property_type = PropertyType::new(property_name.clone(), String::from("string"));
    property_types.push(property_type.clone());
    let component = Component::new(component_name.clone(), property_types.clone());
    assert_eq!(component_name, component.name);
    assert_eq!(property_name.clone(), component.properties.first().unwrap().name);
    assert_eq!(
        property_type.data_type,
        component.properties.first().unwrap().data_type
    );
    assert!(component.has_property(property_name.clone()));
}

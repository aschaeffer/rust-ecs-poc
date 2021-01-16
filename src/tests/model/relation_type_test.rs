use crate::model::{RelationType, PropertyType};
use random_string::{RandomString, Charset, Charsets};
use uuid::v1::Timestamp;
use uuid::Uuid;

#[test]
fn create_relation_type_test() {

    let ts = Timestamp::from_rfc4122(1497624119, 0);
    let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6], ).unwrap();

    let type_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();
    let outbound_type =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();
    let inbound_type =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();
    let component_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();
    let property_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let mut component_names = Vec::new();
    component_names.push(component_name.clone());

    let mut property_types = Vec::new();
    let property_type = PropertyType::new(property_name.clone(), String::from("string"));
    property_types.push(property_type.clone());

    let relation_type = RelationType::new(
        type_name.clone(),
        outbound_type.clone(),
        inbound_type.clone(),
        component_names,
        property_types
    );

    assert_eq!(type_name, relation_type.name);

    assert_eq!(outbound_type, relation_type.outbound_type);

    assert_eq!(inbound_type, relation_type.inbound_type);

    assert_eq!(component_name, *relation_type.components.first().unwrap());

    assert!(relation_type.is_a(component_name.clone()));

    assert_eq!(property_name, *relation_type.properties.first().unwrap().name);

    assert!(relation_type.has_own_property(property_name.clone()));
}

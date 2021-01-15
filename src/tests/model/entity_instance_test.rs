use crate::model::{EntityInstance};
use uuid::Uuid;
use uuid::v1::Timestamp;
use std::collections::HashMap;
use random_string::{RandomString, Charset, Charsets};
use indradb::{VertexProperties, Vertex, Type, NamedProperty};
use std::str::FromStr;
use serde_json::json;

#[test]
fn entity_instance_test() {
    let ts = Timestamp::from_rfc4122(1497624119, 0);
    let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6], ).unwrap();
    let type_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();
    let description =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let properties = HashMap::new();

    let entity_instance = EntityInstance {
        type_name: type_name.clone(),
        id: uuid.clone(),
        description: description.to_string(),
        properties: properties.clone(),
    };

    assert_eq!(type_name.clone(), entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert_eq!(description.clone(), entity_instance.description.clone());
    assert_eq!(properties.clone(), entity_instance.properties.clone());

}

#[test]
fn create_entity_instance_test() {
    let ts = Timestamp::from_rfc4122(1497624119, 0);
    let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6], ).unwrap();
    let type_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let properties = HashMap::new();

    let entity_instance = EntityInstance::new(
        type_name.clone(),
        uuid.clone(),
        properties.clone(),
    );

    assert_eq!(type_name.clone(), entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert_eq!(properties.clone(), properties.clone());

}

#[test]
fn create_entity_instance_from_vertex_properties() {
    let ts = Timestamp::from_rfc4122(1497624119, 0);
    let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6], ).unwrap();

    let type_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let t = Type::from_str(type_name.as_str()).unwrap();

    let property_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let property_value =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let property_value_json = json!(property_value);

    let property = NamedProperty {
        name: property_name.clone(),
        value: property_value_json
    };

    let properties = vec![
        property
    ];

    let vertex_properties = VertexProperties {
        vertex: Vertex { id: uuid, t: t.clone() },
        props: properties.clone()

    };
    let entity_instance = EntityInstance::from_vertex_properties(vertex_properties);

    assert_eq!(type_name.clone(), entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert_eq!(property_value.as_str(), entity_instance.properties.get(property_name.as_str()).unwrap().as_str().unwrap());

}

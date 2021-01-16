use crate::model::ReactiveEntityInstance;
use uuid::v1::Timestamp;
use uuid::Uuid;
use random_string::{RandomString, Charset, Charsets};
use std::collections::HashMap;
use indradb::{VertexProperties, Vertex, Type, NamedProperty};
use std::str::FromStr;
use serde_json::json;

#[test]
fn reactive_entity_instance_test() {
    let ts = Timestamp::from_rfc4122(1497624119, 0);
    let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6], ).unwrap();
    let type_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();
    let description =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let reactive_entity_instance = ReactiveEntityInstance {
        type_name: type_name.clone(),
        id: uuid.clone(),
        description: description.clone(),
        properties: HashMap::new()
    };

    assert_eq!(type_name.clone(), reactive_entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), reactive_entity_instance.id.clone());
    assert_eq!(description.clone(), reactive_entity_instance.description.clone());

}

#[test]
fn reactive_entity_instance_from_vertex_properties_test() {

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

    let reactive_entity_instance = ReactiveEntityInstance::from(vertex_properties);

    assert_eq!(type_name.clone(), reactive_entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), reactive_entity_instance.id.clone());

    assert_eq!(property_name.clone(), reactive_entity_instance.properties[&property_name.clone()].name);
    assert_eq!(property_value.clone(), reactive_entity_instance.properties[&property_name.clone()].get());


}
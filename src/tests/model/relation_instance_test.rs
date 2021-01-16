use crate::model::RelationInstance;
use uuid::Uuid;
use uuid::v1::Timestamp;
use std::collections::HashMap;
use random_string::{RandomString, Charset, Charsets};
use indradb::{Type, NamedProperty, EdgeProperties, Edge, EdgeKey};
use std::str::FromStr;
use serde_json::json;

#[test]
fn relation_instance_test() {
    let ts = Timestamp::from_rfc4122(1497624119, 0);
    let outbound_id = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6], ).unwrap();
    let inbound_id = Uuid::new_v1(ts, &[6, 5, 4, 3, 2, 1], ).unwrap();
    let type_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();
    let description =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let properties = HashMap::new();

    let relation_instance = RelationInstance {
        outbound_id,
        type_name: type_name.clone(),
        inbound_id,
        description: description.to_string(),
        properties: properties.clone(),
    };

    assert_eq!(outbound_id.clone(), relation_instance.outbound_id.clone());
    assert_eq!(type_name.clone(), relation_instance.type_name.clone());
    assert_eq!(inbound_id.clone(), relation_instance.inbound_id.clone());
    assert_eq!(description.clone(), relation_instance.description.clone());
    assert_eq!(properties.clone(), relation_instance.properties.clone());

}

#[test]
fn create_relation_instance_test() {
    let ts = Timestamp::from_rfc4122(1497624119, 0);
    let outbound_id = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6], ).unwrap();
    let inbound_id = Uuid::new_v1(ts, &[6, 5, 4, 3, 2, 1], ).unwrap();
    let type_name =
        RandomString::generate(10, &Charset::from_charsets(Charsets::Letters)).to_string();

    let properties = HashMap::new();

    let relation_instance = RelationInstance::new(
        outbound_id,
        type_name.clone(),
        inbound_id,
        properties.clone(),
    );

    assert_eq!(outbound_id.clone(), relation_instance.outbound_id.clone());
    assert_eq!(type_name.clone(), relation_instance.type_name.clone());
    assert_eq!(inbound_id.clone(), relation_instance.inbound_id.clone());
    assert_eq!(properties.clone(), properties.clone());

}

#[test]
fn create_relation_instance_from_edge_properties() {
    let ts = Timestamp::from_rfc4122(1497624119, 0);
    let outbound_id = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6], ).unwrap();
    let inbound_id = Uuid::new_v1(ts, &[6, 5, 4, 3, 2, 1], ).unwrap();

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
    let edge_key = EdgeKey::new(outbound_id, t, inbound_id);
    let edge_properties = EdgeProperties::new(
        Edge::new_with_current_datetime(edge_key),
        properties.clone()
    );

    let relation_instance = RelationInstance::from_edge_properties(edge_properties);

    assert_eq!(outbound_id.clone(), relation_instance.outbound_id.clone());
    assert_eq!(type_name.clone(), relation_instance.type_name.clone());
    assert_eq!(inbound_id.clone(), relation_instance.inbound_id.clone());
    assert_eq!(property_value.as_str(), relation_instance.properties.get(property_name.as_str()).unwrap().as_str().unwrap());

}

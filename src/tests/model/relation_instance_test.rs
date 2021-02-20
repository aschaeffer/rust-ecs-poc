use crate::model::RelationInstance;
use uuid::Uuid;
use std::collections::HashMap;
use crate::tests::utils::r_string;
use indradb::{Type, NamedProperty, EdgeProperties, Edge, EdgeKey};
use std::str::FromStr;
use serde_json::json;
use crate::api::{MutablePropertyInstanceSetter, PropertyInstanceGetter};

#[test]
fn relation_instance_test() {
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let description = r_string();
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
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
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
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let t = Type::from_str(type_name.as_str()).unwrap();
    let property_name = r_string();
    let property_value = r_string();
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
    let relation_instance = RelationInstance::from(edge_properties);
    assert_eq!(outbound_id.clone(), relation_instance.outbound_id.clone());
    assert_eq!(type_name.clone(), relation_instance.type_name.clone());
    assert_eq!(inbound_id.clone(), relation_instance.inbound_id.clone());
    assert_eq!(property_value.as_str(), relation_instance.properties.get(property_name.as_str()).unwrap().as_str().unwrap());
}

#[test]
fn relation_instance_typed_getter_test() {
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let property_name = r_string();
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(),  json!(false));
    let mut i = RelationInstance::new(
        outbound_id,
        type_name.clone(),
        inbound_id,
        properties.clone(),
    );
    i.set(property_name.clone(), json!(true));
    assert!(i.as_bool(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(false));
    assert!(!i.as_bool(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(123));
    assert_eq!(123, i.as_u64(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(-123));
    assert_eq!(-123, i.as_i64(property_name.clone()).unwrap());
    i.set(property_name.clone(), json!(1.23));
    assert_eq!(1.23, i.as_f64(property_name.clone()).unwrap());
    let s = r_string();
    i.set(property_name.clone(), json!(s.clone()));
    assert_eq!(s, i.as_string(property_name.clone()).unwrap());
}

#[test]
fn relation_instance_get_key_test() {
    let outbound_id = Uuid::new_v4();
    let inbound_id = Uuid::new_v4();
    let type_name = r_string();
    let description = r_string();
    let properties = HashMap::new();
    let relation_instance = RelationInstance {
        outbound_id,
        type_name: type_name.clone(),
        inbound_id,
        description: description.to_string(),
        properties: properties.clone(),
    };
    let edge_key = relation_instance.get_key();
    assert!(edge_key.is_some());
    assert_eq!(EdgeKey::new(outbound_id, Type::new(type_name.clone()).unwrap(), inbound_id), edge_key.unwrap());
}

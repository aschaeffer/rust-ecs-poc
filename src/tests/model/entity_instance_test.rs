use crate::model::EntityInstance;
use uuid::Uuid;
use std::collections::HashMap;
use crate::tests::utils::r_string;
use indradb::{VertexProperties, Vertex, Type, NamedProperty};
use std::str::FromStr;
use serde_json::json;
use crate::api::{MutablePropertyInstanceSetter, PropertyInstanceGetter};

#[test]
fn entity_instance_test() {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let description = r_string();
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
    let uuid = Uuid::new_v4();
    let type_name = r_string();
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
    let uuid = Uuid::new_v4();
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
    let vertex_properties = VertexProperties {
        vertex: Vertex { id: uuid, t: t.clone() },
        props: properties.clone()
    };
    let entity_instance = EntityInstance::from(vertex_properties);
    assert_eq!(type_name.clone(), entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), entity_instance.id.clone());
    assert_eq!(property_value.as_str(), entity_instance.properties.get(property_name.as_str()).unwrap().as_str().unwrap());
}

#[test]
fn entity_instance_typed_getter_test() {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let property_name = r_string();
    let mut properties = HashMap::new();
    properties.insert(property_name.clone(),  json!(false));
    let mut i = EntityInstance::new(
        type_name.clone(),
        uuid.clone(),
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

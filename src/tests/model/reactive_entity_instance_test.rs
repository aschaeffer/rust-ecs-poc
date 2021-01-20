use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::model::ReactiveEntityInstance;
use indradb::{VertexProperties, Vertex, Type, NamedProperty};
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;
use crate::tests::utils::r_string;

#[test]
fn reactive_entity_instance_test() {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let description = r_string();

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
    let reactive_entity_instance = ReactiveEntityInstance::from(vertex_properties);
    assert_eq!(type_name.clone(), reactive_entity_instance.type_name.clone());
    assert_eq!(uuid.clone(), reactive_entity_instance.id.clone());
    assert_eq!(property_name.clone(), reactive_entity_instance.properties[&property_name.clone()].name);
    assert_eq!(property_value.clone(), reactive_entity_instance.properties[&property_name.clone()].get());
}

#[test]
fn reactive_entity_instance_typed_getter_test() {
    let property_name = r_string();
    let i = create_random_entity_instance(property_name.clone());
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

pub fn create_random_entity_instance(property_name: String) -> ReactiveEntityInstance<'static> {
    let uuid = Uuid::new_v4();
    let type_name = r_string();
    let t = Type::from_str(type_name.as_str()).unwrap();
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
    ReactiveEntityInstance::from(vertex_properties)
}

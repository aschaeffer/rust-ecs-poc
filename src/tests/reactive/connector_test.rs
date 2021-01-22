use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::model::{ReactiveRelationInstance, ReactiveEntityInstance};
use crate::reactive::{Connector, PROPERTY_NAME_OUTBOUND, PROPERTY_NAME_INBOUND, TYPE_NAME_CONNECTOR};
use crate::tests::utils::r_string;
use crate::tests::{create_random_entity_instance};
use serde_json::json;
use std::sync::Arc;
use std::collections::HashMap;

#[test]
fn connector_test () {
    let outbound_property_name = r_string();
    let inbound_property_name = r_string();
    let outbound_entity = Arc::new(create_random_entity_instance(outbound_property_name.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(inbound_property_name.clone()));
    let r = Arc::new(create_relation_instance_with_properties(
        outbound_entity.clone(),
        inbound_entity.clone(),
        outbound_property_name.clone(),
        inbound_property_name.clone()
    ));
    let connector = Connector::from_relation(r.clone());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(true));
    assert!(connector.relation.inbound.as_bool(inbound_property_name.clone()).unwrap());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(false));
    assert!(!connector.relation.inbound.as_bool(inbound_property_name.clone()).unwrap());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(123));
    assert_eq!(123, connector.relation.inbound.as_u64(inbound_property_name.clone()).unwrap());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(-123));
    assert_eq!(-123, connector.relation.inbound.as_i64(inbound_property_name.clone()).unwrap());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(1.23));
    assert_eq!(1.23, connector.relation.inbound.as_f64(inbound_property_name.clone()).unwrap());
    let s = r_string();
    connector.relation.outbound.set(outbound_property_name.clone(), json!(s.clone()));
    assert_eq!(s, connector.relation.inbound.as_string(inbound_property_name.clone()).unwrap());
}

#[test]
fn connector_multiple_propagation_test () {
    let e1_property_name = r_string();
    let e1 = Arc::new(create_random_entity_instance(e1_property_name.clone()));

    let e2_property_name = r_string();
    let e2 = Arc::new(create_random_entity_instance(e2_property_name.clone()));

    let e3_property_name = r_string();
    let e3 = Arc::new(create_random_entity_instance(e3_property_name.clone()));

    let r_e1_e2 = Arc::new(create_relation_instance_with_properties(
        e1.clone(),
        e2.clone(),
        e1_property_name.clone(),
        e2_property_name.clone()
    ));
    let c_e1_e2 = Connector::from_relation(r_e1_e2.clone());
    assert_ne!(0, c_e1_e2.handle_id);

    let r_e2_e3 = Arc::new(create_relation_instance_with_properties(
        e2.clone(),
        e3.clone(),
        e2_property_name.clone(),
        e3_property_name.clone()
    ));
    let c_e2_e3 = Connector::from_relation(r_e2_e3.clone());
    assert_ne!(0, c_e2_e3.handle_id);

    e1.set(e1_property_name.clone(), json!(true));
    assert!(e3.as_bool(e3_property_name.clone()).unwrap());
    e1.set(e1_property_name.clone(), json!(false));
    assert!(!e3.as_bool(e3_property_name.clone()).unwrap());
    e1.set(e1_property_name.clone(), json!(123));
    assert_eq!(123, e3.as_u64(e3_property_name.clone()).unwrap());
    e1.set(e1_property_name.clone(), json!(-123));
    assert_eq!(-123, e3.as_i64(e3_property_name.clone()).unwrap());
    e1.set(e1_property_name.clone(), json!(1.23));
    assert_eq!(1.23, e3.as_f64(e3_property_name.clone()).unwrap());
    let s = r_string();
    e1.set(e1_property_name.clone(), json!(s.clone()));
    assert_eq!(s, e3.as_string(e3_property_name.clone()).unwrap());
}

#[test]
fn connector_destruction_test () {
    let e1_property_name = r_string();
    let e1 = Arc::new(create_random_entity_instance(e1_property_name.clone()));

    let e2_property_name = r_string();
    let e2 = Arc::new(create_random_entity_instance(e2_property_name.clone()));

    let e3_property_name = r_string();
    let e3 = Arc::new(create_random_entity_instance(e3_property_name.clone()));

    let r_e1_e2 = Arc::new(create_relation_instance_with_properties(
        e1.clone(),
        e2.clone(),
        e1_property_name.clone(),
        e2_property_name.clone()
    ));
    let c_e1_e2 = Connector::from_relation(r_e1_e2.clone());
    assert_ne!(0, c_e1_e2.handle_id);

    let e3_default_string = "unmodified_e3".to_string();
    e1.set(e1_property_name.clone(), json!(e3_default_string.clone()));
    {
        let r_e2_e3 = Arc::new(create_relation_instance_with_properties(
            e2.clone(),
            e3.clone(),
            e2_property_name.clone(),
            e3_property_name.clone()
        ));
        let c_e2_e3 = Connector::from_relation(r_e2_e3.clone());
        assert_ne!(0, c_e2_e3.handle_id);

        e1.set(e1_property_name.clone(), json!(true));
        assert!(e3.as_bool(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(false));
        assert!(!e3.as_bool(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(123));
        assert_eq!(123, e3.as_u64(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(-123));
        assert_eq!(-123, e3.as_i64(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(1.23));
        assert_eq!(1.23, e3.as_f64(e3_property_name.clone()).unwrap());
        let s = r_string();
        e1.set(e1_property_name.clone(), json!(s.clone()));
        assert_eq!(s, e3.as_string(e3_property_name.clone()).unwrap());

        e1.set(e1_property_name.clone(), json!(e3_default_string.clone()));
    } // Connector c_e2_e3 should be destructed, no more propagation to e3

    assert_eq!(e3_default_string.clone(), e3.as_string(e3_property_name.clone()).unwrap());
    let s = r_string();
    e1.set(e1_property_name.clone(), json!(s.clone()));
    assert_eq!(s, e2.as_string(e2_property_name.clone()).unwrap());
    assert_eq!(e3_default_string.clone().to_string(), e3.as_string(e3_property_name.clone()).unwrap());
}

#[test]
fn connector_connect_test () {
    let e1_property_name = r_string();
    let e1 = Arc::new(create_random_entity_instance(e1_property_name.clone()));
    let e2_property_name = r_string();
    let e2 = Arc::new(create_random_entity_instance(e2_property_name.clone()));
    let e3_property_name = r_string();
    let e3 = Arc::new(create_random_entity_instance(e3_property_name.clone()));
    let c_e1_e2 = Connector::new(
        e1.clone(),
        e1_property_name.clone(),
        e2.clone(),
        e2_property_name.clone()
    );
    assert_ne!(0, c_e1_e2.handle_id);
    let e3_default_string = "unmodified_e3".to_string();
    e1.set(e1_property_name.clone(), json!(e3_default_string.clone()));
    {
        let c_e2_e3 = Connector::new(
            e2.clone(),
            e2_property_name.clone(),
            e3.clone(),
            e3_property_name.clone()
        );
        assert_ne!(0, c_e2_e3.handle_id);
        e1.set(e1_property_name.clone(), json!(true));
        assert!(e3.as_bool(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(false));
        assert!(!e3.as_bool(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(123));
        assert_eq!(123, e3.as_u64(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(-123));
        assert_eq!(-123, e3.as_i64(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(1.23));
        assert_eq!(1.23, e3.as_f64(e3_property_name.clone()).unwrap());
        let s = r_string();
        e1.set(e1_property_name.clone(), json!(s.clone()));
        assert_eq!(s, e3.as_string(e3_property_name.clone()).unwrap());
        e1.set(e1_property_name.clone(), json!(e3_default_string.clone()));
    } // Connector c_e2_e3 should be destructed, no more propagation to e3

    assert_eq!(e3_default_string.clone(), e3.as_string(e3_property_name.clone()).unwrap());
    let s = r_string();
    e1.set(e1_property_name.clone(), json!(s.clone()));
    assert_eq!(s, e2.as_string(e2_property_name.clone()).unwrap());
    assert_eq!(e3_default_string.clone().to_string(), e3.as_string(e3_property_name.clone()).unwrap());

}

pub(crate) fn create_relation_instance_with_properties (
    outbound_entity: Arc<ReactiveEntityInstance<'static>>,
    inbound_entity: Arc<ReactiveEntityInstance<'static>>,
    outbound_property_name: String,
    inbound_property_name: String
) -> ReactiveRelationInstance<'static> {
    let mut properties = HashMap::new();
    properties.insert(PROPERTY_NAME_OUTBOUND.to_string(),  json!(outbound_property_name));
    properties.insert(PROPERTY_NAME_INBOUND.to_string(),  json!(inbound_property_name));
    ReactiveRelationInstance::create_with_properties(
        outbound_entity.clone(),
        TYPE_NAME_CONNECTOR.to_string(),
        inbound_entity.clone(),
        properties
    )
}

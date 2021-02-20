use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::tests::utils::r_string;
use crate::tests::{create_random_entity_instance, create_default_connector};
use serde_json::json;
use std::sync::Arc;
use crate::behaviour::{DefaultConnector, RelationBehaviour};

#[test]
fn default_connector_test () {
    let outbound_property_name = r_string();
    let inbound_property_name = r_string();
    let outbound_entity = Arc::new(create_random_entity_instance(outbound_property_name.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(inbound_property_name.clone()));
    let r = Arc::new(create_default_connector(
        outbound_entity.clone(),
        inbound_entity.clone(),
        outbound_property_name.clone(),
        inbound_property_name.clone()
    ));
    // let connector = DefaultConnector::new(outbound_entity, inbound_entity);
    let connector = DefaultConnector::from_relation_instance(r.clone());
    assert!(connector.is_ok());
    let connector = connector.unwrap();
    // let connector = Connector::from_relation(r.clone(), DefaultConnector::OPERATION);
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

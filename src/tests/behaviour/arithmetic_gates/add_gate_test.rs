use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{AddGate, ReactiveEntityInstanceBehaviour};
use crate::reactive::{Connector};
use crate::reactive::arithmetic_gate::{PROPERTY_NAME_NUMBER_1, PROPERTY_NAME_NUMBER_2, PROPERTY_NAME_RESULT_1};
use crate::tests::create_relation_instance_with_properties;
use std::sync::Arc;
use serde_json::json;

#[test]
fn add_gates_test () {
    // Now it's very convenient to create AND-Gates
    let add_gate_1 = AddGate::new().unwrap();
    let add_gate_2 = AddGate::new().unwrap();
    let add_gate_3 = AddGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let add_1 = add_gate_1.entity.clone();
    let add_2 = add_gate_2.entity.clone();
    let add_3 = add_gate_3.entity.clone();
    // In real world, the and gate have to be registered in the registry (!)

    // Reset all states
    add_1.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(0));
    add_1.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(0));
    add_1.set(PROPERTY_NAME_RESULT_1.to_string(), json!(0));

    add_2.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(0));
    add_2.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(0));
    add_2.set(PROPERTY_NAME_RESULT_1.to_string(), json!(0));

    add_3.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(0));
    add_3.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(0));
    add_3.set(PROPERTY_NAME_RESULT_1.to_string(), json!(0));

    // Connect the results of the first two AND-Gates with the inputs of the third AND-Gate
    let r_add_1_add_3 = Arc::new(create_relation_instance_with_properties(
        add_1.clone(),
        add_3.clone(),
        PROPERTY_NAME_RESULT_1.to_string(),
        PROPERTY_NAME_NUMBER_1.to_string()
    ));
    let c_add_1_add_3 = Connector::from_relation(r_add_1_add_3.clone());
    assert_ne!(0, c_add_1_add_3.handle_id);

    let r_add_2_add_3 = Arc::new(create_relation_instance_with_properties(
        add_2.clone(),
        add_3.clone(),
        PROPERTY_NAME_RESULT_1.to_string(),
        PROPERTY_NAME_NUMBER_2.to_string()
    ));
    let c_add_2_add_3 = Connector::from_relation(r_add_2_add_3.clone());
    assert_ne!(0, c_add_2_add_3.handle_id);

    // Starting point
    assert_eq!(0, add_1.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(0, add_2.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(0, add_3.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    add_1.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(1));
    assert_eq!(1, add_1.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(0, add_2.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(1, add_3.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    add_1.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(1));
    assert_eq!(2, add_1.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(0, add_2.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(2, add_3.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    add_2.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(1));
    assert_eq!(2, add_1.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(1, add_2.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(3, add_3.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    add_2.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(1));
    assert_eq!(2, add_1.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(2, add_2.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(4, add_3.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
}

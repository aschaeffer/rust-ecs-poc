use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{ReactiveEntityInstanceBehaviour, OrGate};
use crate::reactive::{PROPERTY_NAME_RESULT_1, PROPERTY_NAME_BIT_1, Connector, PROPERTY_NAME_BIT_2};
use crate::tests::create_relation_instance_with_properties;
use std::sync::Arc;
use serde_json::json;

#[test]
fn or_gates_test () {
    // Now it's very convenient to create AND-Gates
    let or_gate_1 = OrGate::new().unwrap();
    let or_gate_2 = OrGate::new().unwrap();
    let or_gate_3 = OrGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let or_1 = or_gate_1.entity.clone();
    let or_2 = or_gate_2.entity.clone();
    let or_3 = or_gate_3.entity.clone();
    // In real world, the or gate have to be registered in the registry (!)

    // Reset all states
    or_1.set(PROPERTY_NAME_BIT_1.to_string(), json!(true));
    or_1.set(PROPERTY_NAME_BIT_2.to_string(), json!(true));
    or_1.set(PROPERTY_NAME_RESULT_1.to_string(), json!(true));

    or_2.set(PROPERTY_NAME_BIT_1.to_string(), json!(true));
    or_2.set(PROPERTY_NAME_BIT_2.to_string(), json!(true));
    or_2.set(PROPERTY_NAME_RESULT_1.to_string(), json!(true));

    or_3.set(PROPERTY_NAME_BIT_1.to_string(), json!(true));
    or_3.set(PROPERTY_NAME_BIT_2.to_string(), json!(true));
    or_3.set(PROPERTY_NAME_RESULT_1.to_string(), json!(true));


    // Connect the results of the first two AND-Gates with the inputs of the third AND-Gate
    let r_or_1_or_3 = Arc::new(create_relation_instance_with_properties(
        or_1.clone(),
        or_3.clone(),
        PROPERTY_NAME_RESULT_1.to_string(),
        PROPERTY_NAME_BIT_1.to_string()
    ));
    let c_or_1_or_3 = Connector::from_relation(r_or_1_or_3.clone());
    assert_ne!(0, c_or_1_or_3.handle_id);

    let r_or_2_or_3 = Arc::new(create_relation_instance_with_properties(
        or_2.clone(),
        or_3.clone(),
        PROPERTY_NAME_RESULT_1.to_string(),
        PROPERTY_NAME_BIT_2.to_string()
    ));
    let c_or_2_or_3 = Connector::from_relation(r_or_2_or_3.clone());
    assert_ne!(0, c_or_2_or_3.handle_id);

    // Initial
    assert_eq!(true, or_1.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(true, or_2.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(true, or_3.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    // Bit 1 false
    or_1.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
    assert_eq!(true, or_1.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(true, or_2.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(true, or_3.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    // Bit 2 false
    or_1.set(PROPERTY_NAME_BIT_2.to_string(), json!(false));
    assert_eq!(false, or_1.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(true, or_2.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(true, or_3.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    // Bit 3 false
    or_2.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
    assert_eq!(false, or_1.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(true, or_2.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(true, or_3.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    // Bit 4 false
    or_2.set(PROPERTY_NAME_BIT_2.to_string(), json!(false));
    assert_eq!(false, or_1.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(false, or_2.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(false, or_3.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
}

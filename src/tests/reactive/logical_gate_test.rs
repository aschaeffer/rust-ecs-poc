use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{AndGate, OrGate};
use crate::reactive::{PROPERTY_NAME_BIT_1, PROPERTY_NAME_BIT_2, PROPERTY_NAME_RESULT_1, LogicalGate, Connector, create_logical_gate_entity};
use crate::tests::create_relation_instance_with_properties;
use serde_json::json;
use std::sync::Arc;

#[test]
fn and_gate_type_test () {
    let and = Arc::new(create_logical_gate_entity(AndGate::TYPE_NAME.to_string()));
    let and_gate = LogicalGate::new(and.clone(), AndGate::OPERATION);
    assert_eq!(AndGate::TYPE_NAME.to_string(), and_gate.type_name());
}

#[test]
fn and_gate_test () {
    let and = Arc::new(create_logical_gate_entity(AndGate::TYPE_NAME.to_string()));
    {
        // Create the AND-Gate in scope
        let and_gate = LogicalGate::new(and.clone(), AndGate::OPERATION);
        assert_ne!(0, and_gate.handle_id);

        and.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
        and.set(PROPERTY_NAME_BIT_2.to_string(), json!(false));
        assert_eq!(false, and.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
        and.set(PROPERTY_NAME_BIT_2.to_string(), json!(true));
        assert_eq!(false, and.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
        and.set(PROPERTY_NAME_BIT_2.to_string(), json!(false));
        and.set(PROPERTY_NAME_BIT_1.to_string(), json!(true));
        assert_eq!(false, and.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
        and.set(PROPERTY_NAME_BIT_2.to_string(), json!(true));
        assert_eq!(true, and.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    } // The LogicalGate doesn't live no more
    // Setting the inputs ...
    and.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
    and.set(PROPERTY_NAME_BIT_2.to_string(), json!(false));
    // ... doesn't affect the result anymore (result should have the same value as before)
    assert_eq!(true, and.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
}

#[test]
fn or_gate_test () {
    let or = Arc::new(create_logical_gate_entity("or".to_string()));
    {
        let and_gate = LogicalGate::new(or.clone(), OrGate::OPERATION);
        assert_ne!(0, and_gate.handle_id);

        or.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
        or.set(PROPERTY_NAME_BIT_2.to_string(), json!(false));
        assert_eq!(false, or.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

        or.set(PROPERTY_NAME_BIT_1.to_string(), json!(true));
        assert_eq!(true, or.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
        or.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
        assert_eq!(false, or.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

        or.set(PROPERTY_NAME_BIT_2.to_string(), json!(true));
        assert_eq!(true, or.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
        or.set(PROPERTY_NAME_BIT_2.to_string(), json!(false));
        assert_eq!(false, or.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

        or.set(PROPERTY_NAME_BIT_1.to_string(), json!(true));
        or.set(PROPERTY_NAME_BIT_2.to_string(), json!(true));
        assert_eq!(true, or.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    } // The LogicalGate doesn't live no more
    // Setting the inputs ...
    or.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
    or.set(PROPERTY_NAME_BIT_2.to_string(), json!(false));
    // ... doesn't affect the result anymore (result should have the same value as before)
    assert_eq!(true, or.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
}

/// The results of two AND-Gates are the inputs of the third AND-Gate
#[test]
fn three_and_gates_test () {
    let and_1 = Arc::new(create_logical_gate_entity(AndGate::TYPE_NAME.to_string()));
    let and_2 = Arc::new(create_logical_gate_entity(AndGate::TYPE_NAME.to_string()));
    let and_3 = Arc::new(create_logical_gate_entity(AndGate::TYPE_NAME.to_string()));

    // Reset all states
    and_1.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
    and_1.set(PROPERTY_NAME_BIT_2.to_string(), json!(false));
    and_1.set(PROPERTY_NAME_RESULT_1.to_string(), json!(false));

    and_2.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
    and_2.set(PROPERTY_NAME_BIT_2.to_string(), json!(false));
    and_2.set(PROPERTY_NAME_RESULT_1.to_string(), json!(false));

    and_3.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
    and_3.set(PROPERTY_NAME_BIT_2.to_string(), json!(false));
    and_3.set(PROPERTY_NAME_RESULT_1.to_string(), json!(false));

    // Make the entity instances act as AND-Gates
    let and_gate_1 = LogicalGate::new(and_1.clone(), AndGate::OPERATION);
    assert_ne!(0, and_gate_1.handle_id);

    let and_gate_2 = LogicalGate::new(and_2.clone(), AndGate::OPERATION);
    assert_ne!(0, and_gate_2.handle_id);

    let and_gate_3 = LogicalGate::new(and_3.clone(), AndGate::OPERATION);
    assert_ne!(0, and_gate_3.handle_id);

    // Connect the results of the first two AND-Gates with the inputs of the third AND-Gate
    let r_and_1_and_3 = Arc::new(create_relation_instance_with_properties(
        and_1.clone(),
        and_3.clone(),
        PROPERTY_NAME_RESULT_1.to_string(),
        PROPERTY_NAME_BIT_1.to_string()
    ));
    let c_and_1_and_3 = Connector::from_relation(r_and_1_and_3.clone());
    assert_ne!(0, c_and_1_and_3.handle_id);

    let r_and_2_and_3 = Arc::new(create_relation_instance_with_properties(
        and_2.clone(),
        and_3.clone(),
        PROPERTY_NAME_RESULT_1.to_string(),
        PROPERTY_NAME_BIT_2.to_string()
    ));
    let c_and_2_and_3 = Connector::from_relation(r_and_2_and_3.clone());
    assert_ne!(0, c_and_2_and_3.handle_id);

    and_1.set(PROPERTY_NAME_BIT_1.to_string(), json!(true));
    assert_eq!(false, and_1.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(false, and_3.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    and_1.set(PROPERTY_NAME_BIT_2.to_string(), json!(true));
    assert_eq!(true, and_1.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(false, and_3.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    and_2.set(PROPERTY_NAME_BIT_1.to_string(), json!(true));
    assert_eq!(false, and_2.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(false, and_3.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    and_2.set(PROPERTY_NAME_BIT_2.to_string(), json!(true));
    assert_eq!(true, and_2.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    assert_eq!(true, and_3.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
}

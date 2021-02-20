use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{AddGate};
use crate::reactive::{Connector, create_numeric_gate_entity};
use crate::tests::create_relation_instance_with_properties;
use serde_json::json;
use std::sync::Arc;
use crate::reactive::numeric_gate::{create_trigonometric_gate_entity, TrigonometricGate, PROPERTY_NAME_NUMBER_1, PROPERTY_NAME_NUMBER_2, PROPERTY_NAME_RESULT_1};

#[test]
fn numeric_gate_type_test () {
    // create_numeric_gate_entity(Sin)
    let and = Arc::new(create_trigonometric_gate_entity(AddGate::TYPE_NAME.to_string()));
    let and_gate = TrigonometricGate::new(and.clone(), AddGate::OPERATION);
    assert_eq!(AddGate::TYPE_NAME.to_string(), and_gate.type_name());
}

#[test]
fn and_gate_test () {
    let and = Arc::new(create_trigonometric_gate_entity(AddGate::TYPE_NAME.to_string()));
    {
        // Create the ADD-Gate in scope
        let and_gate = TrigonometricGate::new(and.clone(), AddGate::OPERATION);
        assert_ne!(0, and_gate.handle_id);

        and.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(1));
        and.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(1));
        assert_eq!(2, and.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
        and.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(2));
        assert_eq!(3, and.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
        and.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(2));
        assert_eq!(4, and.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    } // The TrigonometricGate no more alive ...
    // ... so, setting the inputs ...
    and.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(0));
    and.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(0));
    // ... doesn't affect the result anymore (result should have the same value as before)
    assert_eq!(4, and.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
}

/// The results of two ADD-Gates are the inputs of the third ADD-Gate
#[test]
fn three_add_gates_test () {
    let add_1 = Arc::new(create_trigonometric_gate_entity(AddGate::TYPE_NAME.to_string()));
    let add_2 = Arc::new(create_trigonometric_gate_entity(AddGate::TYPE_NAME.to_string()));
    let add_3 = Arc::new(create_trigonometric_gate_entity(AddGate::TYPE_NAME.to_string()));

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

    // Make the entity instances act as AND-Gates
    let add_gate_1 = TrigonometricGate::new(add_1.clone(), AddGate::OPERATION);
    assert_ne!(0, add_gate_1.handle_id);

    let add_gate_2 = TrigonometricGate::new(add_2.clone(), AddGate::OPERATION);
    assert_ne!(0, add_gate_2.handle_id);

    let add_gate_3 = TrigonometricGate::new(add_3.clone(), AddGate::OPERATION);
    assert_ne!(0, add_gate_3.handle_id);

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

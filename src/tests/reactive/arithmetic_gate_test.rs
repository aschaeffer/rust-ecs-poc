use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{AddGate, ReactiveEntityInstanceBehaviour, ArithmeticGateBehaviour};
use crate::reactive::{Connector};
use crate::tests::create_relation_instance_with_properties;
use serde_json::json;
use std::sync::Arc;
use crate::reactive::arithmetic_gate::{create_arithmetic_gate_entity, ArithmeticGate};

const NUMBER_1: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_1;
const NUMBER_2: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_2;
const RESULT_1: &str = ArithmeticGate::PROPERTY_NAME_RESULT_1;

#[test]
fn and_gate_type_test () {
    let and = Arc::new(create_arithmetic_gate_entity::<i64>(AddGate::TYPE_NAME.to_string()));
    let and_gate = ArithmeticGate::new(and.clone(), AddGate::OPERATION);
    assert_eq!(AddGate::TYPE_NAME.to_string(), and_gate.type_name());
}

#[test]
fn and_gate_test () {
    let and = Arc::new(create_arithmetic_gate_entity::<i64>(AddGate::TYPE_NAME.to_string()));
    {
        // Create the ADD-Gate in scope
        let and_gate = ArithmeticGate::new(and.clone(), AddGate::OPERATION);
        assert_ne!(0, and_gate.handle_id);

        and.set(NUMBER_1.to_string(), json!(1));
        and.set(NUMBER_2.to_string(), json!(1));
        assert_eq!(2, and.as_i64(RESULT_1.to_string()).unwrap());
        and.set(NUMBER_2.to_string(), json!(2));
        assert_eq!(3, and.as_i64(RESULT_1.to_string()).unwrap());
        and.set(NUMBER_1.to_string(), json!(2));
        assert_eq!(4, and.as_i64(RESULT_1.to_string()).unwrap());
    } // The ArithmeticGate no more alive ...
    // ... so, setting the inputs ...
    and.set(NUMBER_1.to_string(), json!(0));
    and.set(NUMBER_2.to_string(), json!(0));
    // ... doesn't affect the result anymore (result should have the same value as before)
    assert_eq!(4, and.as_i64(RESULT_1.to_string()).unwrap());
}

/// The results of two ADD-Gates are the inputs of the third ADD-Gate
#[test]
fn three_add_gates_test () {
    let add_1 = Arc::new(create_arithmetic_gate_entity::<i64>(AddGate::TYPE_NAME.to_string()));
    let add_2 = Arc::new(create_arithmetic_gate_entity::<i64>(AddGate::TYPE_NAME.to_string()));
    let add_3 = Arc::new(create_arithmetic_gate_entity::<i64>(AddGate::TYPE_NAME.to_string()));

    // Reset all states
    add_1.set(NUMBER_1.to_string(), json!(0));
    add_1.set(NUMBER_2.to_string(), json!(0));
    add_1.set(RESULT_1.to_string(), json!(0));

    add_2.set(NUMBER_1.to_string(), json!(0));
    add_2.set(NUMBER_2.to_string(), json!(0));
    add_2.set(RESULT_1.to_string(), json!(0));

    add_3.set(NUMBER_1.to_string(), json!(0));
    add_3.set(NUMBER_2.to_string(), json!(0));
    add_3.set(RESULT_1.to_string(), json!(0));

    // Make the entity instances act as AND-Gates
    let add_gate_1 = ArithmeticGate::new(add_1.clone(), AddGate::OPERATION);
    assert_ne!(0, add_gate_1.handle_id);

    let add_gate_2 = ArithmeticGate::new(add_2.clone(), AddGate::OPERATION);
    assert_ne!(0, add_gate_2.handle_id);

    let add_gate_3 = ArithmeticGate::new(add_3.clone(), AddGate::OPERATION);
    assert_ne!(0, add_gate_3.handle_id);

    // Connect the results of the first two AND-Gates with the inputs of the third AND-Gate
    let r_add_1_add_3 = Arc::new(create_relation_instance_with_properties(
        add_1.clone(),
        add_3.clone(),
        RESULT_1.to_string(),
        NUMBER_1.to_string()
    ));
    let c_add_1_add_3 = Connector::from_relation(r_add_1_add_3.clone());
    assert_ne!(0, c_add_1_add_3.handle_id);

    let r_add_2_add_3 = Arc::new(create_relation_instance_with_properties(
        add_2.clone(),
        add_3.clone(),
        RESULT_1.to_string(),
        NUMBER_2.to_string()
    ));
    let c_add_2_add_3 = Connector::from_relation(r_add_2_add_3.clone());
    assert_ne!(0, c_add_2_add_3.handle_id);

    // Starting point
    assert_eq!(0, add_1.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(0, add_2.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(0, add_3.as_i64(RESULT_1.to_string()).unwrap());

    add_1.set(NUMBER_1.to_string(), json!(1));
    assert_eq!(1, add_1.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(0, add_2.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(1, add_3.as_i64(RESULT_1.to_string()).unwrap());

    add_1.set(NUMBER_2.to_string(), json!(1));
    assert_eq!(2, add_1.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(0, add_2.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(2, add_3.as_i64(RESULT_1.to_string()).unwrap());

    add_2.set(NUMBER_1.to_string(), json!(1));
    assert_eq!(2, add_1.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(1, add_2.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(3, add_3.as_i64(RESULT_1.to_string()).unwrap());

    add_2.set(NUMBER_2.to_string(), json!(1));
    assert_eq!(2, add_1.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(2, add_2.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(4, add_3.as_i64(RESULT_1.to_string()).unwrap());
}

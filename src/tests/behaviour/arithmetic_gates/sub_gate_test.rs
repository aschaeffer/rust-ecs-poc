use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{SubGate, ReactiveEntityInstanceBehaviour};
use crate::reactive::{Connector};
use crate::reactive::arithmetic_gate::ArithmeticGate;
use crate::tests::create_relation_instance_with_properties;
use std::sync::Arc;
use serde_json::json;

const NUMBER_1: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_1;
const NUMBER_2: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_2;
const RESULT_1: &str = ArithmeticGate::PROPERTY_NAME_RESULT_1;

#[test]
fn sub_gates_test () {
    let sub_gate_1 = SubGate::new().unwrap();
    let sub_gate_2 = SubGate::new().unwrap();
    let sub_gate_3 = SubGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let sub_1 = sub_gate_1.entity.clone();
    let sub_2 = sub_gate_2.entity.clone();
    let sub_3 = sub_gate_3.entity.clone();
    // In real world, the and gate have to be registered in the registry (!)

    // Reset all states
    sub_1.set(NUMBER_1.to_string(), json!(0));
    sub_1.set(NUMBER_2.to_string(), json!(0));
    sub_1.set(RESULT_1.to_string(), json!(0));

    sub_2.set(NUMBER_1.to_string(), json!(0));
    sub_2.set(NUMBER_2.to_string(), json!(0));
    sub_2.set(RESULT_1.to_string(), json!(0));

    sub_3.set(NUMBER_1.to_string(), json!(0));
    sub_3.set(NUMBER_2.to_string(), json!(0));
    sub_3.set(RESULT_1.to_string(), json!(0));

    // Connect the results of the first two AND-Gates with the inputs of the third AND-Gate
    let r_sub_1_sub_3 = Arc::new(create_relation_instance_with_properties(
        sub_1.clone(),
        sub_3.clone(),
        RESULT_1.to_string(),
        NUMBER_1.to_string()
    ));
    let c_sub_1_sub_3 = Connector::from_relation(r_sub_1_sub_3.clone());
    assert_ne!(0, c_sub_1_sub_3.handle_id);

    let r_sub_2_sub_3 = Arc::new(create_relation_instance_with_properties(
        sub_2.clone(),
        sub_3.clone(),
        RESULT_1.to_string(),
        NUMBER_2.to_string()
    ));
    let c_sub_2_sub_3 = Connector::from_relation(r_sub_2_sub_3.clone());
    assert_ne!(0, c_sub_2_sub_3.handle_id);

    // Starting point
    assert_eq!(0, sub_1.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(0, sub_2.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(0, sub_3.as_i64(RESULT_1.to_string()).unwrap());

    sub_1.set(NUMBER_1.to_string(), json!(10));
    assert_eq!(10, sub_1.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(0, sub_2.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(10, sub_3.as_i64(RESULT_1.to_string()).unwrap());

    sub_1.set(NUMBER_2.to_string(), json!(5));
    assert_eq!(5, sub_1.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(0, sub_2.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(5, sub_3.as_i64(RESULT_1.to_string()).unwrap());

    sub_2.set(NUMBER_1.to_string(), json!(3));
    assert_eq!(5, sub_1.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(3, sub_2.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(2, sub_3.as_i64(RESULT_1.to_string()).unwrap());

    sub_2.set(NUMBER_2.to_string(), json!(22));
    assert_eq!(5, sub_1.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(-19, sub_2.as_i64(RESULT_1.to_string()).unwrap());
    assert_eq!(24, sub_3.as_i64(RESULT_1.to_string()).unwrap());
}

use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{MulGate, ReactiveEntityInstanceBehaviour};
use crate::reactive::arithmetic_gate::{PROPERTY_NAME_NUMBER_1, PROPERTY_NAME_NUMBER_2, PROPERTY_NAME_RESULT_1};
use serde_json::json;

#[test]
fn mul_gate_test () {
    // Now it's very convenient to create AND-Gates
    let mul_gate = MulGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let mul = mul_gate.entity.clone();

    mul.set(PROPERTY_NAME_RESULT_1.to_string(), json!(0));
    mul.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(0));
    mul.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(0));
    assert_eq!(0, mul.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    mul.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(5));
    mul.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(5));
    assert_eq!(25, mul.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
}

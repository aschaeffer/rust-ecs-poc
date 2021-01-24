use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{MulGate, ReactiveEntityInstanceBehaviour};
use crate::reactive::arithmetic_gate::ArithmeticGate;
use serde_json::json;

const NUMBER_1: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_1;
const NUMBER_2: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_2;
const RESULT_1: &str = ArithmeticGate::PROPERTY_NAME_RESULT_1;

#[test]
fn mul_gate_test () {
    let mul_gate = MulGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let mul = mul_gate.entity.clone();

    mul.set(RESULT_1.to_string(), json!(0));
    mul.set(NUMBER_1.to_string(), json!(0));
    mul.set(NUMBER_2.to_string(), json!(0));
    assert_eq!(0, mul.as_i64(RESULT_1.to_string()).unwrap());
    mul.set(NUMBER_1.to_string(), json!(5));
    mul.set(NUMBER_2.to_string(), json!(5));
    assert_eq!(25, mul.as_i64(RESULT_1.to_string()).unwrap());
}

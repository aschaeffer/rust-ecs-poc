use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{MinGate, ReactiveEntityInstanceBehaviour};
use crate::reactive::arithmetic_gate::ArithmeticGate;
use serde_json::json;

const NUMBER_1: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_1;
const NUMBER_2: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_2;
const RESULT_1: &str = ArithmeticGate::PROPERTY_NAME_RESULT_1;

#[test]
fn min_gate_test () {
    let min_gate = MinGate::new().unwrap();
    let min = min_gate.entity.clone();
    min.set(RESULT_1.to_string(), json!(0));
    min.set(NUMBER_1.to_string(), json!(0));
    min.set(NUMBER_2.to_string(), json!(0));
    assert_eq!(0, min.as_i64(RESULT_1.to_string()).unwrap());
    min.set(NUMBER_1.to_string(), json!(5));
    assert_eq!(0, min.as_i64(RESULT_1.to_string()).unwrap());
    min.set(NUMBER_2.to_string(), json!(15));
    assert_eq!(5, min.as_i64(RESULT_1.to_string()).unwrap());
    min.set(NUMBER_1.to_string(), json!(25));
    assert_eq!(15, min.as_i64(RESULT_1.to_string()).unwrap());
}

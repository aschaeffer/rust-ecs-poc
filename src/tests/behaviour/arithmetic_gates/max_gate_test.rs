use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{MaxGate, ReactiveEntityInstanceBehaviour};
use crate::reactive::arithmetic_gate::ArithmeticGate;
use serde_json::json;

const NUMBER_1: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_1;
const NUMBER_2: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_2;
const RESULT_1: &str = ArithmeticGate::PROPERTY_NAME_RESULT_1;

#[test]
fn max_gate_test () {
    let max_gate = MaxGate::new().unwrap();
    let max = max_gate.entity.clone();
    max.set(RESULT_1.to_string(), json!(0));
    max.set(NUMBER_1.to_string(), json!(0));
    max.set(NUMBER_2.to_string(), json!(0));
    assert_eq!(0, max.as_i64(RESULT_1.to_string()).unwrap());
    max.set(NUMBER_1.to_string(), json!(5));
    assert_eq!(5, max.as_i64(RESULT_1.to_string()).unwrap());
    max.set(NUMBER_2.to_string(), json!(15));
    assert_eq!(15, max.as_i64(RESULT_1.to_string()).unwrap());
    max.set(NUMBER_1.to_string(), json!(10));
    assert_eq!(15, max.as_i64(RESULT_1.to_string()).unwrap());
}

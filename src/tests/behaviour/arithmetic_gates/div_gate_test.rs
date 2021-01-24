use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{DivGate, ReactiveEntityInstanceBehaviour};
use crate::reactive::arithmetic_gate::ArithmeticGate;
use serde_json::json;

const NUMBER_1: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_1;
const NUMBER_2: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_2;
const RESULT_1: &str = ArithmeticGate::PROPERTY_NAME_RESULT_1;

#[test]
fn div_gate_test () {
    let div_gate = DivGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let div = div_gate.entity.clone();

    div.set(NUMBER_1.to_string(), json!(10));
    div.set(NUMBER_2.to_string(), json!(5));
    assert_eq!(2, div.as_i64(RESULT_1.to_string()).unwrap());
}

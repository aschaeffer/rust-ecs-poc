use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{ModGate, ReactiveEntityInstanceBehaviour};
use crate::reactive::arithmetic_gate::ArithmeticGate;
use serde_json::json;

const NUMBER_1: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_1;
const NUMBER_2: &str = ArithmeticGate::PROPERTY_NAME_NUMBER_2;
const RESULT_1: &str = ArithmeticGate::PROPERTY_NAME_RESULT_1;

#[test]
fn modulo_gate_test() {
    let modulo_gate = ModGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let modulo = modulo_gate.entity.clone();

    modulo.set(NUMBER_1.to_string(), json!(12));
    modulo.set(NUMBER_2.to_string(), json!(5));
    assert_eq!(2, modulo.as_i64(RESULT_1.to_string()).unwrap());

    modulo.set(NUMBER_1.to_string(), json!(2));
    modulo.set(NUMBER_2.to_string(), json!(2));
    assert_eq!(0, modulo.as_i64(RESULT_1.to_string()).unwrap());
}

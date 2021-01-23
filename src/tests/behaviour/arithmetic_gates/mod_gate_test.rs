use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{ModGate, ReactiveEntityInstanceBehaviour};
use crate::reactive::arithmetic_gate::{PROPERTY_NAME_NUMBER_1, PROPERTY_NAME_NUMBER_2, PROPERTY_NAME_RESULT_1};
use serde_json::json;

#[test]
fn modulo_gate_test() {
    // Now it's very convenient to create AND-Gates
    let modulo_gate = ModGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let modulo = modulo_gate.entity.clone();

    modulo.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(12));
    modulo.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(5));
    assert_eq!(2, modulo.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    modulo.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(2));
    modulo.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(2));
    assert_eq!(0, modulo.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
}

use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{DivGate, ReactiveEntityInstanceBehaviour};
use crate::reactive::arithmetic_gate::{PROPERTY_NAME_NUMBER_1, PROPERTY_NAME_NUMBER_2, PROPERTY_NAME_RESULT_1};
use serde_json::json;

#[test]
fn div_gate_test () {
    // Now it's very convenient to create AND-Gates
    let div_gate = DivGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let div = div_gate.entity.clone();

    div.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(10));
    div.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(5));
    assert_eq!(2, div.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
}

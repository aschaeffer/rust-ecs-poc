use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{MaxGate, ReactiveEntityInstanceBehaviour};
use crate::reactive::arithmetic_gate::{PROPERTY_NAME_NUMBER_1, PROPERTY_NAME_NUMBER_2, PROPERTY_NAME_RESULT_1};
use serde_json::json;

#[test]
fn max_gate_test () {
    let max_gate = MaxGate::new().unwrap();
    let max = max_gate.entity.clone();
    max.set(PROPERTY_NAME_RESULT_1.to_string(), json!(0));
    max.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(0));
    max.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(0));
    assert_eq!(0, max.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    max.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(5));
    assert_eq!(5, max.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    max.set(PROPERTY_NAME_NUMBER_2.to_string(), json!(15));
    assert_eq!(15, max.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
    max.set(PROPERTY_NAME_NUMBER_1.to_string(), json!(10));
    assert_eq!(15, max.as_i64(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
}

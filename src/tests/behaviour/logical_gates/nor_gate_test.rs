use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{ReactiveEntityInstanceBehaviour, NorGate};
use crate::reactive::logical_gate::{PROPERTY_NAME_BIT_1,PROPERTY_NAME_BIT_2,PROPERTY_NAME_RESULT_1};
use serde_json::json;

#[test]
fn nor_gates_test () {
    // Now it's very convenient to create AND-Gates
    let nor_gate = NorGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let nor = nor_gate.entity.clone();

    // Reset all inputs
    nor.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
    nor.set(PROPERTY_NAME_BIT_2.to_string(), json!(false));

    // Initial
    assert_eq!(true, nor.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    nor.set(PROPERTY_NAME_BIT_1.to_string(), json!(true));
    assert_eq!(false, nor.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    nor.set(PROPERTY_NAME_BIT_2.to_string(), json!(true));
    assert_eq!(false, nor.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    nor.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
    assert_eq!(false, nor.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
}

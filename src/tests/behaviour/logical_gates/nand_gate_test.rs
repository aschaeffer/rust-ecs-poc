use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{NandGate, ReactiveEntityInstanceBehaviour};
use crate::reactive::logical_gate::{PROPERTY_NAME_BIT_1,PROPERTY_NAME_BIT_2,PROPERTY_NAME_RESULT_1};
use serde_json::json;

#[test]
fn nand_gates_test () {
    // Now it's very convenient to create NAND-Gates
    let nand_gate = NandGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let nand = nand_gate.entity.clone();
    // In real world, the nand gate have to be registered in the registry (!)

    // Reset state
    nand.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
    nand.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));

    assert_eq!(true, nand.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    nand.set(PROPERTY_NAME_BIT_1.to_string(), json!(true));
    assert_eq!(true, nand.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    nand.set(PROPERTY_NAME_BIT_2.to_string(), json!(true));
    assert_eq!(false, nand.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());

    nand.set(PROPERTY_NAME_BIT_1.to_string(), json!(false));
    assert_eq!(true, nand.as_bool(PROPERTY_NAME_RESULT_1.to_string()).unwrap());
}

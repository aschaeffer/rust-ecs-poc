use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{NandGate, EntityBehaviour};
use crate::reactive::LogicalGateProperties;
use serde_json::json;

#[test]
fn nand_gates_test () {
    // Now it's very convenient to create NAND-Gates
    let nand_gate = NandGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let nand = nand_gate.entity.clone();
    // In real world, the nand gate have to be registered in the registry (!)

    // Reset state
    nand.set(LogicalGateProperties::LHS.to_string(), json!(false));
    nand.set(LogicalGateProperties::LHS.to_string(), json!(false));

    assert_eq!(true, nand.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    nand.set(LogicalGateProperties::LHS.to_string(), json!(true));
    assert_eq!(true, nand.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    nand.set(LogicalGateProperties::RHS.to_string(), json!(true));
    assert_eq!(false, nand.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    nand.set(LogicalGateProperties::LHS.to_string(), json!(false));
    assert_eq!(true, nand.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
}

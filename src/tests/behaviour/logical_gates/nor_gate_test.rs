use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{EntityBehaviour, NorGate};
use crate::reactive::LogicalGateProperties;
use serde_json::json;

#[test]
fn nor_gates_test () {
    // Now it's very convenient to create AND-Gates
    let nor_gate = NorGate::new().unwrap();

    // You can get the inner ReactiveEntityInstance from a LogicalGate
    let nor = nor_gate.entity.clone();

    // Reset all inputs
    nor.set(LogicalGateProperties::LHS.to_string(), json!(false));
    nor.set(LogicalGateProperties::RHS.to_string(), json!(false));

    // Initial
    assert_eq!(true, nor.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    nor.set(LogicalGateProperties::LHS.to_string(), json!(true));
    assert_eq!(false, nor.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    nor.set(LogicalGateProperties::RHS.to_string(), json!(true));
    assert_eq!(false, nor.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());

    nor.set(LogicalGateProperties::LHS.to_string(), json!(false));
    assert_eq!(false, nor.as_bool(LogicalGateProperties::RESULT.to_string()).unwrap());
}
